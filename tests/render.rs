use assert_cmd::cargo::cargo_bin_cmd;

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
