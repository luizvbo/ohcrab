use super::Rule;
use crate::{cli::command::CrabCommand, shell::Shell};
use regex::Regex;
use std::env;
use std::path::Path;

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

#[derive(Debug, PartialEq)]
struct FileMatch {
    file: String,
    line: String,
}

/// Searches the output for a file path and line number that exists on the filesystem.
fn search(output: &str) -> Option<FileMatch> {
    for pattern in PATTERNS {
        if let Some(captures) = Regex::new(pattern).unwrap().captures(output) {
            if let Some(file_match) = captures.name("file") {
                let file = file_match.as_str();
                if Path::new(file).is_file() {
                    if let Some(line_match) = captures.name("line") {
                        return Some(FileMatch {
                            file: file.to_string(),
                            line: line_match.as_str().to_string(),
                        });
                    }
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
        if let Some(file_match) = search(output) {
            if let Ok(editor) = env::var("EDITOR") {
                let editor_call = format!("{} {} +{}", editor, file_match.file, file_match.line);
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
    use tempfile::tempdir;

    struct TestCase<'a> {
        script: &'a str,
        file: &'a str,
        line: &'a str,
        output: &'a str,
    }

    const TESTS: &[TestCase] = &[
        TestCase {
            script: "gcc a.c",
            file: "a.c",
            line: "3",
            output: "a.c: In function 'main':\na.c:3:1: error: expected expression before '}' token\n }",
        },
        TestCase {
            script: "python a.py",
            file: "a.py",
            line: "2",
            output: "  File \"a.py\", line 2\n      +\n          ^\nSyntaxError: invalid syntax",
        },
        TestCase {
            script: "cargo build",
            file: "src/lib.rs",
            line: "3",
            output: "   Compiling test v0.1.0 (file:///tmp/fix-error/test)\n   src/lib.rs:3:5: 3:6 error: unexpected token: `+`\n   src/lib.rs:3     +\n                    ^\nCould not compile `test`.",
        },
        TestCase {
            script: "node fuck.js",
            file: "fuck.js",
            line: "2",
            output: "{file}:2\nconole.log(arg);\n^\nReferenceError: conole is not defined\n    at {file}:2:5",
        },
        TestCase {
            script: "git st",
            file: ".git/config",
            line: "1",
            output: "fatal: bad config file line 1 in {file}",
        },
        TestCase {
            script: "bash a.sh",
            file: "a.sh",
            line: "2",
            output: "{file}: line 2: foo: command not found",
        },
    ];

    #[rstest]
    fn test_match_and_get_new_command() {
        for test in TESTS {
            let temp_dir = tempdir().unwrap();
            let file_path = temp_dir.path().join(test.file);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(&file_path).unwrap();

            let modified_output = test.output.replace("{file}", file_path.to_str().unwrap());
            let mut command =
                CrabCommand::new(test.script.to_string(), Some(modified_output), None);

            // Test match
            env::set_var("EDITOR", "dummy_editor");
            println!("{:?}", command);
            println!("{:?}", match_rule(&mut command, None));
            assert!(match_rule(&mut command, None));

            // Test get_new_command
            let system_shell = Bash {};
            let expected_cmd = format!(
                "dummy_editor {} +{} && {}",
                file_path.to_str().unwrap(),
                test.line,
                test.script
            );
            assert_eq!(
                get_new_command(&mut command, Some(&system_shell)),
                vec![expected_cmd]
            );

            env::remove_var("EDITOR");
        }
    }

    #[test]
    fn test_no_editor() {
        let test = &TESTS[0];
        env::remove_var("EDITOR");
        let mut command =
            CrabCommand::new(test.script.to_string(), Some(test.output.to_string()), None);
        assert!(!match_rule(&mut command, None));
    }

    #[test]
    fn test_not_file() {
        let test = &TESTS[0];
        env::set_var("EDITOR", "dummy_editor");
        // Don't create the file, so Path::is_file() will be false
        let mut command =
            CrabCommand::new(test.script.to_string(), Some(test.output.to_string()), None);
        assert!(!match_rule(&mut command, None));
        env::remove_var("EDITOR");
    }
}
