use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Persona,
    Skill,
    Context,
    Tone,
    Constraint,
}

impl Category {
    pub fn all() -> &'static [Category] {
        &[
            Category::Persona,
            Category::Skill,
            Category::Context,
            Category::Tone,
            Category::Constraint,
        ]
    }

    pub fn dir_name(&self) -> &'static str {
        match self {
            Category::Persona => "personas",
            Category::Skill => "skills",
            Category::Context => "contexts",
            Category::Tone => "tones",
            Category::Constraint => "constraints",
        }
    }

    pub fn from_dir_name(name: &str) -> Option<Category> {
        match name {
            "personas" => Some(Category::Persona),
            "skills" => Some(Category::Skill),
            "contexts" => Some(Category::Context),
            "tones" => Some(Category::Tone),
            "constraints" => Some(Category::Constraint),
            _ => None,
        }
    }

    /// Accepts both singular ("skill") and plural ("skills") forms.
    pub fn from_name(name: &str) -> Option<Category> {
        match name {
            "persona" | "personas" => Some(Category::Persona),
            "skill" | "skills" => Some(Category::Skill),
            "context" | "contexts" => Some(Category::Context),
            "tone" | "tones" => Some(Category::Tone),
            "constraint" | "constraints" => Some(Category::Constraint),
            _ => None,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Persona => write!(f, "persona"),
            Category::Skill => write!(f, "skill"),
            Category::Context => write!(f, "context"),
            Category::Tone => write!(f, "tone"),
            Category::Constraint => write!(f, "constraint"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub skill_groups: Vec<String>,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    BuiltIn,
    Global,
    Project,
}

#[derive(Debug, Clone, Serialize)]
pub struct Fragment {
    pub name: String,
    pub category: Category,
    pub description: String,
    pub tags: Vec<String>,
    pub group: Option<String>,
    pub level: Option<String>,
    pub skill_groups: Vec<String>,
    pub body: String,
    pub source: Source,
}

impl Fragment {
    pub fn parse(content: &str, name: &str, category: Category, source: Source) -> Fragment {
        let (frontmatter, body) = parse_frontmatter(content);
        let body = body.trim().to_string();
        let description = frontmatter
            .description
            .unwrap_or_else(|| first_line(&body).to_string());
        Fragment {
            name: name.to_string(),
            category,
            description,
            tags: frontmatter.tags,
            group: frontmatter.group,
            level: frontmatter.level,
            skill_groups: frontmatter.skill_groups,
            body,
            source,
        }
    }

    pub fn from_file(path: &Path, category: Category, source: Source) -> Option<Fragment> {
        let name = path.file_stem()?.to_str()?;
        let content = std::fs::read_to_string(path).ok()?;
        Some(Fragment::parse(&content, name, category, source))
    }
}

fn parse_frontmatter(content: &str) -> (Frontmatter, &str) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (Frontmatter::default(), content);
    }

    let after_first_fence = &trimmed[3..];
    if let Some(end) = after_first_fence.find("\n---") {
        let yaml = &after_first_fence[..end];
        let body = &after_first_fence[end + 4..];
        let fm: Frontmatter = serde_yaml::from_str(yaml).unwrap_or_default();
        (fm, body)
    } else {
        (Frontmatter::default(), content)
    }
}

/// Strict frontmatter parser that returns errors and unknown fields.
/// Returns `(frontmatter, unknown_field_names, body)` or an error string.
pub fn parse_frontmatter_strict(
    content: &str,
) -> Result<(Frontmatter, Vec<String>, String), String> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Ok((Frontmatter::default(), vec![], content.to_string()));
    }

    let after_first_fence = &trimmed[3..];
    let Some(end) = after_first_fence.find("\n---") else {
        return Ok((Frontmatter::default(), vec![], content.to_string()));
    };

    let yaml = &after_first_fence[..end];
    let body = &after_first_fence[end + 4..];

    // First parse as Value to detect unknown fields
    let value: serde_yaml::Value = serde_yaml::from_str(yaml).map_err(|e| format!("{e}"))?;

    let known_fields: &[&str] = &[
        "description",
        "tags",
        "group",
        "level",
        "skill_groups",
        "category",
    ];
    let mut unknown_fields = Vec::new();

    if let serde_yaml::Value::Mapping(ref map) = value {
        for key in map.keys() {
            if let serde_yaml::Value::String(ref k) = key {
                if !known_fields.contains(&k.as_str()) {
                    unknown_fields.push(k.clone());
                }
            }
        }
    }

    // Now deserialize into Frontmatter
    let fm: Frontmatter = serde_yaml::from_value(value).map_err(|e| format!("{e}"))?;

    Ok((fm, unknown_fields, body.to_string()))
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_frontmatter() {
        let content = r#"---
description: A senior backend engineer
tags: [backend, apis]
---
You are a senior backend engineer."#;
        let frag = Fragment::parse(
            content,
            "backend-engineer",
            Category::Persona,
            Source::BuiltIn,
        );
        assert_eq!(frag.name, "backend-engineer");
        assert_eq!(frag.description, "A senior backend engineer");
        assert_eq!(frag.tags, vec!["backend", "apis"]);
        assert_eq!(frag.body, "You are a senior backend engineer.");
    }

    #[test]
    fn parse_without_frontmatter() {
        let content = "You are a senior backend engineer.\nYou love Rust.";
        let frag = Fragment::parse(
            content,
            "backend-engineer",
            Category::Persona,
            Source::BuiltIn,
        );
        assert_eq!(frag.description, "You are a senior backend engineer.");
        assert!(frag.tags.is_empty());
        assert_eq!(frag.body, content);
    }

    #[test]
    fn parse_empty_frontmatter() {
        let content = "---\n---\nJust a body.";
        let frag = Fragment::parse(content, "test", Category::Skill, Source::Global);
        assert_eq!(frag.description, "Just a body.");
        assert!(frag.tags.is_empty());
    }

    #[test]
    fn parse_with_group_and_level() {
        let content = r#"---
description: Go expertise
tags: [go]
group: backend
---
You write idiomatic Go."#;
        let frag = Fragment::parse(content, "go", Category::Skill, Source::BuiltIn);
        assert_eq!(frag.group.as_deref(), Some("backend"));
        assert_eq!(frag.level, None);
        assert!(frag.skill_groups.is_empty());
    }

    #[test]
    fn parse_with_skill_groups_and_level() {
        let content = r#"---
description: Senior backend engineer
tags: [backend]
level: senior
skill_groups: [backend, data, general]
---
You are a senior backend engineer."#;
        let frag = Fragment::parse(
            content,
            "backend-engineer",
            Category::Persona,
            Source::BuiltIn,
        );
        assert_eq!(frag.level.as_deref(), Some("senior"));
        assert_eq!(frag.skill_groups, vec!["backend", "data", "general"]);
        assert_eq!(frag.group, None);
    }

    #[test]
    fn category_dir_roundtrip() {
        for cat in Category::all() {
            let dir = cat.dir_name();
            assert_eq!(Category::from_dir_name(dir), Some(*cat));
        }
    }

    #[test]
    fn from_name_singular_and_plural() {
        assert_eq!(Category::from_name("skill"), Some(Category::Skill));
        assert_eq!(Category::from_name("skills"), Some(Category::Skill));
        assert_eq!(Category::from_name("persona"), Some(Category::Persona));
        assert_eq!(Category::from_name("personas"), Some(Category::Persona));
        assert_eq!(Category::from_name("context"), Some(Category::Context));
        assert_eq!(Category::from_name("contexts"), Some(Category::Context));
        assert_eq!(Category::from_name("tone"), Some(Category::Tone));
        assert_eq!(Category::from_name("tones"), Some(Category::Tone));
        assert_eq!(
            Category::from_name("constraint"),
            Some(Category::Constraint)
        );
        assert_eq!(
            Category::from_name("constraints"),
            Some(Category::Constraint)
        );
        assert_eq!(Category::from_name("invalid"), None);
    }

    #[test]
    fn from_file_valid() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("test-skill.md");
        std::fs::write(
            &path,
            "---\ndescription: A test skill\ntags: [rust]\ngroup: backend\n---\nSkill body here.",
        )
        .unwrap();

        let frag = Fragment::from_file(&path, Category::Skill, Source::Project).unwrap();
        assert_eq!(frag.name, "test-skill");
        assert_eq!(frag.category, Category::Skill);
        assert_eq!(frag.source, Source::Project);
        assert_eq!(frag.description, "A test skill");
        assert_eq!(frag.tags, vec!["rust"]);
        assert_eq!(frag.group.as_deref(), Some("backend"));
    }

    #[test]
    fn from_file_nonexistent() {
        let result = Fragment::from_file(
            std::path::Path::new("/tmp/nonexistent-fragment.md"),
            Category::Skill,
            Source::BuiltIn,
        );
        assert!(result.is_none());
    }

    #[test]
    fn from_file_dotted_name() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("my.dotted.name.md");
        std::fs::write(&path, "Dotted name body.").unwrap();

        let frag = Fragment::from_file(&path, Category::Context, Source::Global).unwrap();
        assert_eq!(frag.name, "my.dotted.name");
    }
}
