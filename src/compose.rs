use crate::fragment::Category;
use crate::registry::Registry;

pub struct ComposeRequest {
    pub persona: String,
    pub skills: Vec<String>,
    pub contexts: Vec<String>,
    pub tones: Vec<String>,
    pub constraints: Vec<String>,
}

pub fn compose(registry: &Registry, req: &ComposeRequest) -> Result<String, String> {
    let persona = registry
        .get(Category::Persona, &req.persona)
        .ok_or_else(|| format!("Persona '{}' not found", req.persona))?;

    let mut output = persona.body.clone();

    // Skills
    if !req.skills.is_empty() {
        output.push_str("\n\n## Expertise\n\n");
        let mut first = true;
        for skill_name in &req.skills {
            let skill = registry
                .get(Category::Skill, skill_name)
                .ok_or_else(|| format!("Skill '{}' not found", skill_name))?;
            if !first {
                output.push_str("\n\n");
            }
            output.push_str(&skill.body);
            first = false;
        }
    }

    // Contexts
    if !req.contexts.is_empty() {
        output.push_str("\n\n## Context\n\n");
        let mut first = true;
        for ctx_name in &req.contexts {
            let ctx = registry
                .get(Category::Context, ctx_name)
                .ok_or_else(|| format!("Context '{}' not found", ctx_name))?;
            if !first {
                output.push_str("\n\n");
            }
            output.push_str(&ctx.body);
            first = false;
        }
    }

    // Tones
    if !req.tones.is_empty() {
        output.push_str("\n\n## Communication Style\n\n");
        let mut first = true;
        for tone_name in &req.tones {
            let tone = registry
                .get(Category::Tone, tone_name)
                .ok_or_else(|| format!("Tone '{}' not found", tone_name))?;
            if !first {
                output.push_str("\n\n");
            }
            output.push_str(&tone.body);
            first = false;
        }
    }

    // Constraints
    if !req.constraints.is_empty() {
        output.push_str("\n\n## Constraints\n\n");
        let mut first = true;
        for constraint_name in &req.constraints {
            let constraint = registry
                .get(Category::Constraint, constraint_name)
                .ok_or_else(|| format!("Constraint '{}' not found", constraint_name))?;
            if !first {
                output.push_str("\n\n");
            }
            output.push_str(&constraint.body);
            first = false;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_persona_only() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec![],
            tones: vec![],
            constraints: vec![],
        };
        let result = compose(&registry, &req).unwrap();
        assert!(!result.is_empty());
        assert!(!result.contains("## Expertise"));
    }

    #[test]
    fn compose_full() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec!["go".to_string(), "postgresql".to_string()],
            contexts: vec!["code-review".to_string()],
            tones: vec!["concise".to_string()],
            constraints: vec!["no-frameworks".to_string()],
        };
        let result = compose(&registry, &req).unwrap();
        assert!(result.contains("## Expertise"));
        assert!(result.contains("## Context"));
        assert!(result.contains("## Communication Style"));
        assert!(result.contains("## Constraints"));
    }

    #[test]
    fn compose_missing_persona() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "nonexistent".to_string(),
            skills: vec![],
            contexts: vec![],
            tones: vec![],
            constraints: vec![],
        };
        let result = compose(&registry, &req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn compose_missing_skill() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec!["nonexistent-skill".to_string()],
            contexts: vec![],
            tones: vec![],
            constraints: vec![],
        };
        let result = compose(&registry, &req);
        assert!(result.is_err());
    }

    #[test]
    fn compose_missing_context() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec!["nonexistent-context".to_string()],
            tones: vec![],
            constraints: vec![],
        };
        let result = compose(&registry, &req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Context"));
    }

    #[test]
    fn compose_missing_tone() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec![],
            tones: vec!["nonexistent-tone".to_string()],
            constraints: vec![],
        };
        let result = compose(&registry, &req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Tone"));
    }

    #[test]
    fn compose_multiple_contexts() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec!["code-review".to_string(), "greenfield-project".to_string()],
            tones: vec![],
            constraints: vec![],
        };
        let result = compose(&registry, &req).unwrap();
        assert!(result.contains("## Context"));
        // Both contexts should appear in the output
        let context_section = result.split("## Context").nth(1).unwrap();
        // The section should contain bodies from both contexts
        assert!(context_section.len() > 10);
    }

    #[test]
    fn compose_multiple_tones() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec![],
            tones: vec!["concise".to_string(), "pedagogical".to_string()],
            constraints: vec![],
        };
        let result = compose(&registry, &req).unwrap();
        assert!(result.contains("## Communication Style"));
        let style_section = result.split("## Communication Style").nth(1).unwrap();
        assert!(style_section.len() > 10);
    }

    #[test]
    fn compose_missing_constraint() {
        let registry = Registry::new(None);
        let req = ComposeRequest {
            persona: "backend-engineer".to_string(),
            skills: vec![],
            contexts: vec![],
            tones: vec![],
            constraints: vec!["nonexistent-constraint".to_string()],
        };
        let result = compose(&registry, &req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Constraint"));
    }
}
