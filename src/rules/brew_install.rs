use crate::{cli::command::CrabCommand, shell::Shell};
use regex::Regex;

use super::Rule;

fn get_suggestions(str: String) -> Vec<String> {
    str.replace(" or ", ", ")
        .split(", ")
        .map(|s| s.to_string())
        .collect()
}

pub fn match_rule(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> bool {
    if let Some(stdout) = &command.output {
        command.script.contains("install")
            && stdout.contains("No available formula")
            && stdout.contains("Did you mean")
    } else {
        false
    }
}

pub fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
    let re = Regex::new(
        "Warning: No available formula with the name \"(?:[^\"]+)\". Did you mean (.+)\\?",
    )
    .unwrap();
    let stdout = &command.output.as_ref().unwrap();
    let caps = re.captures(stdout).unwrap();
    let suggestions = get_suggestions(caps.get(1).map_or("", |m| m.as_str()).to_owned());
    suggestions
        .iter()
        .map(|formula| format!("brew install {formula}"))
        .collect()
}

pub fn get_rule() -> Rule {
    Rule::new(
        "brew_install".to_owned(),
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
    use super::{get_new_command, get_suggestions, match_rule};
    use crate::cli::command::CrabCommand;
    use rstest::rstest;

    const BREW_NO_AVAILABLE_FORMULA_ONE: &str =
        "Warning: No available formula with the name \"giss\". Did you mean gist?";
    const BREW_NO_AVAILABLE_FORMULA_TWO: &str = "Warning: No available formula with the name \"elasticserar\". Did you mean elasticsearch or elasticsearch@6?";
    const BREW_NO_AVAILABLE_FORMULA_THREE: &str =
        "Warning: No available formula with the name \"gitt\". Did you mean git, gitg or gist?";
    const BREW_INSTALL_NO_ARGUMENT: &str =
        "Install a formula or cask. Additional options specific to a formula may be";
    const BREW_ALREADY_INSTALLED: &str = "Warning: git-2.3.5 already installed";

    #[rstest]
    #[case("brew install giss", BREW_NO_AVAILABLE_FORMULA_ONE, true)]
    #[case("brew install elasticserar", BREW_NO_AVAILABLE_FORMULA_TWO, true)]
    #[case("brew install gitt", BREW_NO_AVAILABLE_FORMULA_THREE, true)]
    #[case("brew install git", BREW_ALREADY_INSTALLED, false)]
    #[case("brew install", BREW_INSTALL_NO_ARGUMENT, false)]
    fn test_match(#[case] command: &str, #[case] stdout: &str, #[case] is_match: bool) {
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(match_rule(&mut command, None), is_match);
    }

    #[rstest]
    #[case("brew install giss", BREW_NO_AVAILABLE_FORMULA_ONE, vec!["brew install gist"])]
    #[case("brew install elasticsear", BREW_NO_AVAILABLE_FORMULA_TWO, vec!["brew install elasticsearch", "brew install elasticsearch@6"])]
    #[case("brew install gitt", BREW_NO_AVAILABLE_FORMULA_THREE, vec!["brew install git", "brew install gitg", "brew install gist"])]
    fn test_get_new_command(
        #[case] command: &str,
        #[case] stdout: &str,
        #[case] expected: Vec<&str>,
    ) {
        let mut command = CrabCommand::new(command.to_owned(), Some(stdout.to_owned()), None);
        assert_eq!(get_new_command(&mut command, None), expected);
    }

    #[test]
    fn test_suggestions() {
        assert_eq!(get_suggestions("one".to_owned()), ["one"]);
        assert_eq!(get_suggestions("one or two".to_owned()), ["one", "two"]);
        assert_eq!(
            get_suggestions("one, two or three".to_owned()),
            ["one", "two", "three"]
        );
    }
}
