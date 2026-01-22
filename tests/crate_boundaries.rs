use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

struct CrateRule {
    name: &'static str,
    manifest_path: &'static str,
    allowed: &'static [&'static str],
}

fn is_dependency_section(section: &str) -> bool {
    section == "dependencies"
        || section == "dev-dependencies"
        || section == "build-dependencies"
        || section.ends_with(".dependencies")
        || section.ends_with(".dev-dependencies")
        || section.ends_with(".build-dependencies")
}

fn section_dependency_name(section: &str) -> Option<&str> {
    let last = section.split('.').next_back()?;
    if last.starts_with("phenome-") {
        Some(last)
    } else {
        None
    }
}

fn extract_phenome_deps(path: &Path) -> BTreeSet<String> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {}", path.display(), err));
    let mut deps = BTreeSet::new();
    let mut in_deps = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = trimmed.trim_matches(&['[', ']'][..]);
            if let Some(dep) = section_dependency_name(section) {
                deps.insert(dep.to_string());
            }
            in_deps = is_dependency_section(section);
            continue;
        }

        if in_deps {
            if let Some((key, _)) = trimmed.split_once('=') {
                let key = key.trim();
                if key.starts_with("phenome-") {
                    deps.insert(key.to_string());
                }
            }
        }
    }

    deps
}

fn assert_allowed_deps(rule: &CrateRule) {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rule.manifest_path);
    if !manifest_path.exists() {
        panic!(
            "missing Cargo.toml for {} at {}",
            rule.name,
            manifest_path.display()
        );
    }

    let deps = extract_phenome_deps(&manifest_path);
    let allowed: BTreeSet<String> = rule.allowed.iter().map(|dep| dep.to_string()).collect();
    let disallowed: Vec<_> = deps
        .iter()
        .filter(|dep| !allowed.contains(*dep))
        .cloned()
        .collect();

    if !disallowed.is_empty() {
        panic!(
            "{} depends on disallowed phenome crates: {:?} (allowed: {:?})",
            rule.name, disallowed, rule.allowed
        );
    }
}

#[test]
fn crate_dependency_boundaries() {
    let rules = [
        CrateRule {
            name: "phenome-domain",
            manifest_path: "lib/domain/phenome-domain/Cargo.toml",
            allowed: &[],
        },
        CrateRule {
            name: "phenome-ports",
            manifest_path: "lib/ports/phenome-ports/Cargo.toml",
            allowed: &["phenome-domain"],
        },
        CrateRule {
            name: "phenome-application",
            manifest_path: "lib/runtime/phenome-application/Cargo.toml",
            allowed: &["phenome-domain", "phenome-ports"],
        },
        CrateRule {
            name: "phenome-adapter-primer",
            manifest_path: "lib/adapters/phenome-adapter-primer/Cargo.toml",
            allowed: &["phenome-domain", "phenome-ports", "phenome-ui-tui"],
        },
        CrateRule {
            name: "phenome-ui-presentation",
            manifest_path: "lib/ui/phenome-ui-presentation/Cargo.toml",
            allowed: &["phenome-domain"],
        },
        CrateRule {
            name: "phenome-ui-core",
            manifest_path: "lib/ui/phenome-ui-core/Cargo.toml",
            allowed: &["phenome-domain", "phenome-ui-presentation"],
        },
        CrateRule {
            name: "phenome-ui-terminal",
            manifest_path: "lib/ui/phenome-ui-terminal/Cargo.toml",
            allowed: &[
                "phenome-domain",
                "phenome-ui-presentation",
                "phenome-adapter-primer",
            ],
        },
        CrateRule {
            name: "phenome-ui-tui",
            manifest_path: "lib/ui/phenome-ui-tui/Cargo.toml",
            allowed: &[
                "phenome-domain",
                "phenome-application",
                "phenome-ports",
                "phenome-ui-core",
                "phenome-ui-presentation",
            ],
        },
    ];

    for rule in &rules {
        assert_allowed_deps(rule);
    }
}
