use crate::fragment::{Category, Fragment};
use crate::registry::Registry;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResolveParams {
    /// Starting persona (e.g. "backend-engineer"). Anchors the graph but isn't required.
    #[serde(default)]
    pub persona: Option<String>,
    /// Seed tags to filter the graph. Only fragments sharing at least one tag are included
    /// (tones are always included).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Filter skills to these groups. Overrides persona's skill_groups when provided.
    #[serde(default)]
    pub groups: Vec<String>,
    /// Whether to include edges in the result (default true). Set to false for a lighter response.
    #[serde(default = "default_true")]
    pub include_edges: Option<bool>,
}

fn default_true() -> Option<bool> {
    Some(true)
}

#[derive(Debug, Serialize)]
pub struct ResolveResult {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub meta: ResolveMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub id: String,
    pub category: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skill_groups: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct ResolveMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
    pub resolved_groups: Vec<String>,
    pub resolved_tags: Vec<String>,
    pub node_count: usize,
    pub edge_count: usize,
}

fn node_id(category: &Category, name: &str) -> String {
    format!("{}:{}", category, name)
}

fn fragment_to_node(f: &Fragment) -> Node {
    Node {
        id: node_id(&f.category, &f.name),
        category: f.category.to_string(),
        name: f.name.clone(),
        description: f.description.clone(),
        tags: f.tags.clone(),
        level: f.level.clone(),
        skill_groups: f.skill_groups.clone(),
        group: f.group.clone(),
        source: format!("{:?}", f.source).to_lowercase(),
    }
}

fn build_edges(nodes: &[Node]) -> Vec<Edge> {
    let threshold = nodes.len() / 2;

    // Build tag frequency counts and identify common tags
    let mut tag_counts: HashMap<&str, usize> = HashMap::new();
    for node in nodes {
        for tag in &node.tags {
            *tag_counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }

    // Index skills by group (used for both group edges and skill_group edges)
    let mut skills_by_group: HashMap<&str, Vec<usize>> = HashMap::new();
    // Index nodes by uncommon tag
    let mut by_tag: HashMap<&str, Vec<usize>> = HashMap::new();
    // Collect personas for skill_group edges
    let mut personas: Vec<usize> = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        if node.category == "persona" {
            personas.push(i);
        }

        if node.category == "skill" {
            if let Some(ref group) = node.group {
                skills_by_group.entry(group.as_str()).or_default().push(i);
            }
        }

        for tag in &node.tags {
            if tag_counts.get(tag.as_str()).copied().unwrap_or(0) <= threshold {
                by_tag.entry(tag.as_str()).or_default().push(i);
            }
        }
    }

    let mut edges = Vec::new();

    // skill_group edges: persona -> skill when persona's skill_groups contains skill's group
    for &pi in &personas {
        let persona = &nodes[pi];
        for sg in &persona.skill_groups {
            if let Some(skill_indices) = skills_by_group.get(sg.as_str()) {
                for &si in skill_indices {
                    let skill = &nodes[si];
                    edges.push(Edge {
                        from: persona.id.clone(),
                        to: skill.id.clone(),
                        relation: "skill_group".to_string(),
                        label: sg.clone(),
                    });
                }
            }
        }
    }

    // group edges: skill <-> skill when same group (triangle iteration guarantees uniqueness)
    for (group, indices) in &skills_by_group {
        for (pos, &i) in indices.iter().enumerate() {
            for &j in &indices[pos + 1..] {
                edges.push(Edge {
                    from: nodes[i].id.clone(),
                    to: nodes[j].id.clone(),
                    relation: "group".to_string(),
                    label: group.to_string(),
                });
            }
        }
    }

    // tag edges: any <-> any sharing uncommon tags (triangle iteration guarantees uniqueness per tag)
    for (tag, indices) in &by_tag {
        for (pos, &i) in indices.iter().enumerate() {
            for &j in &indices[pos + 1..] {
                edges.push(Edge {
                    from: nodes[i].id.clone(),
                    to: nodes[j].id.clone(),
                    relation: "tag".to_string(),
                    label: tag.to_string(),
                });
            }
        }
    }

    edges
}

pub fn resolve(registry: &Registry, params: &ResolveParams) -> Result<ResolveResult, String> {
    // Step 1: Seed resolution
    let persona_fragment = if let Some(ref persona_name) = params.persona {
        let f = registry
            .get(Category::Persona, persona_name)
            .ok_or_else(|| format!("Persona '{}' not found", persona_name))?;
        Some(f)
    } else {
        None
    };

    // Determine effective groups
    let effective_groups_vec: Vec<String> = if !params.groups.is_empty() {
        params.groups.clone()
    } else if let Some(pf) = persona_fragment {
        pf.skill_groups.clone()
    } else {
        vec![] // empty means all groups
    };
    let effective_groups: HashSet<&str> = effective_groups_vec.iter().map(|s| s.as_str()).collect();

    let has_user_tags = !params.tags.is_empty();
    let tag_set: HashSet<&str> = params.tags.iter().map(|s| s.as_str()).collect();

    // Step 2: Gather nodes
    let mut nodes: Vec<Node> = Vec::new();
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();

    // Add persona if provided
    if let Some(pf) = persona_fragment {
        let node = fragment_to_node(pf);
        seen_ids.insert(node.id.clone());
        nodes.push(node);
    }

    // Add skills filtered by effective groups
    let all_skills = registry.list(Some(Category::Skill), None, None);
    for skill in all_skills {
        // If we have effective groups, filter by them
        if !effective_groups.is_empty() {
            if let Some(ref g) = skill.group {
                if !effective_groups.contains(g.as_str()) {
                    continue;
                }
            } else {
                continue; // skill has no group, skip when filtering by groups
            }
        }

        // If user provided tags, hard-filter: only include if sharing at least one tag
        if has_user_tags && !skill.tags.iter().any(|t| tag_set.contains(t.as_str())) {
            continue;
        }

        let node = fragment_to_node(skill);
        if seen_ids.insert(node.id.clone()) {
            nodes.push(node);
        }
    }

    // Add contexts, tones, constraints
    for category in &[Category::Context, Category::Tone, Category::Constraint] {
        let fragments = registry.list(Some(*category), None, None);
        for f in fragments {
            // Tones always included; others filtered by tags if user provided tags
            if has_user_tags
                && *category != Category::Tone
                && !f.tags.iter().any(|t| tag_set.contains(t.as_str()))
            {
                continue;
            }

            let node = fragment_to_node(f);
            if seen_ids.insert(node.id.clone()) {
                nodes.push(node);
            }
        }
    }

    // Step 3: Build edges
    let include_edges = params.include_edges.unwrap_or(true);
    let edges = if include_edges {
        build_edges(&nodes)
    } else {
        vec![]
    };

    // Step 4: Build meta
    let resolved_tags: Vec<String> = if has_user_tags {
        params.tags.clone()
    } else {
        vec![]
    };

    let meta = ResolveMeta {
        seed: params
            .persona
            .as_ref()
            .map(|p| node_id(&Category::Persona, p)),
        resolved_groups: effective_groups_vec,
        resolved_tags,
        node_count: nodes.len(),
        edge_count: edges.len(),
    };

    Ok(ResolveResult { nodes, edges, meta })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> Registry {
        Registry::new(None)
    }

    #[test]
    fn persona_seeded_resolve() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: Some("backend-engineer".to_string()),
                tags: vec![],
                groups: vec![],
                include_edges: Some(true),
            },
        )
        .unwrap();

        // Should have persona node
        assert!(result
            .nodes
            .iter()
            .any(|n| n.id == "persona:backend-engineer"));
        // Should have skills from its skill_groups
        assert!(result.nodes.iter().any(|n| n.category == "skill"));
        // Should have contexts, tones, constraints
        assert!(result.nodes.iter().any(|n| n.category == "tone"));
        assert!(result.nodes.iter().any(|n| n.category == "context"));
        assert!(result.nodes.iter().any(|n| n.category == "constraint"));
        // Meta seed should be set
        assert_eq!(
            result.meta.seed,
            Some("persona:backend-engineer".to_string())
        );
    }

    #[test]
    fn skill_group_edges_from_persona() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: Some("backend-engineer".to_string()),
                tags: vec![],
                groups: vec![],
                include_edges: Some(true),
            },
        )
        .unwrap();

        let skill_group_edges: Vec<&Edge> = result
            .edges
            .iter()
            .filter(|e| e.relation == "skill_group")
            .collect();
        assert!(
            !skill_group_edges.is_empty(),
            "Should have skill_group edges"
        );
        // All skill_group edges should come from the persona
        for e in &skill_group_edges {
            assert_eq!(e.from, "persona:backend-engineer");
        }
    }

    #[test]
    fn tag_edges_connect_fragments() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec![],
                include_edges: Some(true),
            },
        )
        .unwrap();

        let tag_edges: Vec<&Edge> = result
            .edges
            .iter()
            .filter(|e| e.relation == "tag")
            .collect();
        // With all fragments, there should be some tag edges
        // (unless all tags are "common" which is unlikely)
        // Just check the structure is valid
        for e in &tag_edges {
            assert!(result.nodes.iter().any(|n| n.id == e.from));
            assert!(result.nodes.iter().any(|n| n.id == e.to));
        }
    }

    #[test]
    fn group_edges_connect_skills() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec!["backend".to_string()],
                include_edges: Some(true),
            },
        )
        .unwrap();

        let group_edges: Vec<&Edge> = result
            .edges
            .iter()
            .filter(|e| e.relation == "group")
            .collect();
        for e in &group_edges {
            assert_eq!(e.label, "backend");
        }
    }

    #[test]
    fn tag_filter_narrows_nodes() {
        let reg = registry();
        let all = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec![],
                include_edges: Some(false),
            },
        )
        .unwrap();

        let filtered = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec!["security".to_string()],
                groups: vec![],
                include_edges: Some(false),
            },
        )
        .unwrap();

        assert!(
            filtered.meta.node_count < all.meta.node_count,
            "Tag filter should narrow node set: filtered={} all={}",
            filtered.meta.node_count,
            all.meta.node_count
        );
        // Non-tone nodes should all have the security tag
        for node in &filtered.nodes {
            if node.category != "tone" {
                assert!(
                    node.tags.contains(&"security".to_string()),
                    "Node {} should have 'security' tag",
                    node.id
                );
            }
        }
    }

    #[test]
    fn group_override_replaces_persona_skill_groups() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: Some("backend-engineer".to_string()),
                tags: vec![],
                groups: vec!["frontend".to_string()],
                include_edges: Some(false),
            },
        )
        .unwrap();

        assert_eq!(result.meta.resolved_groups, vec!["frontend"]);
        // Skills should be from frontend group only
        for node in &result.nodes {
            if node.category == "skill" {
                assert_eq!(
                    node.group.as_deref(),
                    Some("frontend"),
                    "Skill {} should be in frontend group",
                    node.id
                );
            }
        }
    }

    #[test]
    fn no_persona_resolve() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec![],
                include_edges: Some(true),
            },
        )
        .unwrap();

        // No persona node
        assert!(!result.nodes.iter().any(|n| n.category == "persona"));
        assert!(result.meta.seed.is_none());
        // No skill_group edges (those require a persona)
        assert!(!result.edges.iter().any(|e| e.relation == "skill_group"));
        // But should have group and/or tag edges
        assert!(!result.edges.is_empty());
    }

    #[test]
    fn nonexistent_persona_errors() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: Some("nonexistent-persona".to_string()),
                tags: vec![],
                groups: vec![],
                include_edges: Some(true),
            },
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn include_edges_false_returns_empty_edges() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: Some("backend-engineer".to_string()),
                tags: vec![],
                groups: vec![],
                include_edges: Some(false),
            },
        )
        .unwrap();

        assert!(result.edges.is_empty());
        assert_eq!(result.meta.edge_count, 0);
        assert!(!result.nodes.is_empty());
    }

    #[test]
    fn fragment_to_node_maps_all_fields() {
        use crate::fragment::Source;

        let frag = Fragment {
            name: "test-skill".to_string(),
            category: Category::Skill,
            description: "A test skill".to_string(),
            tags: vec!["rust".to_string(), "backend".to_string()],
            group: Some("backend".to_string()),
            level: Some("senior".to_string()),
            skill_groups: vec!["backend".to_string(), "general".to_string()],
            body: "Test body.".to_string(),
            source: Source::Project,
        };

        let node = fragment_to_node(&frag);
        assert_eq!(node.id, "skill:test-skill");
        assert_eq!(node.category, "skill");
        assert_eq!(node.name, "test-skill");
        assert_eq!(node.description, "A test skill");
        assert_eq!(node.tags, vec!["rust", "backend"]);
        assert_eq!(node.group.as_deref(), Some("backend"));
        assert_eq!(node.level.as_deref(), Some("senior"));
        assert_eq!(node.skill_groups, vec!["backend", "general"]);
        assert_eq!(node.source, "project");
    }

    #[test]
    fn tag_filter_always_includes_tones() {
        let reg = registry();
        let result = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec!["nonexistent-tag-xyz".to_string()],
                groups: vec![],
                include_edges: Some(false),
            },
        )
        .unwrap();

        // Tones should still be present
        assert!(
            result.nodes.iter().any(|n| n.category == "tone"),
            "Tones should always be included regardless of tag filter"
        );
        // Skills should be excluded (none match the nonsense tag)
        assert!(
            !result.nodes.iter().any(|n| n.category == "skill"),
            "No skills should match a nonsense tag"
        );
    }

    #[test]
    fn empty_groups_includes_all_skills() {
        let reg = registry();
        let all = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec![],
                include_edges: Some(false),
            },
        )
        .unwrap();

        let one_group = resolve(
            &reg,
            &ResolveParams {
                persona: None,
                tags: vec![],
                groups: vec!["backend".to_string()],
                include_edges: Some(false),
            },
        )
        .unwrap();

        let all_skills: Vec<_> = all.nodes.iter().filter(|n| n.category == "skill").collect();
        let group_skills: Vec<_> = one_group
            .nodes
            .iter()
            .filter(|n| n.category == "skill")
            .collect();
        assert!(
            all_skills.len() > group_skills.len(),
            "Empty groups should include more skills ({}) than filtering to one group ({})",
            all_skills.len(),
            group_skills.len()
        );
    }

    #[test]
    fn common_tag_exclusion() {
        // Build a set of nodes where one tag appears on >50%
        let nodes = vec![
            Node {
                id: "skill:a".to_string(),
                category: "skill".to_string(),
                name: "a".to_string(),
                description: String::new(),
                tags: vec!["common".to_string(), "unique-a".to_string()],
                level: None,
                skill_groups: vec![],
                group: Some("test".to_string()),
                source: "builtin".to_string(),
            },
            Node {
                id: "skill:b".to_string(),
                category: "skill".to_string(),
                name: "b".to_string(),
                description: String::new(),
                tags: vec!["common".to_string(), "unique-b".to_string()],
                level: None,
                skill_groups: vec![],
                group: Some("test".to_string()),
                source: "builtin".to_string(),
            },
            Node {
                id: "skill:c".to_string(),
                category: "skill".to_string(),
                name: "c".to_string(),
                description: String::new(),
                tags: vec!["common".to_string()],
                level: None,
                skill_groups: vec![],
                group: Some("test".to_string()),
                source: "builtin".to_string(),
            },
        ];

        // "common" appears on 3/3 nodes (>50%), so no tag edges for "common"
        let edges = build_edges(&nodes);
        let common_tag_edges: Vec<&Edge> = edges
            .iter()
            .filter(|e| e.relation == "tag" && e.label == "common")
            .collect();
        assert!(
            common_tag_edges.is_empty(),
            "Common tags (>50%) should not generate edges"
        );
    }
}
