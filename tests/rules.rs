use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::tempdir;

// Helper to get the binary command in an isolated environment
fn ohcrab() -> Command {
    let mut cmd = Command::cargo_bin("ohcrab").unwrap();
    // Isolate tests from the user's shell history and other configs
    cmd.env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default());

    // Preserve rustup and cargo home so cargo can find its toolchain.
    // This is crucial for tests that invoke `cargo`.
    if let Ok(rustup_home) = std::env::var("RUSTUP_HOME") {
        cmd.env("RUSTUP_HOME", rustup_home);
    }
    if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
        cmd.env("CARGO_HOME", cargo_home);
    }

    // By setting a temporary HOME, we prevent ohcrab from finding
    // ~/.bash_history, ~/.zsh_history, ~/.gitconfig, etc.
    cmd.env("HOME", tempdir().unwrap().path());
    cmd
}

// Helper to set up a temporary git repository
fn setup_git_repo() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output()
        .expect("failed to initialize git repo");

    fs::write(repo_path.join("file.txt"), "content").unwrap();

    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(repo_path)
        .output()
        .expect("failed to git add");

    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(repo_path)
        .output()
        .expect("failed to git commit");

    temp_dir
}

#[test]
fn test_rule_apt_get_search() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("apt-get")
        .arg("search")
        .arg("vim")
        .assert()
        .success()
        .stdout(predicate::str::contains("apt-cache search vim"));
}

#[test]
fn test_rule_cargo_no_command() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("cargo")
        .arg("buid")
        .assert()
        .success()
        .stdout(predicate::str::contains("cargo build"));
}

#[test]
fn test_rule_cat_dir() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path().join("mydir");
    fs::create_dir(&dir_path).unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("cat")
        .arg("mydir")
        .assert()
        .success()
        .stdout(predicate::str::contains("ls mydir"));
}

#[test]
fn test_rule_cd_cs() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("cs")
        .arg("/tmp")
        .assert()
        .success()
        .stdout(predicate::str::contains("cd /tmp"));
}

#[test]
fn test_rule_cd_mkdir() {
    let temp_dir = tempdir().unwrap();
    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("cd")
        .arg("non_existent_dir")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "mkdir -p non_existent_dir && cd non_existent_dir",
        ));
}

#[test]
fn test_rule_cd_parent() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("cd..")
        .assert()
        .success()
        .stdout(predicate::str::contains("cd .."));
}

#[test]
#[cfg(unix)]
fn test_rule_chmod_x() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("script.sh");
    fs::write(&file_path, "#!/bin/sh\necho hello").unwrap();
    // No execute permissions by default

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("./script.sh")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "chmod +x script.sh && ./script.sh",
        ));
}

#[test]
fn test_rule_cp_create_destination() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    fs::write(&file_path, "content").unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("cp")
        .arg("file.txt")
        .arg("new_dir/file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "mkdir -p new_dir && cp file.txt new_dir/file.txt",
        ));
}

#[test]
fn test_rule_dry() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("git")
        .arg("git")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("git status"));
}

#[test]
fn test_rule_git_not_command() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("git")
        .arg("brnch")
        .assert()
        .success()
        .stdout(predicate::str::contains("git branch"));
}

#[test]
fn test_rule_go_run() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("main.go");
    fs::write(&file_path, "package main\nfunc main() {}").unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("go")
        .arg("run")
        .arg("main")
        .assert()
        .success()
        .stdout(predicate::str::contains("go run main.go"));
}

#[test]
fn test_rule_grep_arguments_order() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    fs::write(&file_path, "hello world").unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("grep")
        .arg("file.txt")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("grep hello file.txt"));
}

#[test]
#[cfg(unix)]
fn test_rule_ln_no_hard_link() {
    let temp_dir = tempdir().unwrap();
    let dir_to_link = temp_dir.path().join("dir_to_link");
    fs::create_dir(&dir_to_link).unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("ln")
        .arg("dir_to_link")
        .arg("my_link")
        .assert()
        .success()
        .stdout(predicate::str::contains("ln -s dir_to_link my_link"));
}

#[test]
fn test_rule_ls_lah() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("ls -lah"));
}

#[test]
fn test_rule_mkdir_p() {
    let temp_dir = tempdir().unwrap();
    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("mkdir")
        .arg("a/b/c")
        .assert()
        .success()
        .stdout(predicate::str::contains("mkdir -p a/b/c"));
}

#[test]
fn test_rule_no_command() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("gti")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("git status"));
}

#[test]
#[cfg(unix)]
fn test_rule_python_command() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("script.py");
    // Script without execute permissions
    fs::write(&file_path, "print('hello')").unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("./script.py")
        .assert()
        .success()
        .stdout(predicate::str::contains("python ./script.py"));
}

#[test]
fn test_rule_rm_dir() {
    let temp_dir = tempdir().unwrap();
    let dir_to_remove = temp_dir.path().join("dir_to_remove");
    fs::create_dir(&dir_to_remove).unwrap();

    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("rm")
        .arg("dir_to_remove")
        .assert()
        .success()
        .stdout(predicate::str::contains("rm -rf dir_to_remove"));
}

#[test]
#[cfg(unix)]
fn test_rule_sudo() {
    // This command attempts to write to a protected directory (/etc),
    // which should reliably trigger a "Permission denied" error.
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("touch")
        .arg("/etc/testfile")
        .assert()
        .success()
        .stdout(predicate::str::contains("sudo touch /etc/testfile"));
}

#[test]
fn test_rule_git_branch_exists() {
    let temp_dir = setup_git_repo();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .args(["branch", "existing_branch"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    ohcrab()
        .current_dir(repo_path)
        .arg("--select-first")
        .arg("--")
        .arg("git")
        .arg("branch")
        .arg("existing_branch")
        .assert()
        .success()
        .stdout(predicate::str::contains("git checkout existing_branch"));
}

#[test]
fn test_rule_touch_create_missing_dir() {
    let temp_dir = tempdir().unwrap();
    ohcrab()
        .current_dir(temp_dir.path())
        .arg("--select-first")
        .arg("--")
        .arg("touch")
        .arg("a/b/c")
        .assert()
        .success()
        .stdout(predicate::str::contains("mkdir -p a/b && touch a/b/c"));
}

#[test]
fn test_rule_git_add() {
    let temp_dir = setup_git_repo();
    let repo_path = temp_dir.path();
    // This file exists on the filesystem but is not tracked by git
    fs::write(repo_path.join("new_file.txt"), "new content").unwrap();

    ohcrab()
        .current_dir(repo_path)
        .arg("--select-first")
        .arg("--")
        // Use a command that reliably produces the "pathspec" error
        // for an existing but untracked file.
        .arg("git")
        .arg("checkout")
        .arg("new_file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            // The correction should be to add the file, then run the original command.
            // The filename might be quoted by the rule logic, so we don't check for quotes.
            "git add -- new_file.txt && git checkout new_file.txt",
        ));
}

#[test]
fn test_rule_git_pull_no_tracking() {
    // Setup a "remote" bare repository
    let remote_dir = tempdir().unwrap();
    StdCommand::new("git")
        .args(["init", "--bare"])
        .current_dir(remote_dir.path())
        .output()
        .expect("Failed to create bare repo");

    // Setup a "local" repository
    let local_dir = setup_git_repo();
    let local_path = local_dir.path();

    // Add the remote and push the main branch
    StdCommand::new("git")
        .args([
            "remote",
            "add",
            "origin",
            remote_dir.path().to_str().unwrap(),
        ])
        .current_dir(local_path)
        .output()
        .expect("Failed to add remote");

    StdCommand::new("git")
        .args(["push", "-u", "origin", "master"])
        .current_dir(local_path)
        .output()
        .expect("Failed to push to remote");

    // Create a new branch without tracking info
    StdCommand::new("git")
        .args(["checkout", "-b", "new-feature"])
        .current_dir(local_path)
        .output()
        .expect("Failed to create new branch");

    ohcrab()
        .current_dir(local_path)
        .arg("--select-first")
        .arg("--")
        .arg("git")
        .arg("pull")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "git branch --set-upstream-to=origin/new-feature new-feature && git pull",
        ));
}

#[test]
#[ignore]
fn test_rule_docker_login() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("docker")
        .arg("pull")
        .arg("private-repo/private-image")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "docker login && docker pull private-repo/private-image",
        ));
}

#[test]
#[ignore]
fn test_rule_brew_install() {
    ohcrab()
        .arg("--select-first")
        .arg("--")
        .arg("brew")
        .arg("install")
        .arg("giss")
        .assert()
        .success()
        .stdout(predicate::str::contains("brew install gist"));
}
