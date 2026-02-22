use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct CsharpLanguage;

impl Language for CsharpLanguage {
    fn name(&self) -> &str {
        "csharp"
    }

    fn detect(&self, project_path: &Path) -> bool {
        project_path.join("*.csproj").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
