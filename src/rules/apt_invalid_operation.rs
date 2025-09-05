use super::{utils::match_rule_with_is_app, Rule};
use crate::{
    cli::command::CrabCommand,
    shell::Shell,
    utils::{get_close_matches, replace_argument},
};
use std::process::Command as ProcessCommand;

fn auxiliary_match_rule(command: &CrabCommand) -> bool {
    if let Some(output) = &command.output {
        output.contains("Invalid operation")
    } else {
        false
    }
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    match_rule_with_is_app(
        auxiliary_match_rule,
        command,
        vec!["apt", "apt-get", "apt-cache"],
        None,
    )
}

fn get_operations(app: &str) -> Vec<String> {
    if let Ok(output) = ProcessCommand::new(app).arg("--help").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if app == "apt" {
                return stdout
                    .lines()
                    .skip_while(|line| !line.starts_with("Basic commands:"))
                    .skip(1)
                    .take_while(|line| !line.is_empty())
                    .filter_map(|line| line.trim().split_whitespace().next().map(String::from))
                    .collect();
            } else {
                return stdout
                    .lines()
                    .skip_while(|line| !line.starts_with("Commands:"))
                    .skip(1)
                    .take_while(|line| !line.is_empty())
                    .filter_map(|line| line.trim().split_whitespace().next().map(String::from))
                    .collect();
            }
        }
    }
    vec![]
}

pub fn get_new_command(
    command: &mut CrabCommand,
    _system_shell: Option<&dyn Shell>,
) -> Vec<String> {
    if let Some(output) = &command.output {
        if let Some(invalid_op) = output.split_whitespace().last() {
            let app = &command.script_parts[0];
            let operations = get_operations(app);
            if let Some(closest_match) = get_close_matches(
                invalid_op,
                &operations.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                Some(1),
                Some(0.6),
            )
            .get(0)
            {
                return vec![replace_argument(&command.script, invalid_op, closest_match)];
            }
        }
    }
    vec![]
}

pub fn get_rule() -> Rule {
    Rule::new(
        "apt_invalid_operation".to_owned(),
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

    #[test]
    fn test_match() {
        let mut command = CrabCommand::new(
            "apt isntall vim".to_owned(),
            Some("E: Invalid operation isntall".to_owned()),
            None,
        );
        assert!(match_rule(&mut command, None));
        let mut command_no_match =
            CrabCommand::new("apt install vim".to_owned(), Some("".to_owned()), None);
        assert!(!match_rule(&mut command_no_match, None));
    }

    #[test]
    fn test_get_new_command() {
        let mut command = CrabCommand::new(
            "apt-get isntall vim".to_owned(),
            Some("E: Invalid operation isntall".to_owned()),
            None,
        );
        assert!(!get_new_command(&mut command, None).is_empty());
    }
}
