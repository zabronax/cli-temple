use clap::{Parser, Subcommand};
use std::process;

use temple::{config, render, template};

/// A small, easy-to-use CLI tool for structured templating with support for complex configurations like color schemes.
#[derive(Parser, Debug)]
#[command(name = "temple")]
#[command(version = "0.1.0")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render a template into a target format
    Render {
        /// The target format to render the template into
        #[arg(short = 'T', long)]
        target: Option<String>,

        /// The reference to the template to render
        #[arg(short, long, required = true)]
        template_ref: String,

        /// The reference to the config to use for rendering the template
        /// If not provided, config will be read from stdin
        #[arg(short, long)]
        config_ref: Option<String>,
    },

    /// Scaffolds a new config or template
    Create {
        #[command(subcommand)]
        resource: CreateResource,
    },
}

#[derive(Subcommand, Debug)]
enum CreateResource {
    /// Scaffolds a new config
    Config,

    /// Scaffolds a new template
    Template,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Render {
            target,
            template_ref,
            config_ref,
        } => {
            if let Err(e) = render(target.as_deref(), template_ref, config_ref.as_deref()) {
                eprintln!("Error rendering template: {}", e);
                process::exit(1);
            }
        }
        Commands::Create { resource } => match resource {
            CreateResource::Config => create_config(),
            CreateResource::Template => create_template(),
        },
    }
}

fn render(
    _target: Option<&str>,
    template_ref: &str,
    config_ref: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = render::render_template(template_ref, config_ref)?;
    print!("{}", rendered);
    Ok(())
}

fn create_config() {
    print!("{}", config::DEFAULT_CONFIG_TEMPLATE);
}

fn create_template() {
    print!("{}", template::DEFAULT_TEMPLE_TEMPLATE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        // This will panic at test time if CLI is misconfigured
        // Catches issues like conflicting arguments, invalid requirements, etc.
        Cli::command().debug_assert();
    }
}
