use super::Rule;
use crate::{cli::command::CrabCommand, shell::Shell};
use regex::{Captures, Regex};
use std::env;
use std::path::Path;
use std::sync::OnceLock;

const PATTERNS: &[&str] = &[
    // js, node:
    r#"(?m)^    at (?P<file>[^:\n]+):(?P<line>[0-9]+):(?P<col>[0-9]+)"#,
    // cargo:
    r#"(?m)^   (?P<file>[^:\n]+):(?P<line>[0-9]+):(?P<col>[0-9]+)"#,
    // python, thefuck:
    r#"(?m)^  File "(?P<file>[^:\n]+)", line (?P<line>[0-9]+)"#,
    // awk:
    r#"(?m)^awk: (?P<file>[^:\n]+):(?P<line>[0-9]+):"#,
    // git
    r#"(?m)^fatal: bad config file line (?P<line>[0-9]+) in (?P<file>[^:\n]+)"#,
    // llc
    r#"(?m)^llc: (?P<file>[^:\n]+):(?P<line>[0-9]+):(?P<col>[0-9]+):"#,
    // lua
    r#"(?m)^lua: (?P<file>[^:\n]+):(?P<line>[0-9]+):"#,
    // fish
    r#"(?m)^(?P<file>[^:\n]+) \(line (?P<line>[0-9]+)\):"#,
    // bash, sh, ssh:
    r#"(?m)^(?P<file>[^:\n]+): line (?P<line>[0-9]+): "#,
    // cargo, clang, gcc, go, pep8, rustc:
    r#"(?m)^(?P<file>[^:\n]+):(?P<line>[0-9]+):(?P<col>[0-9]+)"#,
    // ghc, make, ruby, zsh:
    r#"(?m)^(?P<file>[^:\n]+):(?P<line>[0-9]+):"#,
    // perl
    r#"(?m)^at (?P<file>[^:\n]+) line (?P<line>[0-9]+)"#,
];

static COMPILED_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn get_patterns() -> &'static Vec<Regex> {
    COMPILED_PATTERNS.get_or_init(|| {
        PATTERNS
            .iter()
            .map(|pattern| Regex::new(pattern).unwrap())
            .collect()
    })
}

#[derive(Debug, PartialEq)]
struct FileMatch {
    file: String,
    line: String,
}

/// Searches the output for a file path and line number that exists on the filesystem.
fn search(output: &str) -> Option<Captures<'_>> {
    for regex in get_patterns() {
        if let Some(captures) = regex.captures(output) {
            if let Some(file_match) = captures.name("file") {
                if Path::new(file_match.as_str()).is_file() {
                    return Some(captures);
                }
            }
        }
    }
    None
}

pub fn match_rule(command: &mut CrabCommand, _system_shell: Option<&dyn Shell>) -> bool {
    if env::var("EDITOR").is_err() {
        return false;
    }
    if let Some(output) = &command.output {
        search(output).is_some()
    } else {
        false
    }
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    if let Some(output) = &command.output {
        if let Some(captures) = search(output) {
            if let (Ok(editor), Some(file), Some(line)) = (
                env::var("EDITOR"),
                captures.name("file"),
                captures.name("line"),
            ) {
                let editor_call = format!("{} {} +{}", editor, file.as_str(), line.as_str());
                return vec![system_shell
                    .unwrap()
                    .and(vec![&editor_call, &command.script])];
            }
        }
    }
    vec![]
}

pub fn get_rule() -> Rule {
    Rule::new(
        "fix_file".to_owned(),
        None,
        None,
        None,
        Box::new(match_rule),
        get_new_command,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::{get_new_command, match_rule};
    use crate::cli::command::CrabCommand;
    use crate::shell::Bash;
    use rstest::rstest;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[rstest]
    #[case(
        "gcc a.c",
        "a.c",
        "3",
        "a.c: In function 'main':\na.c:3:1: error: expected expression before '}' token\n }"
    )]
    #[case(
        "python a.py",
        "a.py",
        "2",
        "  File \"a.py\", line 2\n      +\n          ^\nSyntaxError: invalid syntax"
    )]
    #[case("cargo build", "src/lib.rs", "3", "   Compiling test v0.1.0 (file:///tmp/fix-error/test)\n   src/lib.rs:3:5: 3:6 error: unexpected token: `+`\n   src/lib.rs:3     +\n                    ^\nCould not compile `test`.")]
    #[case(
        "node fuck.js",
        "fuck.js",
        "2",
        "{file}:2\nconole.log(arg);\n^\nReferenceError: console is not defined\n    at {file}:2:5"
    )]
    #[case(
        "git st",
        ".git/config",
        "1",
        "fatal: bad config file line 1 in {file}"
    )]
    #[case("bash a.sh", "a.sh", "2", "{file}: line 2: foo: command not found")]
    fn test_match_and_get_new_command(
        #[case] script: &str,
        #[case] file: &str,
        #[case] line: &str,
        #[case] output: &str,
    ) {
        let temp_dir = tempdir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let file_path = temp_dir.path().join(file);
        if let Some(parent) = file_path.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        fs::File::create(&file_path).unwrap();

        let modified_output = output.replace("{file}", file);
        let mut command = CrabCommand::new(script.to_string(), Some(modified_output), None);

        // Test match
        env::set_var("EDITOR", "dummy_editor");
        assert!(
            match_rule(&mut command, None),
            "Match failed for script: '{script}'"
        );

        // Test get_new_command
        let system_shell = Bash {};
        let expected_cmd = format!("dummy_editor {file} +{line} && {script}");
        assert_eq!(
            get_new_command(&mut command, Some(&system_shell)),
            vec![expected_cmd]
        );

        env::remove_var("EDITOR");
    }

    #[test]
    fn test_no_editor() {
        let mut command = CrabCommand::new("gcc a.c".to_string(), Some("...".to_string()), None);
        env::remove_var("EDITOR");
        assert!(!match_rule(&mut command, None));
    }

    #[test]
    fn test_not_file() {
        let mut command = CrabCommand::new(
            "gcc a.c".to_string(),
            Some("a.c:3:1: error...".to_string()),
            None,
        );
        env::set_var("EDITOR", "dummy_editor");
        // Don't create the file, so Path::is_file() will be false
        assert!(!match_rule(&mut command, None));
        env::remove_var("EDITOR");
    }
}
