use handlebars::Handlebars;
use serde_json::Value as JsonValue;
use std::fs;
use std::io;

pub fn render_template(
    template_path: &str,
    config_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Read config file and parse JSON
    let config_content = fs::read_to_string(config_path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            "config file not found".to_string()
        } else {
            format!("failed to read config file: {}", e)
        }
    })?;

    let config_json: JsonValue =
        serde_json::from_str(&config_content).map_err(|_| "config is invalid JSON".to_string())?;

    // Read template file
    let template_content = fs::read_to_string(template_path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            "template file not found".to_string()
        } else {
            format!("failed to read template file: {}", e)
        }
    })?;

    // Create Handlebars registry and register template
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_template_string("template", &template_content)?;

    // Render template with config data
    let rendered = handlebars.render("template", &config_json).map_err(|e| {
        format!("template references non-existent configuration value: {}", e)
    })?;

    Ok(rendered)
}
