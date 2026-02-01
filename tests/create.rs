use assert_cmd::cargo::cargo_bin_cmd;

const DEFAULT_CONFIG_TEMPLATE: &str = r#"
{
  values: {
    projectName: "temple",
    gitSource: {
      provider: "github",
      user: "zabronax",
      repo: "cli-temple"
    }
  }
  theme: {
    base00: '#1C2023',
    base01: '#2C3033',
    base02: '#3C4043',
    base03: '#747C84',
    base04: '#747C84',
    base05: '#C7CCD1',
    base06: '#C7CCD1',
    base07: '#F3F4F5',
    base08: '#C7AE95',
    base09: '#C7AE95',
    base0A: '#AEC795',
    base0B: '#95C7AE',
    base0C: '#95AEC7',
    base0D: '#AE95C7',
    base0E: '#C795AE',
    base0F: '#747C84'
  }
}
"#;

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
