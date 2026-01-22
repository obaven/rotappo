//! Workspace root crate for composition binaries.

#[cfg(all(
    any(feature = "cli", feature = "tui"),
    not(any(feature = "module-primer", feature = "module-plasmid"))
))]
compile_error!(
    "Enable at least one module feature (`module-primer` or `module-plasmid`) when building cli/tui."
);
