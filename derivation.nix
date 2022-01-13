{ pkgs, stdenv, installShellFiles, lib, darwin, openssl, pkg-config, zlib, gitignore-source }:

(import ./Cargo.nix { inherit pkgs; }).rootCrate.build.overrideAttrs (prev: {
  src = gitignore-source.lib.gitignoreSource ./.;

  buildInputs = prev.buildInputs ++ [
    openssl
    pkgs.rustfmt
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
