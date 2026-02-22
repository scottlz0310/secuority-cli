use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct JavaLanguage;

impl Language for JavaLanguage {
    fn name(&self) -> &str {
        "java"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("pom.xml").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
