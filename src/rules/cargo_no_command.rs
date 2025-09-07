use super::{utils::match_rule_with_is_app, Rule};
use crate::{cli::command::CrabCommand, shell::Shell};
use regex::Regex;

fn auxiliary_match_rule(command: &CrabCommand) -> bool {
    if let Some(output) = &command.output {
        let output_lower = output.to_lowercase();
        command.script_parts.get(1).is_some()
            && (
                // New format
                (output.contains("error: no such command:") && output.contains("a command with a similar name exists:"))
                // Old format
                || (output_lower.contains("no such subcommand") && output.contains("Did you mean"))
            )
    } else {
        false
    }
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    match_rule_with_is_app(auxiliary_match_rule, command, vec!["cargo"], Some(1))
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    if let Some(output) = &command.output {
        // Regex to handle both "Did you mean `build`?" and "a command with a similar name exists: `build`"
        let re = Regex::new(r"(?:Did you mean|a command with a similar name exists:)\s*`([^`]+)`")
            .unwrap();
        let broken = &command.script_parts[1];
        if let Some(caps) = re.captures(output) {
            if let Some(fix) = caps.get(1) {
                return vec![command.script.replace(broken, fix.as_str())];
            }
        }
    }
    vec![]
}

pub fn get_rule() -> Rule {
    Rule::new(
        "cargo_no_command".to_owned(),
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
    use rstest::rstest;

    const NO_SUCH_SUBCOMMAND_OLD: &str = "No such subcommand\n\n        Did you mean `build`?\n";
    const NO_SUCH_SUBCOMMAND_NEW: &str = "error: no such command: `buid`\n\nhelp: a command with a similar name exists: `build`\n\nhelp: view all installed commands with `cargo --list`\nhelp: find a package to install `buid` with `cargo search cargo-buid`\n";

    #[rstest]
    #[case("cargo buid", NO_SUCH_SUBCOMMAND_OLD, true)]
    #[case("cargo buils", NO_SUCH_SUBCOMMAND_NEW, true)]
    fn test_match(#[case] command: &str, #[case] stdout: &str, #[case] is_match: bool) {
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(match_rule(&mut command, None), is_match);
    }

    #[rstest]
    #[case("cargo buid", NO_SUCH_SUBCOMMAND_OLD, vec!["cargo build"])]
    #[case("cargo buils", NO_SUCH_SUBCOMMAND_NEW, vec!["cargo build"])]
    fn test_get_new_command(
        #[case] command: &str,
        #[case] stdout: &str,
        #[case] expected: Vec<&str>,
    ) {
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(get_new_command(&mut command, None), expected);
    }
}
