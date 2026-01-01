use anyhow::{anyhow, Result};
use std::env;

use rotappo::adapters::bootstrappo::BootstrappoBackend;
use rotappo::cli::{format_actions, format_snapshot, OutputMode};

fn main() -> Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    let mut output = OutputMode::Plain;
    let mut command = "snapshot".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "actions" | "snapshot" => {
                command = args[i].clone();
            }
            "--output" | "-o" => {
                let value = args.get(i + 1).ok_or_else(|| anyhow!("Missing output mode"))?;
                output = OutputMode::from_str(value)
                    .ok_or_else(|| anyhow!("Invalid output mode: {}", value))?;
                i += 1;
            }
            "--config" => {
                let value = args.get(i + 1).ok_or_else(|| anyhow!("Missing config path"))?;
                env::set_var("BOOTSTRAPPO_CONFIG_PATH", value);
                i += 1;
            }
            "--plan" => {
                let value = args.get(i + 1).ok_or_else(|| anyhow!("Missing plan path"))?;
                env::set_var("BOOTSTRAPPO_PLAN_PATH", value);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let backend = BootstrappoBackend::from_env()?;
    let runtime = backend.runtime();

    let output_text = match command.as_str() {
        "actions" => format_actions(output, runtime.registry().actions())?,
        "snapshot" => format_snapshot(output, runtime.snapshot())?,
        other => return Err(anyhow!("Unknown command: {}", other)),
    };

    println!("{}", output_text);
    Ok(())
}

fn print_help() {
    println!("rotappo [actions|snapshot] [--output plain|json|ndjson] [--config PATH] [--plan PATH]");
}
