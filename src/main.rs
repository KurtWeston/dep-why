use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::*;
use std::path::PathBuf;

mod parser;
mod tracer;

use parser::LockFile;
use tracer::DependencyTracer;

#[derive(Parser)]
#[command(name = "dep-why")]
#[command(about = "Trace dependency chains to understand why packages are installed")]
struct Cli {
    #[arg(help = "Package name or package@version to trace")]
    package: String,

    #[arg(short, long, help = "Path to lock file (auto-detected if not specified)")]
    file: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value = "tree")]
    output: OutputFormat,

    #[arg(long, help = "Show all dependency paths (not just shortest)")]
    all_paths: bool,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Tree,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let lock_file_path = cli.file.unwrap_or_else(|| detect_lock_file());
    
    let lock_file = LockFile::parse(&lock_file_path)?;
    let tracer = DependencyTracer::new(lock_file);
    
    let (package_name, version) = parse_package_query(&cli.package);
    let chains = tracer.trace(package_name, version.as_deref(), cli.all_paths)?;
    
    match cli.output {
        OutputFormat::Tree => print_tree(&chains, package_name),
        OutputFormat::Json => print_json(&chains)?,
    }
    
    Ok(())
}

fn detect_lock_file() -> PathBuf {
    if PathBuf::from("package-lock.json").exists() {
        PathBuf::from("package-lock.json")
    } else if PathBuf::from("yarn.lock").exists() {
        PathBuf::from("yarn.lock")
    } else if PathBuf::from("pnpm-lock.yaml").exists() {
        PathBuf::from("pnpm-lock.yaml")
    } else {
        eprintln!("{}\n", "No lock file found in current directory".red());
        std::process::exit(1);
    }
}

fn parse_package_query(query: &str) -> (&str, Option<String>) {
    if let Some(pos) = query.rfind('@') {
        if pos > 0 {
            return (&query[..pos], Some(query[pos + 1..].to_string()));
        }
    }
    (query, None)
}

fn print_tree(chains: &[Vec<tracer::DependencyNode>], target: &str) {
    if chains.is_empty() {
        println!("{}", format!("Package '{}' not found in lock file", target).red());
        return;
    }
    
    println!("\n{}", format!("Found {} dependency chain(s) for '{}':\n", chains.len(), target).bold());
    
    for (idx, chain) in chains.iter().enumerate() {
        println!("{}\n", format!("Chain {}:", idx + 1).cyan().bold());
        for (depth, node) in chain.iter().enumerate() {
            let indent = "  ".repeat(depth);
            let arrow = if depth > 0 { "└─ " } else { "" };
            let pkg_info = format!("{}@{}", node.name, node.version);
            println!("{}{}{}", indent, arrow, pkg_info.green());
        }
        println!();
    }
}

fn print_json(chains: &[Vec<tracer::DependencyNode>]) -> Result<()> {
    let json = serde_json::to_string_pretty(chains)?;
    println!("{}", json);
    Ok(())
}