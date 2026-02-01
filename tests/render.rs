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

#[test]
fn render_with_piped_config_renders_correctly() {
    // Create temporary directory for test files
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");

    // Write template to file
    let mut template_file = fs::File::create(&template_path).unwrap();
    template_file
        .write_all(DEFAULT_TEMPLE_TEMPLATE.as_bytes())
        .unwrap();
    template_file.sync_all().unwrap();

    // Invoke render command with config piped via stdin
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("render")
        .arg("--template-ref")
        .arg(template_path.to_str().unwrap())
        .write_stdin(DEFAULT_CONFIG_TEMPLATE);

    // Assert stdout contains rendered values from piped config
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("temple")) // projectName
        .stdout(predicates::str::contains("zabronax")) // gitSource.user
        .stdout(predicates::str::contains("cli-temple")); // gitSource.repo
}

struct ErrorTestCase {
    reference_error: &'static str,
    error_message: &'static str,
    setup_template: fn(&std::path::Path) -> std::path::PathBuf,
    setup_config: fn(&std::path::Path) -> std::path::PathBuf,
}

#[test]
fn render_with_invalid_references_fails() {
    let test_cases = vec![
        ErrorTestCase {
            reference_error: "NonexistentTemplate",
            error_message: "template file not found",
            setup_template: |temp_dir| temp_dir.join("nonexistent.tmpl"),
            setup_config: |temp_dir| {
                let config_path = temp_dir.join("config.json");
                let mut config_file = fs::File::create(&config_path).unwrap();
                config_file
                    .write_all(DEFAULT_CONFIG_TEMPLATE.as_bytes())
                    .unwrap();
                config_file.sync_all().unwrap();
                config_path
            },
        },
        ErrorTestCase {
            reference_error: "NonexistentConfig",
            error_message: "config file not found",
            setup_template: |temp_dir| {
                let template_path = temp_dir.join("template.tmpl");
                let mut template_file = fs::File::create(&template_path).unwrap();
                template_file
                    .write_all(DEFAULT_TEMPLE_TEMPLATE.as_bytes())
                    .unwrap();
                template_file.sync_all().unwrap();
                template_path
            },
            setup_config: |temp_dir| temp_dir.join("nonexistent.json"),
        },
        ErrorTestCase {
            reference_error: "InvalidJsonConfig",
            error_message: "config is invalid JSON",
            setup_template: |temp_dir| {
                let template_path = temp_dir.join("template.tmpl");
                let mut template_file = fs::File::create(&template_path).unwrap();
                template_file
                    .write_all(DEFAULT_TEMPLE_TEMPLATE.as_bytes())
                    .unwrap();
                template_file.sync_all().unwrap();
                template_path
            },
            setup_config: |temp_dir| {
                let config_path = temp_dir.join("config.json");
                let mut config_file = fs::File::create(&config_path).unwrap();
                config_file.write_all(b"{ invalid json }").unwrap();
                config_file.sync_all().unwrap();
                config_path
            },
        },
    ];

    for test_case in test_cases {
        let temp_dir = TempDir::new().unwrap();
        let template_path = (test_case.setup_template)(temp_dir.path());
        let config_path = (test_case.setup_config)(temp_dir.path());

        let mut cmd = cargo_bin_cmd!("temple");
        cmd.arg("render")
            .arg("--template-ref")
            .arg(template_path.to_str().unwrap())
            .arg("--config-ref")
            .arg(config_path.to_str().unwrap());

        let assert = cmd.assert().failure();
        let output = assert.get_output();

        // Include reference_error in test output for debugging
        eprintln!(
            "Test case '{}' - Expected: '{}', Got: {}",
            test_case.reference_error,
            test_case.error_message,
            String::from_utf8_lossy(&output.stderr)
        );

        assert.stderr(predicates::str::contains(test_case.error_message));
    }
}

#[test]
fn render_with_missing_template_value_fails() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let config_path = temp_dir.path().join("config.json");

    // Write template that references a missing value
    fs::write(&template_path, "{{values.missingValue}}").unwrap();

    // Write minimal config without the referenced value
    fs::write(&config_path, r#"{"values": {}}"#).unwrap();

    // Invoke render command
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("render")
        .arg("--template-ref")
        .arg(template_path.to_str().unwrap())
        .arg("--config-ref")
        .arg(config_path.to_str().unwrap());

    // Assert it fails with a templating error message
    cmd.assert().failure().stderr(predicates::str::contains(
        "template references non-existent configuration value",
    ));
}
