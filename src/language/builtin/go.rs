use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct GoLanguage;

impl Language for GoLanguage {
    fn name(&self) -> &str {
        "go"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("go.mod").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
