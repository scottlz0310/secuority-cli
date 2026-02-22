use std::sync::Arc;
use super::Language;

/// Registry holding all registered language plugins.
pub struct LanguageRegistry {
    languages: Vec<Arc<dyn Language>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self { languages: Vec::new() }
    }

    pub fn register(&mut self, lang: Arc<dyn Language>) {
        self.languages.push(lang);
    }

    pub fn detect_all(&self, project_path: &std::path::Path) -> Vec<Arc<dyn Language>> {
        self.languages.iter().filter(|l| l.detect(project_path)).cloned().collect()
    }
}
