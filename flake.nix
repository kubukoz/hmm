{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:nix-community/naersk";
  inputs.naersk.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk = pkgs.callPackage inputs.naersk { };
      in
      {
        defaultPackage = naersk.buildPackage {
          # Assuming there's `Cargo.toml` right in this directory:
          src = ./.;

          buildInputs = [
            pkgs.openssl
            pkgs.rustfmt
            pkgs.crate2nix
          ];

          nativeBuildInputs = [
            pkgs.installShellFiles
          ] ++
          pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.pkg-config
            pkgs.zlib
          ];

          # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          postInstall = ''
            installShellCompletion --name _hmm completions/zsh/_hmm --zsh
          '';
        };
        checks = {
          defaultPackageBuild = self.defaultPackage.${system};
        };
      }
    );
}
