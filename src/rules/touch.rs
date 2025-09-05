use super::Rule;
use crate::{cli::command::CrabCommand, rules::utils::match_rule_with_is_app, shell::Shell};
use regex::Regex;
use std::path::Path;

fn auxiliary_match_rule(command: &CrabCommand) -> bool {
    if let Some(output) = &command.output {
        output.contains("touch: cannot touch") && output.contains("No such file or directory")
    } else {
        false
    }
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    match_rule_with_is_app(auxiliary_match_rule, command, vec!["touch"], None)
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    if let Some(output) = &command.output {
        let re = Regex::new(r"touch: cannot touch '([^']+)': No such file or directory").unwrap();
        if let Some(caps) = re.captures(output) {
            if let Some(file_path) = caps.get(1) {
                if let Some(parent_dir) = Path::new(file_path.as_str()).parent() {
                    if let Some(dir_str) = parent_dir.to_str() {
                        return vec![system_shell
                            .unwrap()
                            .and(vec![&format!("mkdir -p {}", dir_str), &command.script])];
                    }
                }
            }
        }
    }
    vec![]
}

pub fn get_rule() -> Rule {
    Rule::new(
        "touch".to_owned(),
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

    #[test]
    fn test_match() {
        let mut command = CrabCommand::new(
            "touch a/b/c".to_owned(),
            Some("touch: cannot touch 'a/b/c': No such file or directory".to_owned()),
            None,
        );
        assert!(match_rule(&mut command, None));
        let mut command_no_match =
            CrabCommand::new("touch a/b/c".to_owned(), Some("".to_owned()), None);
        assert!(!match_rule(&mut command_no_match, None));
    }

    #[test]
    fn test_get_new_command() {
        let mut command = CrabCommand::new(
            "touch a/b/c".to_owned(),
            Some("touch: cannot touch 'a/b/c': No such file or directory".to_owned()),
            None,
        );
        let system_shell = Bash {};
        assert_eq!(
            get_new_command(&mut command, Some(&system_shell)),
            vec!["mkdir -p a/b && touch a/b/c"]
        );
    }
}
