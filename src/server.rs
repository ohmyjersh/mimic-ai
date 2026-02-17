use crate::compose::{self, ComposeRequest};
use crate::fragment::Category;
use crate::registry::Registry;
use crate::resolve;
use crate::version::VersionChecker;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    AnnotateAble, CompleteRequestParams, CompleteResult, CompletionInfo, GetPromptRequestParams,
    GetPromptResult, Implementation, ListPromptsResult, ListResourcesResult,
    PaginatedRequestParams, Prompt, PromptMessage, PromptMessageRole, RawResource,
    ReadResourceRequestParams, ReadResourceResult, ResourceContents, ServerCapabilities,
    ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, RoleServer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct MimicServer {
    registry: Arc<RwLock<Registry>>,
    tool_router: ToolRouter<Self>,
    version_checker: Arc<VersionChecker>,
    // Keep the watcher alive for the server's lifetime
    _watcher: Option<notify::RecommendedWatcher>,
}

impl MimicServer {
    pub fn new(
        registry: Arc<RwLock<Registry>>,
        watcher: Option<notify::RecommendedWatcher>,
    ) -> Self {
        Self {
            registry,
            tool_router: Self::tool_router(),
            version_checker: Arc::new(VersionChecker::new()),
            _watcher: watcher,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ComposeParams {
    /// The persona to use (e.g. "backend-engineer")
    persona: String,
    /// Skills to include (e.g. ["go", "postgresql"])
    #[serde(default)]
    skills: Vec<String>,
    /// Contexts to apply (e.g. ["code-review", "greenfield"])
    #[serde(default)]
    contexts: Vec<String>,
    /// Tones to use (e.g. ["concise", "teaching"])
    #[serde(default)]
    tones: Vec<String>,
    /// Constraints to apply (e.g. ["no-frameworks"])
    #[serde(default)]
    constraints: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListParams {
    /// Filter by category: persona, skill, context, tone, constraint
    #[serde(default)]
    category: Option<String>,
    /// Filter by tag
    #[serde(default)]
    tag: Option<String>,
    /// Filter by skill group: backend, frontend, mobile, infrastructure, data, security, general
    #[serde(default)]
    group: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct RecommendParams {
    /// The persona to get recommendations for (e.g. "backend-engineer")
    persona: String,
    /// Override the persona's skill_groups (e.g. ["backend", "data"]). Empty = use persona's groups.
    #[serde(default)]
    groups: Vec<String>,
    /// Filter recommendations by tags (e.g. ["security"]). Empty = no tag filter.
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RecommendResult {
    persona: RecommendPersona,
    skills: Vec<RecommendFragment>,
    contexts: Vec<RecommendFragment>,
    tones: Vec<RecommendFragment>,
    constraints: Vec<RecommendFragment>,
}

#[derive(Debug, Serialize)]
struct RecommendPersona {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<String>,
    skill_groups: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RecommendFragment {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

#[derive(Debug, Serialize)]
struct FragmentInfo {
    name: String,
    category: String,
    description: String,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    skill_groups: Vec<String>,
}

#[tool_router]
impl MimicServer {
    #[tool(
        description = "Compose a system prompt from fragments. Combines a persona with optional skills, contexts, tones, and constraints into a structured system prompt. Use 'recommend' first to see available fragments for a persona."
    )]
    fn compose(&self, Parameters(params): Parameters<ComposeParams>) -> Result<String, String> {
        let req = ComposeRequest {
            persona: params.persona,
            skills: params.skills,
            contexts: params.contexts,
            tones: params.tones,
            constraints: params.constraints,
        };
        let registry = self.registry.read().unwrap();
        let mut result = compose::compose(&registry, &req)?;

        if let Some(info) = self.version_checker.cached() {
            if info.update_available {
                result.push_str(&format!(
                    "\n\n---\n> Update available: mimic v{} (current: v{})",
                    info.latest.as_deref().unwrap_or("unknown"),
                    info.current,
                ));
            }
        }

        Ok(result)
    }

    #[tool(description = "Check if a newer version of mimic is available")]
    async fn check_update(&self) -> String {
        let info = self.version_checker.check().await;
        serde_json::to_string_pretty(&info).unwrap_or_else(|e| format!("error: {}", e))
    }

    #[tool(
        description = "Advanced: returns a raw graph of fragment relationships. \
            Most clients should use 'recommend' instead. \
            Given a persona, tags, or groups as a starting point, returns nodes and edges \
            connected by shared tags, groups, and skill_groups."
    )]
    fn resolve(
        &self,
        Parameters(params): Parameters<resolve::ResolveParams>,
    ) -> Result<String, String> {
        let registry = self.registry.read().unwrap();
        let result = resolve::resolve(&registry, &params)?;
        serde_json::to_string_pretty(&result)
            .map_err(|e| format!("Failed to serialize resolve result: {}", e))
    }

    #[tool(
        description = "Get recommended fragments for a persona. Returns a flat, categorized list of skills, contexts, tones, and constraints that match the persona's skill_groups. Use this before 'compose' to see what's available."
    )]
    fn recommend(&self, Parameters(params): Parameters<RecommendParams>) -> Result<String, String> {
        let registry = self.registry.read().unwrap();
        let persona = registry
            .get(Category::Persona, &params.persona)
            .ok_or_else(|| format!("Persona '{}' not found", params.persona))?;

        let effective_groups = if params.groups.is_empty() {
            &persona.skill_groups
        } else {
            &params.groups
        };

        let has_tag = |f: &crate::fragment::Fragment| -> bool {
            params.tags.is_empty() || params.tags.iter().any(|t| f.tags.contains(t))
        };

        // Skills: filter by effective groups (empty = all) and tags
        let skills: Vec<RecommendFragment> = registry
            .list(Some(Category::Skill), None, None)
            .into_iter()
            .filter(|f| {
                if effective_groups.is_empty() {
                    true
                } else {
                    f.group
                        .as_ref()
                        .is_some_and(|g| effective_groups.contains(g))
                }
            })
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: f.group.clone(),
            })
            .collect();

        let contexts: Vec<RecommendFragment> = registry
            .list(Some(Category::Context), None, None)
            .into_iter()
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        // Tones are always included (no tag filter)
        let tones: Vec<RecommendFragment> = registry
            .list(Some(Category::Tone), None, None)
            .into_iter()
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        let constraints: Vec<RecommendFragment> = registry
            .list(Some(Category::Constraint), None, None)
            .into_iter()
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        let result = RecommendResult {
            persona: RecommendPersona {
                name: persona.name.clone(),
                description: persona.description.clone(),
                level: persona.level.clone(),
                skill_groups: persona.skill_groups.clone(),
            },
            skills,
            contexts,
            tones,
            constraints,
        };

        serde_json::to_string_pretty(&result)
            .map_err(|e| format!("Failed to serialize recommend result: {}", e))
    }

    #[tool(
        description = "List available fragments. Returns a JSON array of fragments with name, category, description, and tags. Use to browse all fragments or discover personas."
    )]
    fn list(&self, Parameters(params): Parameters<ListParams>) -> String {
        let category = params.category.as_deref().and_then(parse_category);
        let registry = self.registry.read().unwrap();
        let fragments = registry.list(category, params.tag.as_deref(), params.group.as_deref());
        let infos: Vec<FragmentInfo> = fragments
            .into_iter()
            .map(|f| FragmentInfo {
                name: f.name.clone(),
                category: f.category.to_string(),
                description: f.description.clone(),
                tags: f.tags.clone(),
                group: f.group.clone(),
                level: f.level.clone(),
                skill_groups: f.skill_groups.clone(),
            })
            .collect();
        serde_json::to_string_pretty(&infos).unwrap_or_else(|_| "[]".to_string())
    }
}

#[tool_handler]
impl rmcp::ServerHandler for MimicServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "mimic".into(),
                title: Some("mimic — Composable Persona Prompts".into()),
                version: env!("CARGO_PKG_VERSION").into(),
                description: Some(env!("CARGO_PKG_DESCRIPTION").into()),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "mimic composes LLM system prompts from reusable markdown fragments.\n\n\
                 Workflow:\n\
                 1. Call 'recommend' with a persona name to see available skills, contexts, tones, and constraints.\n\
                 2. Call 'compose' with the persona and your chosen fragments to build a system prompt.\n\
                 3. Use 'list' to browse all fragments or discover personas.\n\n\
                 Example: recommend(persona: \"backend-engineer\") → pick skills → \
                 compose(persona: \"backend-engineer\", skills: [\"go\", \"postgresql\"], tones: [\"concise\"])"
                    .into(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            ..ServerInfo::default()
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, rmcp::ErrorData>> + Send + '_
    {
        let registry = self.registry.read().unwrap();
        let fragments = registry.list(None, None, None);
        let resources = fragments
            .into_iter()
            .map(|f| {
                let mut raw = RawResource::new(
                    format!("mimic://fragments/{}/{}", f.category.dir_name(), f.name),
                    &f.name,
                );
                raw.description = Some(f.description.clone());
                raw.mime_type = Some("text/markdown".into());
                raw.no_annotation()
            })
            .collect();
        std::future::ready(Ok(ListResourcesResult {
            resources,
            next_cursor: None,
            meta: None,
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, rmcp::ErrorData>> + Send + '_
    {
        let registry = self.registry.read().unwrap();
        let result = parse_resource_uri(&request.uri)
            .and_then(|(cat, name)| registry.get(cat, name))
            .map(|frag| ReadResourceResult {
                contents: vec![ResourceContents::text(&frag.body, &request.uri)],
            })
            .ok_or_else(|| {
                rmcp::ErrorData::resource_not_found(
                    format!("Resource not found: {}", request.uri),
                    None,
                )
            });
        std::future::ready(result)
    }

    fn complete(
        &self,
        request: CompleteRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CompleteResult, rmcp::ErrorData>> + Send + '_
    {
        let arg_name = &request.argument.name;
        let prefix = &request.argument.value;

        let registry = self.registry.read().unwrap();

        let category = match arg_name.as_str() {
            "persona" => Some(Category::Persona),
            "skills" => Some(Category::Skill),
            "context" | "contexts" => Some(Category::Context),
            "tone" | "tones" => Some(Category::Tone),
            "constraints" => Some(Category::Constraint),
            _ => None,
        };

        let values = if let Some(cat) = category {
            registry
                .names_for_category(cat)
                .iter()
                .filter(|n| n.starts_with(prefix))
                .take(CompletionInfo::MAX_VALUES)
                .cloned()
                .collect()
        } else if arg_name == "groups" {
            registry
                .all_groups()
                .iter()
                .filter(|g| g.starts_with(prefix))
                .take(CompletionInfo::MAX_VALUES)
                .cloned()
                .collect()
        } else if arg_name == "tags" {
            registry
                .all_tags()
                .iter()
                .filter(|t| t.starts_with(prefix))
                .take(CompletionInfo::MAX_VALUES)
                .cloned()
                .collect()
        } else {
            vec![]
        };

        std::future::ready(Ok(CompleteResult {
            completion: CompletionInfo {
                values,
                total: None,
                has_more: None,
            },
        }))
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, rmcp::ErrorData>> + Send + '_
    {
        let registry = self.registry.read().unwrap();
        let personas = registry.names_for_category(Category::Persona);
        let prompts = personas
            .iter()
            .map(|name| {
                use rmcp::model::PromptArgument;
                Prompt {
                    name: format!("mimic-{name}"),
                    title: None,
                    description: Some(format!("Compose the {name} persona prompt")),
                    arguments: Some(vec![
                        PromptArgument {
                            name: "skills".into(),
                            title: None,
                            description: Some(
                                "Comma-separated skill names (e.g. \"go,postgresql\")".into(),
                            ),
                            required: Some(false),
                        },
                        PromptArgument {
                            name: "tone".into(),
                            title: None,
                            description: Some(
                                "Tone to use (e.g. \"concise\"). Defaults to concise.".into(),
                            ),
                            required: Some(false),
                        },
                    ]),
                    icons: None,
                    meta: None,
                }
            })
            .collect();
        std::future::ready(Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
            meta: None,
        }))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, rmcp::ErrorData>> + Send + '_
    {
        let persona_name = request.name.strip_prefix("mimic-").unwrap_or(&request.name);
        let args = request.arguments.unwrap_or_default();
        let skills: Vec<String> = args
            .get("skills")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        let tones: Vec<String> = args
            .get("tone")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| vec![s.to_string()])
            .unwrap_or_else(|| vec!["concise".to_string()]);
        let req = ComposeRequest {
            persona: persona_name.to_string(),
            skills,
            contexts: vec![],
            tones,
            constraints: vec![],
        };
        let registry = self.registry.read().unwrap();
        let result = match compose::compose(&registry, &req) {
            Ok(text) => Ok(GetPromptResult {
                description: Some(format!("{} persona prompt", persona_name)),
                messages: vec![PromptMessage::new_text(PromptMessageRole::Assistant, text)],
            }),
            Err(e) => Err(rmcp::ErrorData::new(
                rmcp::model::ErrorCode::INVALID_PARAMS,
                e,
                None,
            )),
        };
        std::future::ready(result)
    }
}

fn parse_category(s: &str) -> Option<Category> {
    match s {
        "persona" => Some(Category::Persona),
        "skill" => Some(Category::Skill),
        "context" => Some(Category::Context),
        "tone" => Some(Category::Tone),
        "constraint" => Some(Category::Constraint),
        _ => None,
    }
}

fn parse_resource_uri(uri: &str) -> Option<(Category, &str)> {
    let path = uri.strip_prefix("mimic://fragments/")?;
    let (cat_dir, name) = path.split_once('/')?;
    let category = Category::from_dir_name(cat_dir)?;
    Some((category, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_category_persona() {
        assert_eq!(parse_category("persona"), Some(Category::Persona));
    }

    #[test]
    fn parse_category_skill() {
        assert_eq!(parse_category("skill"), Some(Category::Skill));
    }

    #[test]
    fn parse_category_context() {
        assert_eq!(parse_category("context"), Some(Category::Context));
    }

    #[test]
    fn parse_category_tone() {
        assert_eq!(parse_category("tone"), Some(Category::Tone));
    }

    #[test]
    fn parse_category_constraint() {
        assert_eq!(parse_category("constraint"), Some(Category::Constraint));
    }

    #[test]
    fn parse_category_invalid() {
        assert_eq!(parse_category("unknown"), None);
        assert_eq!(parse_category(""), None);
        assert_eq!(parse_category("Persona"), None);
    }

    #[test]
    fn parse_resource_uri_valid() {
        let (cat, name) =
            parse_resource_uri("mimic://fragments/personas/backend-engineer").unwrap();
        assert_eq!(cat, Category::Persona);
        assert_eq!(name, "backend-engineer");
    }

    #[test]
    fn parse_resource_uri_valid_skill() {
        let (cat, name) = parse_resource_uri("mimic://fragments/skills/go").unwrap();
        assert_eq!(cat, Category::Skill);
        assert_eq!(name, "go");
    }

    #[test]
    fn parse_resource_uri_wrong_prefix() {
        assert!(parse_resource_uri("other://fragments/personas/test").is_none());
    }

    #[test]
    fn parse_resource_uri_missing_parts() {
        assert!(parse_resource_uri("mimic://fragments/").is_none());
        assert!(parse_resource_uri("mimic://fragments/personas").is_none());
    }

    #[test]
    fn parse_resource_uri_invalid_category() {
        assert!(parse_resource_uri("mimic://fragments/unknown/test").is_none());
    }

    fn recommend(
        registry: &Registry,
        persona: &str,
        groups: Vec<String>,
        tags: Vec<String>,
    ) -> Result<RecommendResult, String> {
        let params = RecommendParams {
            persona: persona.to_string(),
            groups,
            tags,
        };
        let p = registry
            .get(Category::Persona, &params.persona)
            .ok_or_else(|| format!("Persona '{}' not found", params.persona))?;

        let effective_groups = if params.groups.is_empty() {
            &p.skill_groups
        } else {
            &params.groups
        };

        let has_tag = |f: &crate::fragment::Fragment| -> bool {
            params.tags.is_empty() || params.tags.iter().any(|t| f.tags.contains(t))
        };

        let skills: Vec<RecommendFragment> = registry
            .list(Some(Category::Skill), None, None)
            .into_iter()
            .filter(|f| {
                if effective_groups.is_empty() {
                    true
                } else {
                    f.group
                        .as_ref()
                        .is_some_and(|g| effective_groups.contains(g))
                }
            })
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: f.group.clone(),
            })
            .collect();

        let contexts: Vec<RecommendFragment> = registry
            .list(Some(Category::Context), None, None)
            .into_iter()
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        let tones: Vec<RecommendFragment> = registry
            .list(Some(Category::Tone), None, None)
            .into_iter()
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        let constraints: Vec<RecommendFragment> = registry
            .list(Some(Category::Constraint), None, None)
            .into_iter()
            .filter(|f| has_tag(f))
            .map(|f| RecommendFragment {
                name: f.name.clone(),
                description: f.description.clone(),
                group: None,
            })
            .collect();

        Ok(RecommendResult {
            persona: RecommendPersona {
                name: p.name.clone(),
                description: p.description.clone(),
                level: p.level.clone(),
                skill_groups: p.skill_groups.clone(),
            },
            skills,
            contexts,
            tones,
            constraints,
        })
    }

    #[test]
    fn recommend_returns_skills_for_persona() {
        let registry = Registry::new(None);
        let result = recommend(&registry, "backend-engineer", vec![], vec![]).unwrap();
        assert_eq!(result.persona.name, "backend-engineer");
        assert!(!result.skills.is_empty(), "should have skills");
        // Skills should be filtered by persona's skill_groups
        let persona = registry.get(Category::Persona, "backend-engineer").unwrap();
        if !persona.skill_groups.is_empty() {
            for skill in &result.skills {
                assert!(
                    skill
                        .group
                        .as_ref()
                        .is_some_and(|g| persona.skill_groups.contains(g)),
                    "skill {} should belong to persona's skill_groups",
                    skill.name
                );
            }
        }
    }

    #[test]
    fn recommend_groups_override() {
        let registry = Registry::new(None);
        let result = recommend(
            &registry,
            "backend-engineer",
            vec!["frontend".to_string()],
            vec![],
        )
        .unwrap();
        for skill in &result.skills {
            assert_eq!(
                skill.group.as_deref(),
                Some("frontend"),
                "skill {} should be in frontend group",
                skill.name
            );
        }
    }

    #[test]
    fn recommend_tag_filter() {
        let registry = Registry::new(None);
        let result = recommend(
            &registry,
            "backend-engineer",
            vec![],
            vec!["security".to_string()],
        )
        .unwrap();
        // Tones are always included regardless of tags
        assert!(!result.tones.is_empty(), "tones should always be included");
        // If there are skills, they should have the security tag
        for skill in &result.skills {
            let frag = registry.get(Category::Skill, &skill.name).unwrap();
            assert!(
                frag.tags.contains(&"security".to_string()),
                "skill {} should have security tag",
                skill.name
            );
        }
    }

    #[test]
    fn recommend_unknown_persona_errors() {
        let registry = Registry::new(None);
        let result = recommend(&registry, "nonexistent-persona", vec![], vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn recommend_includes_all_categories() {
        let registry = Registry::new(None);
        let result = recommend(&registry, "backend-engineer", vec![], vec![]).unwrap();
        assert!(!result.tones.is_empty(), "should have tones");
        assert!(!result.contexts.is_empty(), "should have contexts");
        // Constraints and skills may or may not be empty depending on builtins
    }

    #[test]
    fn parse_resource_uri_all_categories() {
        for cat in Category::all() {
            let uri = format!("mimic://fragments/{}/test-name", cat.dir_name());
            let (parsed_cat, parsed_name) =
                parse_resource_uri(&uri).unwrap_or_else(|| panic!("Failed to parse URI: {}", uri));
            assert_eq!(parsed_cat, *cat);
            assert_eq!(parsed_name, "test-name");
        }
    }
}
