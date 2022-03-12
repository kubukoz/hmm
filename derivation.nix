{ pkgs, stdenv, installShellFiles, lib, darwin, openssl, pkg-config, zlib, gitignore-source }:
let cargo_nix = import ./Cargo.nix { inherit pkgs; }; in

cargo_nix.rootCrate.build.overrideAttrs (prev: {
  src = gitignore-source.lib.gitignoreSource ./.;

  buildInputs = prev.buildInputs ++ [
    openssl
    pkgs.rustfmt
    pkgs.crate2nix
  ];
  nativeBuildInputs = prev.nativeBuildInputs ++ [
    installShellFiles
  ] ++
    lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
      pkg-config
      zlib
    ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  postInstall = ''
    installShellCompletion --name _hmm completions/zsh/_hmm --zsh
  '';
})
