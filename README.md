<h1 align="center">
  <img height="160" src="https://raw.githubusercontent.com/zabronax/cli-temple/main/docs/logo.svg" />
  <p>cli temple</p>
</h1>

<p align="center">
  <a href="https://github.com/zabronax/cli-temple"><img
      src="https://img.shields.io/github/stars/zabronax/cli-temple?colorA=1C2023&colorB=AEC795&style=for-the-badge"></a>
  <a href="https://github.com/zabronax/cli-temple/commits"><img
      src="https://img.shields.io/github/last-commit/zabronax/cli-temple?colorA=1C2023&colorB=95C7AE&style=for-the-badge"></a>
  <a href="https://github.com/zabronax/cli-temple/blob/main/LICENSE"><img
      src="https://img.shields.io/github/license/zabronax/cli-temple?colorA=1C2023&colorB=95AEC7&style=for-the-badge"></a>
</p>

A small, easy-to-use CLI tool for structured templating with support for complex configurations like color schemes.

## Installation

Download the latest release for your platform:

```sh
curl -L https://github.com/zabronax/cli-temple/releases/latest/download/temple-$platform -o temple
chmod +x temple
```

## Quick Start

```sh
# Create a config and template file
temple create config > config.json
temple create template > template.html

# Customize config.json and template.html with your values

# Render output
cat config.json | temple render --template-ref template.html > output.html
```

## Examples

### README Header with Badges

Generate a consistent project header with logo, badges, and theme colors:

**config.json:**
```json
{
  "values": {
    "projectDisplayName": "cli temple",
    "projectUrl": "https://github.com/user/my-project",
    "logoUrl": "https://example.com/logo.svg",
    "licenseUrl": "https://github.com/user/my-project/blob/main/LICENSE",
    "gitSource": {
      "provider": "github",
      "user": "user",
      "repo": "my-project"
    }
  },
  "theme": {
    "base00": "1C2023",
    "base0A": "AEC795",
    "base0B": "95C7AE",
    "base0C": "95AEC7"
  }
}
```

**template.html:**
```html
<h1 align="center">
  <img height="160" src="{{values.logoUrl}}" />
  <p>{{values.projectDisplayName}}</p>
</h1>

<p align="center">
  <a href="{{values.projectUrl}}">
    <img
      src="https://img.shields.io/github/stars/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0A}}&style=for-the-badge">
  </a>
  <a href="{{values.projectUrl}}/commits">
    <img
      src="https://img.shields.io/github/last-commit/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0B}}&style=for-the-badge">
  </a>
  <a href="{{values.projectUrl}}/blob/main/LICENSE">
    <img
      src="https://img.shields.io/github/license/{{values.gitSource.user}}/{{values.gitSource.repo}}?colorA={{theme.base00}}&colorB={{theme.base0C}}&style=for-the-badge">
  </a>
</p>
```

**Render for GitHub Markdown:**
```sh
cat config.json | temple render --template-ref index.html > README.md
```

This generates a centered header with your project logo, name, and badges for stars, last commit, and license, styled with your theme colors.

## Use Cases

### Templating Project Headers

Create consistent headers across multiple projects with shared templates. Define your project metadata once and generate headers that maintain visual and structural coherence.

### Templating READMEs

Generate README files from templates and configuration. Keep your project documentation consistent while customizing content per project through structured configuration.

See [docs/design-musings.md](docs/design-musings.md) for interface design, implementation constraints, and design decisions.
