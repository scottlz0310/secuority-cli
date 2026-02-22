use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct RustLanguage;

impl Language for RustLanguage {
    fn name(&self) -> &str {
        "rust"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("Cargo.toml").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
