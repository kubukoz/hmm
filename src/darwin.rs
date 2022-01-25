use std::process::Command;

/// Runs darwin-rebuild and expects it to succeed.
pub(crate) fn rebuild_system() {
    let exit = Command::new("darwin-rebuild")
        .arg("switch")
        .arg("--flake")
        .arg("~/.nixpkgs")
        .spawn()
        .expect("Couldn't start darwin-rebuild")
        .wait()
        .expect("System rebuild failed")
        .code()
        .expect("Command didn't return an exit code");

    assert_eq!(exit, 0, "System rebuild failed");
}
