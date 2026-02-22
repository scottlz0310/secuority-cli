use std::path::Path;

/// Trait representing a supported language plugin.
/// Each built-in language (Python, Rust, Go, etc.) implements this trait.
/// User-defined languages are handled via `YamlLanguage`.
pub trait Language: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, project_path: &Path) -> bool;
    fn templates(&self) -> Vec<TemplateRef>;
}

/// Reference to a template file inside secuority-templates repo.
pub struct TemplateRef {
    /// e.g. "python/base/workflows/security-check.yml"
    pub path: String,
    /// Destination relative to project root, e.g. ".github/workflows/security-check.yml"
    pub dest: String,
}

pub mod registry;
pub mod builtin;
