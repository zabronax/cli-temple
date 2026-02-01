{
  description = "CLI for generating readme headers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      package = cargoToml.package;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-darwin"
        # TODO! Not verified, but should work.
        # Move them up when verified.
        "x86_64-darwin" # macOS on x86_64
        "aarch64-linux" # Linux on aarch64
      ];

      # Helper functions for describing the various shells.
      withSystem = nixpkgs.lib.genAttrs supportedSystems;
      withPkgs = callback: withSystem (system: callback (import nixpkgs { inherit system; }));
    in
    {
      packages = withPkgs (pkgs: rec {
        default = temple;

        temple = pkgs.rustPlatform.buildRustPackage {
          pname = package.name;
          version = package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = package.description;
            homepage = package.homepage;
            license = licenses.mit;
            maintainers = [ ];
            mainProgram = package.name;
          };
        };
      });

      devShells = withPkgs (pkgs: rec {
        default = development;

        development = pkgs.mkShell {
          packages = with pkgs; [
            cargo # Build tool and package manager
            rustfmt # Code formatter (needed for `cargo fmt`)
            rust-analyzer # LSP server for IDE support
            clippy # Linter for catching common mistakes
            cargo-cross # Cross-compilation tool
            opentofu # OpenTofu package managers
          ];
        };
      });

      # The Nix file formatter for the project.
      formatter = withPkgs (pkgs: pkgs.nixfmt-rfc-style);
    };
}
