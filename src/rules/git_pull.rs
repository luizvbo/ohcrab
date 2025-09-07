use crate::{
    cli::command::CrabCommand,
    rules::{
        utils::git::{get_new_command_with_git_support, match_rule_with_git_support},
        Rule,
    },
    shell::Shell,
};
use regex::Regex;

fn auxiliary_match_rule(command: &CrabCommand) -> bool {
    if let Some(stdout) = &command.output {
        command.script.contains("pull") && stdout.contains("set-upstream")
    } else {
        false
    }
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    match_rule_with_git_support(auxiliary_match_rule, command)
}

fn auxiliary_get_new_command(
    command: &CrabCommand,
    system_shell: Option<&dyn Shell>,
) -> Vec<String> {
    if let Some(stdout) = &command.output {
        // Regex to find the full suggested command line, e.g., "git branch --set-upstream-to=origin/master master"
        let re = Regex::new(r"(git branch --set-upstream-to=\S+ \S+)").unwrap();
        if let Some(caps) = re.captures(stdout) {
            if let Some(set_upstream) = caps.get(1) {
                return vec![system_shell
                    .unwrap()
                    .and(vec![set_upstream.as_str(), &command.script])];
            }
        }
    }
    Vec::<String>::new()
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    get_new_command_with_git_support(auxiliary_get_new_command, command, system_shell)
}

pub fn get_rule() -> Rule {
    Rule::new(
        "git_pull".to_owned(),
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
    use crate::{cli::command::CrabCommand, shell::Bash};
    use rstest::rstest;

    const OUTPUT: &str = r#"There is no tracking information for the current branch.
Please specify which branch you want to merge with.
See git-pull(1) for details

    git pull <remote> <branch>

If you wish to set tracking information for this branch you can do so with:

    git branch --set-upstream-to=origin/master master


"#;

    #[rstest]
    #[case("git pull", OUTPUT, true)]
    #[case("git pull", "", false)]
    #[case("ls", OUTPUT, false)]
    fn test_match(#[case] command: &str, #[case] stdout: &str, #[case] is_match: bool) {
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(match_rule(&mut command, None), is_match);
    }

    #[rstest]
    #[case("git pull", OUTPUT, vec!["git branch --set-upstream-to=origin/master master && git pull"])]
    fn test_get_new_command(
        #[case] command: &str,
        #[case] stdout: &str,
        #[case] expected: Vec<&str>,
    ) {
        let system_shell = Bash {};
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(get_new_command(&mut command, Some(&system_shell)), expected);
    }
}
