use std::fs;
use std::path::{Path, PathBuf};

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn find_forbidden_dependencies(paths: &[PathBuf], forbidden: &[String]) -> Vec<String> {
    let mut hits = Vec::new();
    for path in paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("failed to read {}: {}", path.display(), err));
        for (line_no, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            for needle in forbidden {
                if trimmed.contains(needle) {
                    hits.push(format!(
                        "{}:{}: {}",
                        path.display(),
                        line_no + 1,
                        trimmed
                    ));
                }
            }
        }
    }
    hits
}

#[test]
fn cli_boundaries_are_enforced() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut rotappo_files = Vec::new();
    for dir in ["src", "crates", "tests"] {
        let path = root.join(dir);
        if path.exists() {
            collect_rs_files(&path, &mut rotappo_files)
                .unwrap_or_else(|err| panic!("failed to scan {}: {}", path.display(), err));
        }
    }
    assert!(
        !rotappo_files.is_empty(),
        "no rotappo sources found in src, crates, or tests"
    );

    let rotappo_forbidden = vec![format!("{}::{}", "bootstrappo", "cli")];
    let rotappo_hits = find_forbidden_dependencies(&rotappo_files, &rotappo_forbidden);
    assert!(
        rotappo_hits.is_empty(),
        "rotappo must not import bootstrappo CLI modules:\n{}",
        rotappo_hits.join("\n")
    );

    let bootstrappo_root = root.join("..").join("bootstrappo");
    assert!(
        bootstrappo_root.exists(),
        "bootstrappo repo not found at {}",
        bootstrappo_root.display()
    );

    let mut bootstrappo_files = Vec::new();
    for dir in ["src/cli", "src/bin"] {
        let path = bootstrappo_root.join(dir);
        if path.exists() {
            collect_rs_files(&path, &mut bootstrappo_files)
                .unwrap_or_else(|err| panic!("failed to scan {}: {}", path.display(), err));
        }
    }
    if !bootstrappo_files.is_empty() {
        let bootstrappo_forbidden = vec!["rotappo_".to_string(), "rotappo::".to_string()];
        let bootstrappo_hits =
            find_forbidden_dependencies(&bootstrappo_files, &bootstrappo_forbidden);
        assert!(
            bootstrappo_hits.is_empty(),
            "bootstrappo CLI must not import rotappo crates:\n{}",
            bootstrappo_hits.join("\n")
        );
    }
}
