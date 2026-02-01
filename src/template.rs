pub const DEFAULT_TEMPLE_TEMPLATE: &str = r##"<h1 align="center">
  <img height="160" src="{{values.logoUrl}}" />
  <p>{{values.projectDisplayName}}</p>
</h1>

<p align="center">
  <a href="{{values.projectUrl}}"><img
      src="https://img.shields.io/github/stars/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0A}}&style=for-the-badge"></a>
  <a href="{{values.projectUrl}}/commits"><img
      src="https://img.shields.io/github/last-commit/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0B}}&style=for-the-badge"></a>
  <a href="{{values.projectUrl}}/blob/main/LICENSE"><img
      src="https://img.shields.io/github/license/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0C}}&style=for-the-badge"></a>
</p>
"##;
