use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::io::Write;
use tempfile::TempDir;
use temple::{config::DEFAULT_CONFIG_TEMPLATE, template::DEFAULT_TEMPLE_TEMPLATE};

#[test]
fn render_without_args_shows_help() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("render");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn render_with_help_flag_shows_help() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("render").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}

#[test]
fn render_with_template_and_config_renders_correctly() {
    // Create temporary directory for test files
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let config_path = temp_dir.path().join("config.json");

    // Write template to file
    let mut template_file = fs::File::create(&template_path).unwrap();
    template_file
        .write_all(DEFAULT_TEMPLE_TEMPLATE.as_bytes())
        .unwrap();
    template_file.sync_all().unwrap();

    // Write config to file
    let mut config_file = fs::File::create(&config_path).unwrap();
    config_file
        .write_all(DEFAULT_CONFIG_TEMPLATE.as_bytes())
        .unwrap();
    config_file.sync_all().unwrap();

    // Invoke render command
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("render")
        .arg("--template-ref")
        .arg(template_path.to_str().unwrap())
        .arg("--config-ref")
        .arg(config_path.to_str().unwrap())
        .arg("--target")
        .arg("html");

    // Assert stdout contains rendered values from config
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("temple")) // projectName
        .stdout(predicates::str::contains("zabronax")) // gitSource.user
        .stdout(predicates::str::contains("cli-temple")); // gitSource.repo
}
