use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct PythonLanguage;

impl Language for PythonLanguage {
    fn name(&self) -> &str {
        "python"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("pyproject.toml").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
