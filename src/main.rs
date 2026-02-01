use clap::{Arg, Command};

use temple::config;

fn cli() -> Command {
    Command::new("temple")
        .version("0.1.0")
        .about("A small, easy-to-use CLI tool for structured templating with support for complex configurations like color schemes.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Rendering subcommand
        .subcommand(
            Command::new("render")
            .about("Render a template into a target format")
            // Target format to render the template into
            .arg(Arg::new("target")
            .short('T')
            .long("target")
            .help("The target format to render the template into"))
            // Reference to the template to render
            .arg(Arg::new("template_ref")
            .short('t')
            .long("template-ref")
            .required(true)
            .help("The reference to the template to render"))
            // Reference to the config to use for rendering the template
            .arg(Arg::new("config_ref")
            .short('c')
            .long("config-ref")
            .help("The reference to the config to use for rendering the template"))
        )
        // Creating subcommand
        .subcommand(
            Command::new("create")
            .about("Scaffolds a new config or template")
            .subcommand_required(true)
            .arg_required_else_help(true)
            // Keep the option open for extensibility for more resource types
              .subcommand(
                Command::new("config")
                .about("Scaffolds a new config")
              )
              .subcommand(
                Command::new("template")
                .about("Scaffolds a new template")
              )
            )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("render", sub_matches)) => {
            let target = sub_matches
                .get_one::<String>("target")
                .map(String::as_str)
                .expect("target is required");

            let template_ref = sub_matches
                .get_one::<String>("template_ref")
                .map(String::as_str)
                .expect("template_ref is required");

            let config_ref = sub_matches
                .get_one::<String>("config_ref")
                .map(String::as_str)
                .expect("config_ref is required");

            render(target, template_ref, config_ref);
        }
        Some(("create", sub_matches)) => match sub_matches.subcommand() {
            Some(("config", _)) => {
                create_config();
            }
            Some(("template", _)) => {
                create_template();
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn render(target: &str, template_ref: &str, config_ref: &str) {
    println!("Target: {}", target);
    println!("Template reference: {}", template_ref);
    println!("Config reference: {}", config_ref);
}

fn create_config() {
    print!("{}", config::DEFAULT_CONFIG_TEMPLATE);
}

fn create_template() {
    println!("Creating template");
}
