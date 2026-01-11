#[cfg(all(feature = "module-bootstrappo", feature = "module-rotato"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    anyhow::bail!("Enable only one module feature when building the cli bin.")
}

#[cfg(all(feature = "module-bootstrappo", not(feature = "module-rotato")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rotappo_ui_terminal::cli::bootstrappo::run().await
}

#[cfg(all(feature = "module-rotato", not(feature = "module-bootstrappo")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rotappo_ui_terminal::cli::rotato::run().await
}
