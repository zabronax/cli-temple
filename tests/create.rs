use assert_cmd::cargo::cargo_bin_cmd;
use temple::config::DEFAULT_CONFIG_TEMPLATE;

pub const DEFAULT_TEMPLE_TEMPLATE: &str = r##"
<div align="center">
  <a href="{{.values.projectUrl}}">
    <img alt="Logo" src="{{.values.logoUrl}}" height="128">
  </a>
  <h1>{{.values.projectName}}</h1>

  <div>
    <a href="{{.values.licenseUrl}}">
        <img alt="License" src="https://img.shields.io/github/license/{{.values.gitSource.user}}/{{.values.gitSource.repo}}?style=for-the-badge&labelColor={{.theme.base00}}&color={{.theme.base08}}">
    </a>
  </div>

  <div>
    <a href="https://github.com/{{.values.gitSource.user}}/{{.values.gitSource.repo}}">
      <img alt="Repository Status" src="https://img.shields.io/github/last-commit/{{.values.gitSource.user}}/{{.values.gitSource.repo}}?style=for-the-badge&label=Last%20Updated&labelColor={{.theme.base00}}">
    </a>
  </div>
</div>
"##;

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
fn create_template_invocation_creates_default_template() {
    let mut cmd = cargo_bin_cmd!("temple");
    cmd.arg("create").arg("template");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(DEFAULT_TEMPLE_TEMPLATE));
}
