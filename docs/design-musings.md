# Design Musings

This is some semi structured format for shaping my thoughts about what the project actually should be. What's requried and how things should be handled.

## Interface

```sh
# Basic usage with explicit arguments
temple render --target githubMarkdown < config.json > README.md
cat config.json | temple render ~/templates/site.html > index.html

# Using environment variables for template reference
export TEMPLE_REF=file:/absolute/path/header.html
cat config.json | temple render --target githubMarkdown > README.md

# Minimal invocation with all config via environment
export TEMPLE_REF=file:/absolute/path/header.html
export TEMPLE_REF_CONFIG=file:/absolute/path/config.json
export TEMPLE_TARGET=githubMarkdown
temple render > README.md

# Fully explicit invocation
cat config.json | temple render --target githubMarkdown template.html > README.md
```

```ts
// CLI interface shape
type cli = (
  target?: Target,
  templateRef?: Uri<Template>,
  configRef?: Uri<Config>
) => string

// Generic reference to a resource
type Uri<T> = `${Protocol}:${string}`

// Supported lookup protocols (v1: file only)
type Protocol = "file" // | "github" | "ipfs" // v2+

// Configuration resource for setting values
// The actual runtime type is flexible (see Root below), but this represents
// a common/recommended structure for templating with values and themes
type Config = {
  values: Record<string, string | Record<string, string>>
  theme: Record<Base16ColorString, HexCode>
}

// Template function that fills values from config
// Templates receive the full config graph and can access any path
type Template = (config: Root) => string

// Supported output targets
type Target = "markdown" | "githubMarkdown" | "html"
```

## Constraint

### Static binary

To avoid unnecessary headaches in usage and reduce the size of the resulting tool (it should be tiny), I am settling on Rust as the implementation language. This is primarily due to its capability for statically compiling against many platforms.

Go was also considered, but its type system was deemed subpar to Rust. My mind was thinking in ADTs when sketching this out.

## Config Schema

**Values:**

The values set is rather straightforward. Just allow organization over paths.

**Theming:**

Here it's a bit more complicated, as we want something simple, while still comforming to broad standards.
All my systems uses (or are transitioning to using) Base16 for easy of transferability. So this will likely be the default.

**Base16 Semantic Concern:**

The Base16 naming convention (`base00`-`base0F`) is semantically opaque—the names don't convey *what* each color represents, only their position in the scheme. However, this is a feature, not a bug: Base16 is designed to be theme-agnostic. The same `base08` might be red in one theme and blue in another, depending on the designer's intent.

For templating purposes, this works well because:
- Templates can reference semantic roles (e.g., "accent color") via Base16 slots
- Users can swap entire themes by replacing the `theme` object
- It maintains compatibility with existing Base16 tooling and themes

To allow for richer semantics, the config shape is only loosely enforced. Templates can access any path in the JSON graph, so users have full flexibility in how they structure their data. The `Config` type above represents a common pattern, but the actual runtime type is the more flexible `Root` below.

```ts
// Minimal configuration supported (actual runtime type)
type Root = Record<string, Node>
type Node = string | number | boolean | Record<string, Node> | Array<Node>

// Minimal example - templates can access via paths like `title`
const minimalConfig: Root = {
  title: "Big Buck Project"
}

// Recommended structure following the Config pattern
const configExample: Root = {
  values: {
    projectName: "temple",
    gitSource: {
      provider: "github",
      user: "zabronax",
      repo: "cli-temple"
    },
  },
  theme: {
    base00: "#1C2023",  // Background (darkest)
    base01: "#2C3033",  // Lighter background
    base02: "#3C4043",  // Selection background
    base03: "#747C84",  // Comments, invisibles
    base04: "#747C84",  // Dark foreground (unused)
    base05: "#C7CCD1",  // Default foreground
    base06: "#C7CCD1",  // Light foreground (unused)
    base07: "#F3F4F5",  // Lightest foreground
    base08: "#C7AE95",  // Red
    base09: "#C7AE95",  // Orange
    base0A: "#AEC795",  // Yellow
    base0B: "#95C7AE",  // Green
    base0C: "#95AEC7",  // Cyan
    base0D: "#AE95C7",  // Blue
    base0E: "#C795AE",  // Magenta
    base0F: "#747C84",  // Brown
  }
}
```

## Error Handling

The template enforces runtime constraints on the config structure by referencing specific paths. The config can only signal when values exist but are unused. This leads to two cases requiring handling:

- Template references non-existent configuration values → `ErrorMissingConfiguration`
- Config contains unused values → `WarningUnusedConfigurationValues`

**Unused Configuration Values:**

Some configuration values are intentionally never referenced but are included to maintain a standardized structure (e.g., the full Base16 theme palette). These warnings should only be emitted at higher verbosity/logging levels to avoid noise.

**Missing Configuration Values:**

While there are legitimate use cases for templates to have default fallbacks, supporting this is not desirable for v1. This is a declarative tool—templates should explicitly declare their dependencies. If default fallbacks are needed in the future, config merging before template application would be a better and more flexible approach (v2+).

## Design Decisions

- Defaulting templates to HTML seems prudent due to its semantic richness, then optionally downgrading to subsets
- Supporting remote protocols (github, ipfs) will come in v2+. Local file references only for v1
- Template and config references can be provided via arguments or environment variables (`TEMPLE_REF`, `TEMPLE_REF_CONFIG`, `TEMPLE_TARGET`)
- Insertion into the middle of documents will have to be handled by other tools
- This is more like a JSX function call in a CLI than a traditional header generator
