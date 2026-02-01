# Header Template

A small, easy-to-use CLI tool for standardizing the creation of headers for projects.

## Motivation

I seem to scaffold a lot of projects, and want most of them to at least include some basic badges as well as an image with links.

This handy tool would help with ensuring some form of coherence across those.

## Imagined Interface

**Call Pattern Alternatives:**
```sh
# Option #1
header-template render --postProcessor githubMarkdown ~/templates/project-header.html ./config.json > README.md
# Option #2
cat config.json | header-template render ~/templates/default-header.html > index.html
cat config.json | header-template render github:zabronax/templates?path=/headers/basic-header.html > index.html

# Reading from Env
cat config.json | header-template render --templateRefEnv BASIC_HEADER > index.html
# Standardized Env?
export HEADER_GEN_TEMPLATE_REF=github:zabronax/templates/project-header.html
cat config.json | header-template render > index.html
# Minimal set
cat config.json | header-template render > README.md
header-template render < config.json > README.md

# Write a config-template (includes schema). Reserve the options for adding other types later.
header-template create config > config.json
header-template create header-ci-badges > header-standard-project.html
header-template create header-private-project > header-private-project.html

# A better name for the tool
temple render --target githubMarkdown < config.json > README.md
cat config.json | temple render > index.html
# Fitting env name (My current favorite, and what I see as most sensible)
export TEMPLE_REF=file:/absolute/path/header.html
cat config.json | temple render --target githubMarkdown > README.md
# Possibly the config as well, then user can hand edit the result
export TEMPLE_REF=file:/absolute/path/header.html
export TEMPLE_REF_CONFIG=file:/absolute/path/config.json
temple render --target githubMarkdown > README.md
```

```ts
// The imagined shape of the CLI interface with flags or arguments, expressed as a function
type cli = (targetFlag: Target?, templateRef: Uri<Template>, configRef: Uri<Config>) => string
// A generic reference to a resource
type Uri<T> = (`${protocol: Protocol}:${path}?${options: Options<Protocol>}`) => T
// Supported lookup protocols
type Protocol = "file" | "github" | "ipfs"
// Narrow Records for each protocol (My TS is rusty so this is likely wrong)
type Options<T extends Protocol> = {
  "file": null | undefined,
  "github": `path=${path}` | `ref=${ref}`, // This is definitely wrong
  "ipfs": null | undefined, // Uncertain what would be meaningful here
}
// Configuration resource for setting values
type Config = {
  value: Record<string, string>
  theme: Record<Base16ColorString, HexCode>
}
// Fills out a template string with values from the config
type Template = (config) => string
// Transforms the template into a subset matching target constraints
type PostProcessor = Record<PostProcessorType, (string) => string>
// Supported narrowing processors
type Target = "markdown" | "githubMarkdown" | "html" 
```

## Realizations

- Defaulting templates to HTML seems prudent due to its semantic richness, and then optionally downgrading to subsets
- Supporting remote protocols will have to come in version >1. Local is for v1.
- Providing a utility for scaffolding a config in repo is easier than advanced UI flows, and straightforward to version and automate in CI
- Insertion into the middle of documents will have to be handled by other tools
- TemplateRef should likely also be settable through an Env variable to ease standardization. Perhaps something like `--templateRefEnv HEADER_GEN_TEMPLATE_REF`
- Template scaffolding explosion is a thing. Likely better to allow the user to supply their own template directory which can be sourced from. With just a tiny handful hardcoded in the tool (default, private, organization). This is v2+ material
- This is also not really a header generator though, that is my primary goal. It's more like a JSX function call in a CLI than anything else.
