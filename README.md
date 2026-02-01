# Temple

A small, easy-to-use CLI tool for structured templating with support for complex configurations like color schemes.

## Motivation

I scaffold many projects and want most of them to include basic badges and an image with links. This tool helps ensure coherence across those projects.

I also realized this is better thought of as a generic templating tool that's more structured than something like `envsubst`. Tools like `envsubst` work well for a small set of variables, but I needed support for structured inputs like color schemes, which requires more sophisticated templating.

## Constraint

### Static binary

To avoid unnecessary headaches in usage and reduce the size of the resulting tool (it should be tiny), I am settling on Rust as the implementation language. This is primarily due to its capability for statically compiling against many platforms.

Go was also considered, but its type system was deemed subpar to Rust. My mind was thinking in ADTs when sketching this out.

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
type Config = {
  value: Record<string, string>
  theme: Record<Base16ColorString, HexCode>
}

// Template function that fills values from config
type Template = (config: Config) => string

// Supported output targets
type Target = "markdown" | "githubMarkdown" | "html"
```

## Design Decisions

- Defaulting templates to HTML seems prudent due to its semantic richness, then optionally downgrading to subsets
- Supporting remote protocols (github, ipfs) will come in v2+. Local file references only for v1
- Template and config references can be provided via arguments or environment variables (`TEMPLE_REF`, `TEMPLE_REF_CONFIG`, `TEMPLE_TARGET`)
- Insertion into the middle of documents will have to be handled by other tools
- This is more like a JSX function call in a CLI than a traditional header generator
