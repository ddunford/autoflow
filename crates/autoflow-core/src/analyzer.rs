use autoflow_data::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Codebase analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseAnalysis {
    pub project_root: String,
    pub tech_stack: TechStack,
    pub frameworks: Vec<Framework>,
    pub structure: ProjectStructure,
    pub integration_points: Vec<IntegrationPoint>,
}

impl CodebaseAnalysis {
    /// Save analysis to INTEGRATION_GUIDE.md
    pub fn save(&self, path: &str) -> Result<()> {
        let content = self.to_markdown();
        fs::write(path, content)?;
        Ok(())
    }

    /// Generate markdown documentation
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Integration Guide\n\n");
        md.push_str(&format!("**Generated**: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        md.push_str("---\n\n");

        // Tech Stack
        md.push_str("## Tech Stack\n\n");
        md.push_str(&format!("- **Language**: {}\n", self.tech_stack.language));
        if let Some(version) = &self.tech_stack.version {
            md.push_str(&format!("- **Version**: {}\n", version));
        }
        md.push_str(&format!("- **Package Manager**: {}\n", self.tech_stack.package_manager));
        md.push_str("\n");

        // Frameworks
        if !self.frameworks.is_empty() {
            md.push_str("## Frameworks\n\n");
            for framework in &self.frameworks {
                md.push_str(&format!("### {}\n\n", framework.name));
                md.push_str(&format!("- **Type**: {}\n", framework.framework_type));
                if let Some(version) = &framework.version {
                    md.push_str(&format!("- **Version**: {}\n", version));
                }
                md.push_str("\n");
            }
        }

        // Project Structure
        md.push_str("## Project Structure\n\n");
        md.push_str("```\n");
        if let Some(src) = &self.structure.source_dir {
            md.push_str(&format!("{}/ - Source code\n", src));
        }
        if let Some(tests) = &self.structure.test_dir {
            md.push_str(&format!("{}/ - Tests\n", tests));
        }
        if let Some(config) = &self.structure.config_dir {
            md.push_str(&format!("{}/ - Configuration\n", config));
        }
        md.push_str("```\n\n");

        // Integration Points
        if !self.integration_points.is_empty() {
            md.push_str("## Integration Points\n\n");
            for point in &self.integration_points {
                md.push_str(&format!("### {} ({})\n\n", point.name, point.point_type));
                if !point.files.is_empty() {
                    md.push_str("**Files**:\n");
                    for file in &point.files {
                        md.push_str(&format!("- `{}`\n", file));
                    }
                }
                md.push_str("\n");
            }
        }

        md
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStack {
    pub language: String,
    pub version: Option<String>,
    pub package_manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Framework {
    pub name: String,
    pub framework_type: String, // frontend, backend, testing
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub source_dir: Option<String>,
    pub test_dir: Option<String>,
    pub config_dir: Option<String>,
    pub entry_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoint {
    pub name: String,
    pub point_type: String, // api, model, component, service
    pub files: Vec<String>,
    pub patterns: Vec<String>,
}

/// Codebase analyzer
pub struct CodebaseAnalyzer {
    root: PathBuf,
}

impl CodebaseAnalyzer {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Analyze the codebase
    pub fn analyze(&self) -> Result<CodebaseAnalysis> {
        let tech_stack = self.detect_tech_stack();
        let frameworks = self.detect_frameworks();
        let structure = self.analyze_structure();
        let integration_points = self.find_integration_points();

        Ok(CodebaseAnalysis {
            project_root: self.root.to_string_lossy().to_string(),
            tech_stack,
            frameworks,
            structure,
            integration_points,
        })
    }

    /// Detect tech stack (language, package manager)
    fn detect_tech_stack(&self) -> TechStack {
        // Check for Node.js
        if self.root.join("package.json").exists() {
            let version = self.read_package_json_field("version");
            return TechStack {
                language: "JavaScript/TypeScript".to_string(),
                version,
                package_manager: if self.root.join("pnpm-lock.yaml").exists() {
                    "pnpm".to_string()
                } else if self.root.join("yarn.lock").exists() {
                    "yarn".to_string()
                } else {
                    "npm".to_string()
                },
            };
        }

        // Check for PHP/Laravel
        if self.root.join("composer.json").exists() {
            return TechStack {
                language: "PHP".to_string(),
                version: None,
                package_manager: "composer".to_string(),
            };
        }

        // Check for Rust
        if self.root.join("Cargo.toml").exists() {
            return TechStack {
                language: "Rust".to_string(),
                version: None,
                package_manager: "cargo".to_string(),
            };
        }

        // Check for Python
        if self.root.join("requirements.txt").exists() || self.root.join("pyproject.toml").exists() {
            return TechStack {
                language: "Python".to_string(),
                version: None,
                package_manager: "pip".to_string(),
            };
        }

        // Check for Go
        if self.root.join("go.mod").exists() {
            return TechStack {
                language: "Go".to_string(),
                version: None,
                package_manager: "go".to_string(),
            };
        }

        TechStack {
            language: "Unknown".to_string(),
            version: None,
            package_manager: "Unknown".to_string(),
        }
    }

    /// Detect frameworks
    fn detect_frameworks(&self) -> Vec<Framework> {
        let mut frameworks = Vec::new();

        // React
        if self.has_package_dependency("react") {
            frameworks.push(Framework {
                name: "React".to_string(),
                framework_type: "frontend".to_string(),
                version: self.get_package_version("react"),
            });
        }

        // Vue
        if self.has_package_dependency("vue") {
            frameworks.push(Framework {
                name: "Vue".to_string(),
                framework_type: "frontend".to_string(),
                version: self.get_package_version("vue"),
            });
        }

        // Laravel (check for artisan)
        if self.root.join("artisan").exists() {
            frameworks.push(Framework {
                name: "Laravel".to_string(),
                framework_type: "backend".to_string(),
                version: None,
            });
        }

        // Express
        if self.has_package_dependency("express") {
            frameworks.push(Framework {
                name: "Express".to_string(),
                framework_type: "backend".to_string(),
                version: self.get_package_version("express"),
            });
        }

        // Vite
        if self.has_package_dependency("vite") {
            frameworks.push(Framework {
                name: "Vite".to_string(),
                framework_type: "build-tool".to_string(),
                version: self.get_package_version("vite"),
            });
        }

        // Playwright
        if self.has_package_dependency("@playwright/test") {
            frameworks.push(Framework {
                name: "Playwright".to_string(),
                framework_type: "testing".to_string(),
                version: self.get_package_version("@playwright/test"),
            });
        }

        frameworks
    }

    /// Analyze project structure
    fn analyze_structure(&self) -> ProjectStructure {
        let mut source_dir = None;
        let mut test_dir = None;
        let mut config_dir = None;
        let mut entry_points = Vec::new();

        // Common source directories
        for dir in &["src", "app", "lib"] {
            if self.root.join(dir).is_dir() {
                source_dir = Some(dir.to_string());
                break;
            }
        }

        // Common test directories
        for dir in &["tests", "test", "__tests__", "spec"] {
            if self.root.join(dir).is_dir() {
                test_dir = Some(dir.to_string());
                break;
            }
        }

        // Common config directories
        for dir in &["config", "conf", ".config"] {
            if self.root.join(dir).is_dir() {
                config_dir = Some(dir.to_string());
                break;
            }
        }

        // Entry points
        for file in &["index.js", "index.ts", "main.js", "main.ts", "app.js", "server.js"] {
            if self.root.join(file).exists() {
                entry_points.push(file.to_string());
            }
        }

        ProjectStructure {
            source_dir,
            test_dir,
            config_dir,
            entry_points,
        }
    }

    /// Find integration points (APIs, models, components)
    fn find_integration_points(&self) -> Vec<IntegrationPoint> {
        let mut points = Vec::new();

        // API endpoints (Laravel routes)
        if self.root.join("routes/api.php").exists() {
            points.push(IntegrationPoint {
                name: "API Routes".to_string(),
                point_type: "api".to_string(),
                files: vec!["routes/api.php".to_string()],
                patterns: vec!["Route::".to_string()],
            });
        }

        // Models (Laravel)
        if self.root.join("app/Models").is_dir() {
            let models = self.find_files_in_dir("app/Models", "php");
            points.push(IntegrationPoint {
                name: "Models".to_string(),
                point_type: "model".to_string(),
                files: models,
                patterns: vec!["class".to_string(), "extends Model".to_string()],
            });
        }

        // React components
        if let Some(src_dir) = self.find_components_dir() {
            let components = self.find_files_in_dir(&src_dir, "tsx");
            if !components.is_empty() {
                points.push(IntegrationPoint {
                    name: "Components".to_string(),
                    point_type: "component".to_string(),
                    files: components,
                    patterns: vec!["export".to_string(), "function".to_string()],
                });
            }
        }

        points
    }

    // Helper methods

    fn has_package_dependency(&self, package: &str) -> bool {
        if let Ok(content) = fs::read_to_string(self.root.join("package.json")) {
            content.contains(&format!("\"{}\"", package))
        } else {
            false
        }
    }

    fn get_package_version(&self, package: &str) -> Option<String> {
        if let Ok(content) = fs::read_to_string(self.root.join("package.json")) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(deps) = json.get("dependencies") {
                    if let Some(version) = deps.get(package) {
                        return version.as_str().map(String::from);
                    }
                }
            }
        }
        None
    }

    fn read_package_json_field(&self, field: &str) -> Option<String> {
        if let Ok(content) = fs::read_to_string(self.root.join("package.json")) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                return json.get(field).and_then(|v| v.as_str()).map(String::from);
            }
        }
        None
    }

    fn find_components_dir(&self) -> Option<String> {
        for dir in &["src/components", "app/components", "components"] {
            if self.root.join(dir).is_dir() {
                return Some(dir.to_string());
            }
        }
        None
    }

    fn find_files_in_dir(&self, dir: &str, extension: &str) -> Vec<String> {
        let mut files = Vec::new();
        let full_path = self.root.join(dir);

        if full_path.exists() {
            for entry in WalkDir::new(full_path).max_depth(3) {
                if let Ok(entry) = entry {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some(extension) {
                        if let Ok(relative) = entry.path().strip_prefix(&self.root) {
                            files.push(relative.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        files
    }
}
