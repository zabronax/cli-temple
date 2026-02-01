{
  description = "CLI for generating readme headers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
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
      devShells = withPkgs (pkgs: rec {
        default = development;

        development = pkgs.mkShell {
          packages = with pkgs; [
          ];
        };
      });

      # The Nix file formatter for the project.
      formatter = withPkgs (pkgs: pkgs.nixfmt-rfc-style);
    };
}
