use super::{utils::match_rule_with_is_app, Rule};
use crate::{cli::command::CrabCommand, shell::Shell};
use std::path::Path;

pub fn auxiliary_match_rule(command: &CrabCommand) -> bool {
    if let Some(output) = &command.output {
        output.contains("No such file or directory")
            || (output.trim_end().ends_with("Not a directory"))
            || (output.starts_with("cp: directory")
                && output.trim_end().ends_with("does not exist"))
    } else {
        false
    }
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    match_rule_with_is_app(auxiliary_match_rule, command, vec!["cp", "mv"], None)
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    if let Some(dest_path_str) = command.script_parts.last() {
        let dir_to_create = if dest_path_str.ends_with('/') || dest_path_str.ends_with('\\') {
            // Case 1: Destination is a directory, like `bar/qux/`.
            Some(dest_path_str.trim_end_matches(['/', '\\']))
        } else {
            // Case 2: Destination is a file, like `bar/qux/file.txt`.
            Path::new(dest_path_str).parent().and_then(|p| p.to_str())
        };

        if let Some(dir_str) = dir_to_create {
            if !dir_str.is_empty() {
                return vec![system_shell
                    .unwrap()
                    .and(vec![&format!("mkdir -p {}", dir_str), &command.script])];
            }
        }
    }
    vec![]
}

pub fn get_rule() -> Rule {
    Rule::new(
        "cp_create_destination".to_owned(),
        None,
        None,
        None,
        match_rule,
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

    #[rstest]
    #[case("cp", "cp: directory foo does not exist\n", true)]
    #[case("mv", "No such file or directory", true)]
    #[case("cp", "", false)]
    #[case("mv", "", false)]
    #[case("ls", "No such file or directory", false)]
    fn test_match(#[case] script: &str, #[case] output: &str, #[case] is_match: bool) {
        let mut command = CrabCommand::new(script.to_owned(), Some(output.to_owned()), None);
        assert_eq!(match_rule(&mut command, None), is_match);
    }

    #[rstest]
    #[case(
        "cp foo bar/baz",
        "cp: bar/baz: No such file or directory",
        "mkdir -p bar && cp foo bar/baz"
    )]
    #[case(
        "mv foo bar/qux/",
        "mv: bar/qux/: No such file or directory",
        "mkdir -p bar/qux && mv foo bar/qux/"
    )]
    fn test_get_new_command(#[case] script: &str, #[case] output: &str, #[case] expected: &str) {
        let system_shell = Bash {};
        let mut command = CrabCommand::new(script.to_owned(), Some(output.to_owned()), None);
        assert_eq!(
            get_new_command(&mut command, Some(&system_shell)),
            vec![expected]
        );
    }
}
