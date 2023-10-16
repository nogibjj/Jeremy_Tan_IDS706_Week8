// tests/cli_tests.rs
#[cfg(test)]
use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn test_encrypt_command() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let temp_dir_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&["--message", "Hello World", "--encrypt"]);

    cmd.assert().success();
}

#[test]
fn test_decrypt_command() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let temp_dir_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&["--message", "QTrbyq87yaS8jwVFpr6stw=", "--decrypt"]);

    cmd.assert().success();
}
