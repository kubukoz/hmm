{ pkgs, stdenv, installShellFiles, lib, darwin, openssl }:

(import ./Cargo.nix { inherit pkgs; }).rootCrate.build.overrideAttrs (prev: {
  buildInputs = prev.buildInputs ++ [ openssl ];
  nativeBuildInputs = prev.nativeBuildInputs ++ [
    installShellFiles
  ] ++
    lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
    ];

  postInstall = ''
    installShellCompletion --name _hmm completions/zsh/_hmm --zsh
  '';
})
