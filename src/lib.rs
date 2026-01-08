//! Workspace root crate for composition binaries.

#[cfg(all(
    any(feature = "cli", feature = "tui"),
    not(any(feature = "module-bootstrappo", feature = "module-rotato"))
))]
compile_error!(
    "Enable at least one module feature (`module-bootstrappo` or `module-rotato`) when building cli/tui."
);
