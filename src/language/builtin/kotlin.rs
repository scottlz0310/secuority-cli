use std::path::Path;
use crate::language::{Language, TemplateRef};

pub struct KotlinLanguage;

impl Language for KotlinLanguage {
    fn name(&self) -> &str {
        "kotlin"
    }

    fn detect(&self, project_path: &Path) -> bool {
        // Kotlin projects use Gradle (build.gradle.kts or build.gradle)
        project_path.join("build.gradle.kts").exists()
            || project_path.join("build.gradle").exists()
    }

    fn templates(&self) -> Vec<TemplateRef> {
        // TODO: populate with actual template refs
        vec![]
    }
}
