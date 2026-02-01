# Temple

A small, easy-to-use CLI tool for structured templating with support for complex configurations like color schemes.

## Motivation

I scaffold many projects and want most of them to include basic badges and an image with links. This tool helps ensure coherence across those projects.

I also realized this is better thought of as a generic templating tool that's more structured than something like `envsubst`. Tools like `envsubst` work well for a small set of variables, but I needed support for structured inputs like color schemes, which requires more sophisticated templating.

See [docs/design-musings.md](docs/design-musings.md) for interface design, implementation constraints, and design decisions.
