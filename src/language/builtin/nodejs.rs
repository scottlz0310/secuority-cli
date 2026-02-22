use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct NodejsLanguage;

impl Language for NodejsLanguage {
    fn name(&self) -> &str {
        "nodejs"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("package.json").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
