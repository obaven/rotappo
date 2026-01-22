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
                    hits.push(format!("{}:{}: {}", path.display(), line_no + 1, trimmed));
                }
            }
        }
    }
    hits
}

#[test]
fn cli_boundaries_are_enforced() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut phenome_files = Vec::new();
    for dir in ["src", "crates", "tests"] {
        let path = root.join(dir);
        if path.exists() {
            collect_rs_files(&path, &mut phenome_files)
                .unwrap_or_else(|err| panic!("failed to scan {}: {}", path.display(), err));
        }
    }
    assert!(
        !phenome_files.is_empty(),
        "no phenome sources found in src, crates, or tests"
    );

    let phenome_forbidden = vec![format!("{}::{}", "primer", "cli")];
    let phenome_hits = find_forbidden_dependencies(&phenome_files, &phenome_forbidden);
    assert!(
        phenome_hits.is_empty(),
        "phenome must not import primer CLI modules:\n{}",
        phenome_hits.join("\n")
    );

    let primer_root = root.join("..").join("primer");
    assert!(
        primer_root.exists(),
        "primer repo not found at {}",
        primer_root.display()
    );

    let mut primer_files = Vec::new();
    for dir in ["src/cli", "src/bin"] {
        let path = primer_root.join(dir);
        if path.exists() {
            collect_rs_files(&path, &mut primer_files)
                .unwrap_or_else(|err| panic!("failed to scan {}: {}", path.display(), err));
        }
    }
    if !primer_files.is_empty() {
        let primer_forbidden = vec![
            "phenome_".to_string(),
            "phenome::".to_string(),
        ];
        let primer_hits = find_forbidden_dependencies(&primer_files, &primer_forbidden);
        assert!(
            primer_hits.is_empty(),
            "primer CLI must not import phenome crates:\n{}",
            primer_hits.join("\n")
        );
    }
}
