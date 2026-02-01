use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn given_no_arguments_when_invoking_cli_then_help_is_displayed() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn given_help_flag_when_invoking_cli_then_help_is_displayed() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}
