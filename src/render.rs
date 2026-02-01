use handlebars::Handlebars;
use serde_json::Value as JsonValue;
use std::fs;

pub fn render_template(
    template_path: &str,
    config_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Read config file and parse JSON
    let config_content = fs::read_to_string(config_path)?;
    let config_json: JsonValue = serde_json::from_str(&config_content)?;

    // Read template file
    let template_content = fs::read_to_string(template_path)?;

    // Create Handlebars registry and register template
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", &template_content)?;

    // Render template with config data
    let rendered = handlebars.render("template", &config_json)?;

    Ok(rendered)
}
