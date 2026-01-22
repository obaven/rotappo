#[cfg(all(feature = "module-primer", feature = "module-plasmid"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    anyhow::bail!("Enable only one module feature when building the cli bin.")
}

#[cfg(all(feature = "module-primer", not(feature = "module-plasmid")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phenome_ui_terminal::cli::primer::run().await
}

#[cfg(all(feature = "module-plasmid", not(feature = "module-primer")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phenome_ui_terminal::cli::plasmid::run().await
}
