use assert_cmd::Command;
use predicates::prelude::*;

// Add this line to include the new test module
mod rules;

#[test]
fn test_alias_generation() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--alias=crabalias")
        .assert()
        .success()
        .stdout(predicate::str::contains("crabalias"));
}

#[test]
fn test_debug_output() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--debug")
        .arg("--")
        .arg("git")
        .arg("brnch")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Retrieved command(s):")
                .and(predicate::str::contains("git branch")),
        );
}

// Command with Quoted Arguments
#[test]
fn test_command_with_quotes() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        .arg("git comit -m \"a message with spaces\"")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "git commit -m \"a message with spaces\"",
        ));
}

// Piped Command Correction
#[test]
fn test_piped_command() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        .arg("gitt status | grep foo")
        .assert()
        .success()
        .stdout(predicate::str::contains("git status | grep foo"));
}
