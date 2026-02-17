use crate::fragment::{parse_frontmatter_strict, Category, Frontmatter, Source};
use include_dir::{include_dir, Dir};
use std::fmt;
use std::path::Path;

static DEFAULTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/defaults");

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub struct FragmentContext {
    pub file_path: String,
    pub filename_stem: String,
    pub category: Category,
    pub source: Source,
    pub raw_content: String,
    pub frontmatter: Option<Frontmatter>,
    pub yaml_error: Option<String>,
    pub unknown_fields: Vec<String>,
    pub body: String,
    pub is_root_level: bool,
}

#[derive(Debug)]
pub struct LintDiagnostic {
    pub severity: Severity,
    pub file_path: String,
    pub rule: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

// ---------------------------------------------------------------------------
// LintRule trait
// ---------------------------------------------------------------------------

pub trait LintRule {
    fn name(&self) -> &'static str;
    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic>;
}

// ---------------------------------------------------------------------------
// Rules
// ---------------------------------------------------------------------------

pub struct ValidYaml;

impl LintRule for ValidYaml {
    fn name(&self) -> &'static str {
        "valid-yaml"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if let Some(ref err) = ctx.yaml_error {
            vec![LintDiagnostic {
                severity: Severity::Error,
                file_path: ctx.file_path.clone(),
                rule: self.name(),
                message: format!("invalid YAML frontmatter: {err}"),
            }]
        } else {
            vec![]
        }
    }
}

pub struct NonEmptyBody;

impl LintRule for NonEmptyBody {
    fn name(&self) -> &'static str {
        "non-empty-body"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if ctx.body.trim().is_empty() {
            vec![LintDiagnostic {
                severity: Severity::Error,
                file_path: ctx.file_path.clone(),
                rule: self.name(),
                message: "body is empty".to_string(),
            }]
        } else {
            vec![]
        }
    }
}

pub struct HasDescription;

impl LintRule for HasDescription {
    fn name(&self) -> &'static str {
        "has-description"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if let Some(ref fm) = ctx.frontmatter {
            if fm.description.is_none() {
                return vec![LintDiagnostic {
                    severity: Severity::Warning,
                    file_path: ctx.file_path.clone(),
                    rule: self.name(),
                    message: "missing `description` field in frontmatter".to_string(),
                }];
            }
        }
        vec![]
    }
}

pub struct HasTags;

impl LintRule for HasTags {
    fn name(&self) -> &'static str {
        "has-tags"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if let Some(ref fm) = ctx.frontmatter {
            if fm.tags.is_empty() {
                return vec![LintDiagnostic {
                    severity: Severity::Warning,
                    file_path: ctx.file_path.clone(),
                    rule: self.name(),
                    message: "missing or empty `tags` field".to_string(),
                }];
            }
        }
        vec![]
    }
}

pub struct UnknownFields;

impl LintRule for UnknownFields {
    fn name(&self) -> &'static str {
        "unknown-fields"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        ctx.unknown_fields
            .iter()
            .map(|field| LintDiagnostic {
                severity: Severity::Warning,
                file_path: ctx.file_path.clone(),
                rule: self.name(),
                message: format!("unknown frontmatter field `{field}`"),
            })
            .collect()
    }
}

pub struct SkillHasGroup;

impl LintRule for SkillHasGroup {
    fn name(&self) -> &'static str {
        "skill-has-group"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if ctx.category != Category::Skill {
            return vec![];
        }
        if let Some(ref fm) = ctx.frontmatter {
            if fm.group.is_none() {
                return vec![LintDiagnostic {
                    severity: Severity::Warning,
                    file_path: ctx.file_path.clone(),
                    rule: self.name(),
                    message: "skill is missing `group` field".to_string(),
                }];
            }
        }
        vec![]
    }
}

pub struct PersonaHasLevel;

impl LintRule for PersonaHasLevel {
    fn name(&self) -> &'static str {
        "persona-has-level"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if ctx.category != Category::Persona {
            return vec![];
        }
        if let Some(ref fm) = ctx.frontmatter {
            if fm.level.is_none() {
                return vec![LintDiagnostic {
                    severity: Severity::Warning,
                    file_path: ctx.file_path.clone(),
                    rule: self.name(),
                    message: "persona is missing `level` field".to_string(),
                }];
            }
        }
        vec![]
    }
}

pub struct RootFileHasCategory;

impl LintRule for RootFileHasCategory {
    fn name(&self) -> &'static str {
        "root-file-has-category"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if !ctx.is_root_level {
            return vec![];
        }
        if let Some(ref fm) = ctx.frontmatter {
            match &fm.category {
                None => {
                    return vec![LintDiagnostic {
                        severity: Severity::Error,
                        file_path: ctx.file_path.clone(),
                        rule: self.name(),
                        message: "root-level file is missing `category` in frontmatter".to_string(),
                    }];
                }
                Some(cat_str) => {
                    if Category::from_name(cat_str).is_none() {
                        return vec![LintDiagnostic {
                            severity: Severity::Error,
                            file_path: ctx.file_path.clone(),
                            rule: self.name(),
                            message: format!("unknown category `{cat_str}` in frontmatter"),
                        }];
                    }
                }
            }
        }
        vec![]
    }
}

pub struct CategoryConflict;

impl LintRule for CategoryConflict {
    fn name(&self) -> &'static str {
        "category-conflict"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if ctx.is_root_level {
            return vec![];
        }
        if let Some(ref fm) = ctx.frontmatter {
            if let Some(ref cat_str) = fm.category {
                if let Some(fm_category) = Category::from_name(cat_str) {
                    if fm_category != ctx.category {
                        return vec![LintDiagnostic {
                            severity: Severity::Warning,
                            file_path: ctx.file_path.clone(),
                            rule: self.name(),
                            message: format!(
                                "frontmatter `category: {}` conflicts with subdirectory `{}`",
                                cat_str,
                                ctx.category.dir_name()
                            ),
                        }];
                    }
                }
            }
        }
        vec![]
    }
}

pub struct PersonaHasSkillGroups;

impl LintRule for PersonaHasSkillGroups {
    fn name(&self) -> &'static str {
        "persona-has-skill-groups"
    }

    fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        if ctx.category != Category::Persona {
            return vec![];
        }
        if let Some(ref fm) = ctx.frontmatter {
            if fm.skill_groups.is_empty() {
                return vec![LintDiagnostic {
                    severity: Severity::Warning,
                    file_path: ctx.file_path.clone(),
                    rule: self.name(),
                    message: "persona is missing `skill_groups` field".to_string(),
                }];
            }
        }
        vec![]
    }
}

// ---------------------------------------------------------------------------
// Linter runner
// ---------------------------------------------------------------------------

pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(ValidYaml),
                Box::new(NonEmptyBody),
                Box::new(HasDescription),
                Box::new(HasTags),
                Box::new(UnknownFields),
                Box::new(SkillHasGroup),
                Box::new(PersonaHasLevel),
                Box::new(PersonaHasSkillGroups),
                Box::new(RootFileHasCategory),
                Box::new(CategoryConflict),
            ],
        }
    }

    pub fn check(&self, ctx: &FragmentContext) -> Vec<LintDiagnostic> {
        self.rules.iter().flat_map(|r| r.check(ctx)).collect()
    }
}

// ---------------------------------------------------------------------------
// Context builder
// ---------------------------------------------------------------------------

fn build_context(
    raw_content: &str,
    file_path: &str,
    filename_stem: &str,
    category: Category,
    source: Source,
    is_root_level: bool,
) -> FragmentContext {
    match parse_frontmatter_strict(raw_content) {
        Ok((fm, unknown_fields, body)) => FragmentContext {
            file_path: file_path.to_string(),
            filename_stem: filename_stem.to_string(),
            category,
            source,
            raw_content: raw_content.to_string(),
            frontmatter: Some(fm),
            yaml_error: None,
            unknown_fields,
            body,
            is_root_level,
        },
        Err(err) => FragmentContext {
            file_path: file_path.to_string(),
            filename_stem: filename_stem.to_string(),
            category,
            source,
            raw_content: raw_content.to_string(),
            frontmatter: None,
            yaml_error: Some(err),
            unknown_fields: vec![],
            body: raw_content.to_string(),
            is_root_level,
        },
    }
}

// ---------------------------------------------------------------------------
// Fragment walking
// ---------------------------------------------------------------------------

fn walk_builtin(linter: &Linter, diagnostics: &mut Vec<LintDiagnostic>) {
    for category in Category::all() {
        if let Some(dir) = DEFAULTS_DIR.get_dir(category.dir_name()) {
            for file in dir.files() {
                let path = file.path();
                if path.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                let stem = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                let content = match file.contents_utf8() {
                    Some(c) => c,
                    None => continue,
                };
                let display_path = format!("defaults/{}/{}.md", category.dir_name(), stem);
                let ctx = build_context(
                    content,
                    &display_path,
                    stem,
                    *category,
                    Source::BuiltIn,
                    false,
                );
                diagnostics.extend(linter.check(&ctx));
            }
        }
    }
}

fn walk_fs_layer(
    linter: &Linter,
    base: &Path,
    source: Source,
    label: &str,
    diagnostics: &mut Vec<LintDiagnostic>,
) {
    for category in Category::all() {
        let dir = base.join(category.dir_name());
        if !dir.is_dir() {
            continue;
        }
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let stem = match path.file_stem().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };
            let content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let display_path = format!("{}/{}/{}.md", label, category.dir_name(), stem);
            let ctx = build_context(&content, &display_path, stem, *category, source, false);
            diagnostics.extend(linter.check(&ctx));
        }
    }

    // Root-level files
    let root_entries = match std::fs::read_dir(base) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in root_entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // Determine category from frontmatter for context; default to Skill as placeholder
        let category = match parse_frontmatter_strict(&content) {
            Ok((fm, _, _)) => fm
                .category
                .as_deref()
                .and_then(Category::from_name)
                .unwrap_or(Category::Skill),
            Err(_) => Category::Skill,
        };
        let display_path = format!("{}/{}.md", label, stem);
        let ctx = build_context(&content, &display_path, stem, category, source, true);
        diagnostics.extend(linter.check(&ctx));
    }
}

// ---------------------------------------------------------------------------
// Reporter
// ---------------------------------------------------------------------------

fn report(diagnostics: &[LintDiagnostic], show_warnings: bool) -> (usize, usize) {
    let mut errors = 0usize;
    let mut warnings = 0usize;

    for d in diagnostics {
        match d.severity {
            Severity::Error => errors += 1,
            Severity::Warning => {
                warnings += 1;
                if !show_warnings {
                    continue;
                }
            }
        }
        eprintln!(
            "  {} [{}] {}: {}",
            d.severity, d.rule, d.file_path, d.message
        );
    }

    (errors, warnings)
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn run(show_warnings: bool) -> i32 {
    let linter = Linter::new();
    let mut diagnostics = Vec::new();

    // Built-in fragments
    walk_builtin(&linter, &mut diagnostics);

    // Global config dir
    if let Some(global) = dirs::config_dir().map(|d| d.join("mimic")) {
        if global.is_dir() {
            walk_fs_layer(
                &linter,
                &global,
                Source::Global,
                "~/.mimic",
                &mut diagnostics,
            );
        }
    }

    // Project dir
    if let Some(project) = find_project_dir() {
        walk_fs_layer(
            &linter,
            &project,
            Source::Project,
            ".mimic",
            &mut diagnostics,
        );
    }

    let (errors, warnings) = report(&diagnostics, show_warnings);

    if errors == 0 && warnings == 0 {
        eprintln!("All fragments OK.");
    } else if errors == 0 {
        if show_warnings {
            eprintln!("\n{warnings} warning(s), 0 errors.");
        } else {
            eprintln!("All fragments OK ({warnings} warning(s) hidden, use --warnings to show).");
        }
    } else {
        eprintln!("\n{errors} error(s), {warnings} warning(s).");
    }

    if errors > 0 {
        1
    } else {
        0
    }
}

fn find_project_dir() -> Option<std::path::PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();
    loop {
        let candidate = dir.join(".mimic");
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ctx(content: &str, category: Category) -> FragmentContext {
        build_context(content, "test.md", "test", category, Source::BuiltIn, false)
    }

    #[test]
    fn valid_yaml_passes() {
        let ctx = make_ctx(
            "---\ndescription: Hello\ntags: [a]\n---\nBody.",
            Category::Skill,
        );
        let diags = ValidYaml.check(&ctx);
        assert!(diags.is_empty());
    }

    #[test]
    fn valid_yaml_fails_on_bad_yaml() {
        let ctx = FragmentContext {
            file_path: "test.md".to_string(),
            filename_stem: "test".to_string(),
            category: Category::Skill,
            source: Source::BuiltIn,
            raw_content: String::new(),
            frontmatter: None,
            yaml_error: Some("bad yaml".to_string()),
            unknown_fields: vec![],
            body: String::new(),
            is_root_level: false,
        };
        let diags = ValidYaml.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
    }

    #[test]
    fn non_empty_body_passes() {
        let ctx = make_ctx("---\ndescription: x\n---\nSome body.", Category::Skill);
        let diags = NonEmptyBody.check(&ctx);
        assert!(diags.is_empty());
    }

    #[test]
    fn non_empty_body_fails_on_empty() {
        let ctx = make_ctx("---\ndescription: x\n---\n  \n", Category::Skill);
        let diags = NonEmptyBody.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
    }

    #[test]
    fn has_description_warns_when_missing() {
        let ctx = make_ctx("---\ntags: [a]\n---\nBody.", Category::Skill);
        let diags = HasDescription.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn has_tags_warns_when_empty() {
        let ctx = make_ctx("---\ndescription: x\n---\nBody.", Category::Skill);
        let diags = HasTags.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn unknown_fields_detects_typos() {
        let ctx = make_ctx(
            "---\ndescrption: typo\ntags: [a]\n---\nBody.",
            Category::Skill,
        );
        let diags = UnknownFields.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("descrption"));
    }

    #[test]
    fn skill_has_group_warns_when_missing() {
        let ctx = make_ctx(
            "---\ndescription: x\ntags: [a]\n---\nBody.",
            Category::Skill,
        );
        let diags = SkillHasGroup.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn skill_has_group_skips_non_skills() {
        let ctx = make_ctx("---\ndescription: x\n---\nBody.", Category::Persona);
        let diags = SkillHasGroup.check(&ctx);
        assert!(diags.is_empty());
    }

    #[test]
    fn persona_has_level_warns_when_missing() {
        let ctx = make_ctx(
            "---\ndescription: x\ntags: [a]\n---\nBody.",
            Category::Persona,
        );
        let diags = PersonaHasLevel.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn persona_has_skill_groups_warns_when_missing() {
        let ctx = make_ctx(
            "---\ndescription: x\ntags: [a]\n---\nBody.",
            Category::Persona,
        );
        let diags = PersonaHasSkillGroups.check(&ctx);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn linter_runs_all_rules() {
        let linter = Linter::new();
        // A well-formed skill should only get warnings for missing group
        let ctx = make_ctx(
            "---\ndescription: x\ntags: [a]\n---\nBody.",
            Category::Skill,
        );
        let diags = linter.check(&ctx);
        // Should have warning for missing group, nothing else
        assert!(diags.iter().all(|d| d.severity == Severity::Warning));
        assert!(diags.iter().any(|d| d.rule == "skill-has-group"));
    }

    #[test]
    fn linter_catches_errors_and_warnings() {
        let linter = Linter::new();
        // Bad yaml and empty body
        let ctx = FragmentContext {
            file_path: "test.md".to_string(),
            filename_stem: "test".to_string(),
            category: Category::Skill,
            source: Source::BuiltIn,
            raw_content: String::new(),
            frontmatter: None,
            yaml_error: Some("parse error".to_string()),
            unknown_fields: vec![],
            body: String::new(),
            is_root_level: false,
        };
        let diags = linter.check(&ctx);
        let errors: Vec<_> = diags
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(errors.len() >= 2); // valid-yaml + non-empty-body
    }

    #[test]
    fn builtin_fragments_lint_clean() {
        let linter = Linter::new();
        let mut diagnostics = Vec::new();
        walk_builtin(&linter, &mut diagnostics);
        let errors: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "built-in fragments should have no errors: {errors:?}"
        );
    }
}
