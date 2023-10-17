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
    cmd.args(&["--message", "Q9LCh9YvaLPGcXeEMQ8bSWz2nU0FMoqYmqBl0GxX6D1MXj76zHXboHyrDMZHeiR0H2MiDekXUV1FBeZYBUivc++YYzD78RhBMfpP6lq+IttySE0ns/P/xueKX4wY3ln8sBy/b1Zrm52Lun5KqmFhZvHj7d2pwHsz7DiuK37TkHg=", "--decrypt"]);
    cmd.assert().success();
}
