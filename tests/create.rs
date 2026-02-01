use assert_cmd::cargo::cargo_bin_cmd;
use temple::config::DEFAULT_CONFIG_TEMPLATE;

#[test]
fn create_without_subcommand_shows_help() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn create_config_invocation_creates_default_config() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create").arg("config");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(DEFAULT_CONFIG_TEMPLATE));
}

#[test]
fn create_template_creates_template() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create").arg("template");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Creating template"));
}
