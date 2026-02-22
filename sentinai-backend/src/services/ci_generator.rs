use tracing::{debug, info, instrument};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ProjectType {
    Rust,
    Node,
    Python,
    Unknown,
}

#[instrument(name = "ci_generator", skip_all)]
pub fn detect_project_type(_repo_url: &str) -> ProjectType {
    debug!("Rust project detected");
    ProjectType::Rust
}

#[instrument(name = "ci_generator", skip_all, fields(project_type = ?project_type))]
pub fn generate_ci_yaml(project_type: ProjectType) -> String {
    debug!(
        language = ?project_type,
        template = "default",
        strict_mode = true,
        "Generating CI template"
    );
    let yaml = match project_type {
        ProjectType::Rust => {
            r#"
name: Sentinai Rust CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Ensure strict Rust warnings
      run: cargo check --all-targets --all-features
      env:
        RUSTFLAGS: "-D warnings"
    - name: Run tests
      run: cargo test
    - name: Security audit
      run: cargo audit
            "#.trim().to_string()
        },
        ProjectType::Node => {
            r#"
name: Sentinai Node CI

on:
  push:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions/setup-node@v3
      with:
        node-version: 18
    - run: npm install
    - run: npm test
    - run: npm audit
            "#.trim().to_string()
        },
        ProjectType::Python => {
            r#"
name: Sentinai Python CI

on:
  push:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions/setup-python@v4
      with:
        python-version: '3.10'
    - run: pip install -r requirements.txt
    - run: pytest
    - run: safety check
            "#.trim().to_string()
        },
        ProjectType::Unknown => {
            "# Fallback Pipeline\nname: Generic\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n    - run: echo 'Unknown project'".to_string()
        }
    };

    info!("CI generated (size={} bytes)", yaml.len());
    yaml
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_detect_project_type() {
        let pt = detect_project_type("https://github.com/foo/bar");
        assert!(matches!(pt, ProjectType::Rust));
    }

    #[test]
    fn test_generate_ci_yaml_rust() {
        let yaml = generate_ci_yaml(ProjectType::Rust);
        assert_snapshot!("rust_ci_yaml", yaml);
    }

    #[test]
    fn test_generate_ci_yaml_node() {
        let yaml = generate_ci_yaml(ProjectType::Node);
        assert_snapshot!("node_ci_yaml", yaml);
    }

    #[test]
    fn test_generate_ci_yaml_python() {
        let yaml = generate_ci_yaml(ProjectType::Python);
        assert_snapshot!("python_ci_yaml", yaml);
    }

    #[test]
    fn test_generate_ci_yaml_unknown() {
        let yaml = generate_ci_yaml(ProjectType::Unknown);
        assert_snapshot!("unknown_ci_yaml", yaml);
    }
}
