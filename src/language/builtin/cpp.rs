use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct CppLanguage;

impl Language for CppLanguage {
    fn name(&self) -> &str {
        "cpp"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("CMakeLists.txt").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
