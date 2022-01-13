{ pkgs, stdenv, installShellFiles, lib, darwin, openssl, pkg-config, zlib }:

(import ./Cargo.nix { inherit pkgs; }).rootCrate.build.overrideAttrs (prev: {
  buildInputs = prev.buildInputs ++ [ openssl ];
  nativeBuildInputs = prev.nativeBuildInputs ++ [
    installShellFiles
  ] ++
    lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
      pkg-config
      zlib
    ];

  postInstall = ''
    installShellCompletion --name _hmm completions/zsh/_hmm --zsh
  '';
})
