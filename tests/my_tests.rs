// tests/cli_tests.rs
#[cfg(test)]
use assert_cmd::prelude::*;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn test_encrypt_command() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // First Command
    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&["--message", "Hello World", "--encrypt"]);
    cmd.assert().success();

    // Second Command
    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&[
        "--message",
        "Mary had a little lamb a litte lamb",
        "--encrypt",
    ]);
    cmd.assert().success();
}

#[test]
fn test_decrypt_command() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // First Command
    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&["--message", "QTrbyq87yaS8jwVFpr6stw=", "--decrypt"]);
    cmd.assert().success();

    // Second Command
    let mut cmd = Command::cargo_bin("aes_cbc").unwrap();
    cmd.args(&["--message", "P0JVe8U8gF4QCBnWnpmKlzI9DGK7Ek4Vm6iTyA0zLb+Pwx+9HLHzv1BYrNPRnGsCBEu5JiJZpBB0KOjxiTh41U2tytK9/t/4AiaXBYVXsG9Z8utU/ZzbPNb1DBTOqzGB", "--decrypt"]);
    cmd.assert().success();
}
