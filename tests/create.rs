use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn given_create_command_when_invoking_without_subcommand_then_help_is_displayed() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn given_create_config_command_when_invoking_then_config_is_created() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create").arg("config");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Creating config"));
}
