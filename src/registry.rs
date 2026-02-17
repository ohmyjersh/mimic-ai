use crate::fragment::{Category, Fragment, Frontmatter, Source};
use include_dir::{include_dir, Dir};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

static DEFAULTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/defaults");

pub struct Registry {
    fragments: HashMap<Category, HashMap<String, Fragment>>,
    // Pre-computed indexes
    cached_tags: Vec<String>,
    cached_groups: Vec<String>,
    cached_names: HashMap<Category, Vec<String>>,
    // Watched directories for hot reload
    project_dir: Option<PathBuf>,
    global_dir: Option<PathBuf>,
}

impl Registry {
    pub fn new(project_dir: Option<PathBuf>) -> Registry {
        let mut fragments: HashMap<Category, HashMap<String, Fragment>> = HashMap::new();

        // Load in reverse priority order (later inserts don't override)
        // Built-in defaults (lowest priority)
        load_builtin(&mut fragments);

        let global_dir = global_config_dir();

        // Ensure directories exist on first run
        if let Some(ref global_dir) = global_dir {
            ensure_dir_exists(global_dir);
        }
        if let Some(ref proj) = project_dir {
            ensure_dir_exists(proj);
        }

        // Global config
        if let Some(ref global_dir) = global_dir {
            load_from_fs(&mut fragments, global_dir, Source::Global);
        }

        // Project-local (highest priority)
        if let Some(ref proj) = project_dir {
            load_from_fs(&mut fragments, proj, Source::Project);
        }

        let (cached_tags, cached_groups, cached_names) = build_indexes(&fragments);

        Registry {
            fragments,
            cached_tags,
            cached_groups,
            cached_names,
            project_dir,
            global_dir,
        }
    }

    pub fn get(&self, category: Category, name: &str) -> Option<&Fragment> {
        self.fragments.get(&category).and_then(|m| m.get(name))
    }

    pub fn list(
        &self,
        category: Option<Category>,
        tag: Option<&str>,
        group: Option<&str>,
    ) -> Vec<&Fragment> {
        let iter: Box<dyn Iterator<Item = &Fragment>> = if let Some(cat) = category {
            if let Some(map) = self.fragments.get(&cat) {
                Box::new(map.values())
            } else {
                Box::new(std::iter::empty())
            }
        } else {
            Box::new(self.fragments.values().flat_map(|m| m.values()))
        };

        let mut results: Vec<&Fragment> = iter
            .filter(|f| tag.is_none_or(|t| f.tags.iter().any(|ft| ft == t)))
            .filter(|f| group.is_none_or(|g| f.group.as_deref() == Some(g)))
            .collect();
        results.sort_by(|a, b| {
            a.category
                .dir_name()
                .cmp(b.category.dir_name())
                .then(a.name.cmp(&b.name))
        });
        results
    }

    pub fn all_groups(&self) -> &[String] {
        &self.cached_groups
    }

    pub fn all_tags(&self) -> &[String] {
        &self.cached_tags
    }

    pub fn names_for_category(&self, category: Category) -> &[String] {
        static EMPTY: Vec<String> = Vec::new();
        self.cached_names.get(&category).unwrap_or(&EMPTY)
    }

    /// Returns the directories that should be watched for hot reload.
    pub fn watched_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(ref d) = self.project_dir {
            dirs.push(d.clone());
        }
        if let Some(ref d) = self.global_dir {
            dirs.push(d.clone());
        }
        dirs
    }
}

fn build_indexes(
    fragments: &HashMap<Category, HashMap<String, Fragment>>,
) -> (Vec<String>, Vec<String>, HashMap<Category, Vec<String>>) {
    let mut tags: BTreeSet<String> = BTreeSet::new();
    let mut groups: BTreeSet<String> = BTreeSet::new();

    for map in fragments.values() {
        for f in map.values() {
            for t in &f.tags {
                tags.insert(t.clone());
            }
            if let Some(ref g) = f.group {
                groups.insert(g.clone());
            }
        }
    }

    let mut names: HashMap<Category, Vec<String>> = HashMap::new();
    for cat in Category::all() {
        if let Some(map) = fragments.get(cat) {
            let mut cat_names: Vec<String> = map.keys().cloned().collect();
            cat_names.sort();
            names.insert(*cat, cat_names);
        }
    }

    (
        tags.into_iter().collect(),
        groups.into_iter().collect(),
        names,
    )
}

fn global_config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|d| d.join(".mimic"))
}

fn ensure_dir_exists(base: &Path) {
    for category in Category::all() {
        let dir = base.join(category.dir_name());
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("mimic: warning: failed to create {}: {}", dir.display(), e);
        }
    }
}

fn load_from_fs(
    fragments: &mut HashMap<Category, HashMap<String, Fragment>>,
    base: &Path,
    source: Source,
) {
    for category in Category::all() {
        let dir = base.join(category.dir_name());
        if !dir.is_dir() {
            continue;
        }
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                eprintln!("mimic: warning: failed to read {}: {}", dir.display(), e);
                continue;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            if let Some(frag) = Fragment::from_file(&path, *category, source) {
                // First-match-wins: higher priority sources are loaded last,
                // so they override.
                fragments
                    .entry(*category)
                    .or_default()
                    .insert(frag.name.clone(), frag);
            }
        }
    }

    // Root-level files: require a `category` field in frontmatter
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
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let fm = parse_root_frontmatter(&content);
        let category_str = match fm.category {
            Some(ref c) => c.as_str(),
            None => {
                eprintln!(
                    "mimic: warning: skipping {}: missing `category` in frontmatter",
                    path.display()
                );
                continue;
            }
        };
        let category = match Category::from_name(category_str) {
            Some(c) => c,
            None => {
                eprintln!(
                    "mimic: warning: skipping {}: unknown category `{}`",
                    path.display(),
                    category_str
                );
                continue;
            }
        };
        if let Some(frag) = Fragment::from_file(&path, category, source) {
            fragments
                .entry(category)
                .or_default()
                .insert(frag.name.clone(), frag);
        }
    }
}

/// Minimal frontmatter parse to extract the `category` field from root-level files.
fn parse_root_frontmatter(content: &str) -> Frontmatter {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Frontmatter::default();
    }
    let after_first_fence = &trimmed[3..];
    if let Some(end) = after_first_fence.find("\n---") {
        let yaml = &after_first_fence[..end];
        serde_yaml::from_str(yaml).unwrap_or_default()
    } else {
        Frontmatter::default()
    }
}

fn load_builtin(fragments: &mut HashMap<Category, HashMap<String, Fragment>>) {
    for category in Category::all() {
        if let Some(dir) = DEFAULTS_DIR.get_dir(category.dir_name()) {
            for file in dir.files() {
                let path = file.path();
                if path.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                let name = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                let content = match file.contents_utf8() {
                    Some(c) => c,
                    None => continue,
                };
                let frag = Fragment::parse(content, name, *category, Source::BuiltIn);
                // Don't override — built-in is lowest priority
                fragments
                    .entry(*category)
                    .or_default()
                    .entry(frag.name.clone())
                    .or_insert(frag);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_global_dir_creates_directory_and_subdirs() {
        let tmp = tempfile::tempdir().unwrap();
        let global = tmp.path().join("mimic");
        assert!(!global.exists());

        ensure_dir_exists(&global);

        assert!(global.is_dir());
        for cat in Category::all() {
            assert!(
                global.join(cat.dir_name()).is_dir(),
                "Should create {} subdirectory",
                cat.dir_name()
            );
        }
    }

    #[test]
    fn ensure_global_dir_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let global = tmp.path().join("mimic");
        ensure_dir_exists(&global);
        ensure_dir_exists(&global); // should not error
        assert!(global.is_dir());
    }

    #[test]
    fn builtin_defaults_load() {
        let registry = Registry::new(None);
        // Should have at least the built-in personas
        let personas = registry.names_for_category(Category::Persona);
        assert!(!personas.is_empty(), "Should have built-in personas");
        assert!(
            registry
                .get(Category::Persona, "backend-engineer")
                .is_some(),
            "Should have backend-engineer persona"
        );
    }

    #[test]
    fn list_with_category_filter() {
        let registry = Registry::new(None);
        let skills = registry.list(Some(Category::Skill), None, None);
        assert!(skills.iter().all(|f| f.category == Category::Skill));
    }

    #[test]
    fn list_with_group_filter() {
        let registry = Registry::new(None);
        let backend_skills = registry.list(Some(Category::Skill), None, Some("backend"));
        assert!(
            !backend_skills.is_empty(),
            "Should have backend-grouped skills"
        );
        assert!(backend_skills
            .iter()
            .all(|f| f.group.as_deref() == Some("backend")));
    }

    #[test]
    fn get_nonexistent_fragment() {
        let registry = Registry::new(None);
        assert!(registry.get(Category::Persona, "nonexistent").is_none());
    }

    #[test]
    fn list_with_tag_filter() {
        let registry = Registry::new(None);
        let tagged = registry.list(None, Some("security"), None);
        assert!(
            tagged
                .iter()
                .all(|f| f.tags.contains(&"security".to_string())),
            "All fragments should have the 'security' tag"
        );
    }

    #[test]
    fn list_with_nonexistent_group() {
        let registry = Registry::new(None);
        let results = registry.list(None, None, Some("nonexistent-group"));
        assert!(results.is_empty());
    }

    #[test]
    fn names_for_category_sorted() {
        let registry = Registry::new(None);
        let names = registry.names_for_category(Category::Persona);
        let mut sorted = names.to_vec();
        sorted.sort();
        assert_eq!(
            names,
            &sorted[..],
            "names_for_category should return sorted names"
        );
    }

    #[test]
    fn all_tags_deduplicated() {
        let registry = Registry::new(None);
        let tags = registry.all_tags();
        let unique: std::collections::BTreeSet<&String> = tags.iter().collect();
        assert_eq!(
            tags.len(),
            unique.len(),
            "all_tags should have no duplicates"
        );
    }

    #[test]
    fn all_groups_deduplicated() {
        let registry = Registry::new(None);
        let groups = registry.all_groups();
        let unique: std::collections::BTreeSet<&String> = groups.iter().collect();
        assert_eq!(
            groups.len(),
            unique.len(),
            "all_groups should have no duplicates"
        );
    }

    #[test]
    fn load_from_fs_skips_non_md() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        std::fs::create_dir_all(&skills_dir).unwrap();
        std::fs::write(
            skills_dir.join("valid.md"),
            "---\ndescription: A skill\ntags: [test]\ngroup: backend\n---\nSkill body.",
        )
        .unwrap();
        std::fs::write(skills_dir.join("ignored.txt"), "Not a markdown file.").unwrap();
        std::fs::write(skills_dir.join("ignored.yaml"), "also: ignored").unwrap();

        let registry = Registry::new(Some(tmp.path().to_path_buf()));
        // The .txt and .yaml files should be ignored; only valid.md should be loaded
        let result = registry.get(Category::Skill, "valid");
        assert!(result.is_some(), "Should load .md files");
        assert!(
            registry.get(Category::Skill, "ignored").is_none(),
            "Should not load non-.md files"
        );
    }

    #[test]
    fn project_local_overrides_builtin() {
        let tmp = tempfile::tempdir().unwrap();
        let personas_dir = tmp.path().join("personas");
        std::fs::create_dir_all(&personas_dir).unwrap();
        std::fs::write(
            personas_dir.join("backend-engineer.md"),
            "---\ndescription: Custom backend engineer\n---\nCustom body.",
        )
        .unwrap();

        let registry = Registry::new(Some(tmp.path().to_path_buf()));
        let frag = registry.get(Category::Persona, "backend-engineer").unwrap();
        assert_eq!(frag.description, "Custom backend engineer");
        assert_eq!(frag.source, Source::Project);
    }

    #[test]
    fn root_level_file_with_category_loads() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("react.md"),
            "---\ncategory: skill\ndescription: React expertise\ntags: [frontend]\ngroup: frontend\n---\nYou are an expert in React.",
        )
        .unwrap();

        let mut fragments: HashMap<Category, HashMap<String, Fragment>> = HashMap::new();
        load_from_fs(&mut fragments, tmp.path(), Source::Project);

        let frag = fragments
            .get(&Category::Skill)
            .and_then(|m| m.get("react"))
            .expect("Should load root-level file with category: skill");
        assert_eq!(frag.name, "react");
        assert_eq!(frag.category, Category::Skill);
        assert_eq!(frag.description, "React expertise");
    }

    #[test]
    fn root_level_file_without_category_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("orphan.md"),
            "---\ndescription: No category\n---\nBody here.",
        )
        .unwrap();

        let mut fragments: HashMap<Category, HashMap<String, Fragment>> = HashMap::new();
        load_from_fs(&mut fragments, tmp.path(), Source::Project);

        // Should not appear in any category
        for cat in Category::all() {
            assert!(
                fragments.get(cat).and_then(|m| m.get("orphan")).is_none(),
                "Root file without category should be skipped"
            );
        }
    }

    #[test]
    fn root_level_file_overrides_builtin() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("backend-engineer.md"),
            "---\ncategory: persona\ndescription: Custom backend from root\n---\nCustom root body.",
        )
        .unwrap();

        let registry = Registry::new(Some(tmp.path().to_path_buf()));
        let frag = registry.get(Category::Persona, "backend-engineer").unwrap();
        assert_eq!(frag.description, "Custom backend from root");
        assert_eq!(frag.source, Source::Project);
    }

    #[test]
    fn subdirectory_file_still_works() {
        let tmp = tempfile::tempdir().unwrap();
        let tones_dir = tmp.path().join("tones");
        std::fs::create_dir_all(&tones_dir).unwrap();
        std::fs::write(
            tones_dir.join("friendly.md"),
            "---\ndescription: Friendly tone\ntags: [tone]\n---\nBe friendly.",
        )
        .unwrap();

        let mut fragments: HashMap<Category, HashMap<String, Fragment>> = HashMap::new();
        load_from_fs(&mut fragments, tmp.path(), Source::Project);

        let frag = fragments
            .get(&Category::Tone)
            .and_then(|m| m.get("friendly"))
            .expect("Subdirectory file should still load");
        assert_eq!(frag.description, "Friendly tone");
    }
}
