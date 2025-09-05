use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_alias_generation() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--alias=crabalias")
        .assert()
        .success()
        .stdout(predicate::str::contains("crabalias"));
}

#[test]
fn test_command_correction_suggestion() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        // Use a command with a single, correctable error
        .arg("git")
        .arg("brnch")
        .assert()
        .success()
        // Assert the correct, single-step suggestion
        .stdout(predicate::str::contains("git branch"));
}

#[test]
fn test_debug_output() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--debug")
        .arg("--")
        // Use the same robust command here
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
/// Tests if a command with quoted arguments is corrected properly.
/// This ensures that the argument parsing logic (shlex) correctly handles quotes and spaces.
#[test]
fn test_command_with_quotes() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        // Pass the entire command as a single argument to simulate shell behavior
        .arg("git comit -m \"a message with spaces\"")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "git commit -m \"a message with spaces\"",
        ));
}

// Command Corrected by `no_command` Rule
/// Tests a command with a typo in the executable name itself.
/// This relies on the `no_command` rule, which has a higher priority, to suggest a correction
/// from the list of available system executables.
#[test]
fn test_executable_typo_correction() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        .arg("gti") // Typo for "git"
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("git status"));
}

// Rule that Creates a Directory (`no_such_file`)
/// Tests the `no_such_file` rule for `cp`, which fails when the destination directory doesn't exist.
/// The corrected command should first create the directory and then execute the original command.
#[test]
fn test_command_creating_directory() {
    // Setup a temporary directory for the test
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    std::fs::write(&file_path, "content").unwrap();

    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--debug") // Keep debug for useful output if it fails
        .arg("--")
        .arg("cp")
        .arg("file.txt")
        .arg("non_existent_dir/file.txt")
        .assert()
        .success()
        // This is the corrected assertion:
        .stdout(predicate::str::contains(
            "mkdir -p non_existent_dir && cp file.txt non_existent_dir/file.txt",
        ));
}

// Piped Command Correction
/// Tests if `ohcrab` can correct the failing part of a piped command.
/// The `no_command` rule should correct `lss` to `ls`.
#[test]
fn test_piped_command() {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    cmd.arg("--select-first")
        .arg("--")
        .arg("gitt status | grep foo") // `gitt` is a less ambiguous typo for `git`
        .assert()
        .success()
        .stdout(predicate::str::contains("git status | grep foo"));
}
