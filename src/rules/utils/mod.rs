use crate::{cli::command::CrabCommand, shell::Shell};
use std::path::Path;

pub mod git;
pub mod npm;
pub mod parameterized_tests;

/// Matches a rule with a given command if it is an application.
///
/// # Arguments
///
/// * `func` - Match function that takes a `CrabCommand` and returns a boolean.
/// * `command` - A reference to a `CrabCommand` instance.
/// * `app_names` - A vector of application names to check against.
/// * `at_least` - An optional usize that specifies the minimum number of script parts.
///
/// # Returns
///
/// * `bool` - Returns true if the command matches the rule and is an application, false otherwise.
pub fn match_rule_with_is_app<F>(
    func: F,
    command: &CrabCommand,
    app_names: Vec<&str>,
    at_least: Option<usize>,
) -> bool
where
    F: Fn(&CrabCommand) -> bool,
{
    if is_app(command, &app_names, at_least) {
        func(command)
    } else {
        false
    }
}

/// Checks if a given command is an application.
///
/// # Arguments
///
/// * `command` - A reference to a `CrabCommand` instance.
/// * `app_names` - A vector of application names to check against.
/// * `at_least` - An optional usize that specifies the minimum number of script parts.
///
/// # Returns
///
/// * `bool` - Returns true if the command is an application, false otherwise.
fn is_app(command: &CrabCommand, app_names: &[&str], at_least: Option<usize>) -> bool {
    let at_least = at_least.unwrap_or(0);
    if command.script_parts.len() > at_least {
        let app_name = Path::new(&command.script_parts[0])
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("");
        return app_names.contains(&app_name);
    }
    false
}

/// A factory function that creates a `match_rule` closure for a specific application.
///
/// This is a higher-order function that acts as an ergonomic helper, similar to
/// the `@for_app` decorator in `thefuck`. It wraps the provided core `rule_logic`
/// in a check that first verifies if the command's executable matches one of the
/// specified `app_names`. This avoids boilerplate code in every rule that is
/// specific to one or more applications.
///
/// # Arguments
///
/// * `app_names` - A vector of string slices representing the application names
///   (e.g., `vec!["git", "hub"]`) that the rule should match. The check is performed
///   on the base name of the command executable.
/// * `at_least` - An optional `usize` specifying the minimum number of parts
///   (including the command itself) that the script must have for the rule to match.
///   For example, `Some(2)` would be appropriate for a rule matching `brew install`.
/// * `rule_logic` - A function pointer to the core matching logic. This function
///   is only executed if the application name and `at_least` checks pass. It
///   takes a `&CrabCommand` and returns `true` if the rule is a match.
///
/// # Returns
///
/// A `Box<dyn Fn(...) -> bool>`, which is a boxed closure. This closure has the
/// exact signature required by the `match_rule` field in a `Rule` struct and
/// can be used directly in its constructor.
///
/// # Example
///
/// ```rust
/// // in some_rule.rs
/// use crate::rules::{Rule, utils};
/// use crate::cli::command::CrabCommand;
/// use crate::shell::Shell;
///
/// // The core logic for the rule, which doesn't need to check the app name.
/// fn match_logic(command: &CrabCommand) -> bool {
///     if let Some(output) = &command.output {
///         output.contains("No such file or directory")
///     } else {
///         false
///     }
/// }
///
/// // A placeholder get_new_command function for the example.
/// fn get_new_command(command: &mut CrabCommand, system_shell: Option<&dyn Shell>) -> Vec<String> {
///     vec!["touch new_file".to_string()]
/// }
///
/// pub fn get_rule() -> Rule {
///     Rule::new(
///         "touch_rule".to_owned(),
///         None,
///         None,
///         None,
///         // The factory function creates the final match_rule.
///         utils::for_app(vec!["touch"], None, match_logic),
///         get_new_command,
///         None,
///     )
/// }
/// ```
pub fn is_app_match_rule<'a>(
    app_names: Vec<&'a str>,
    at_least: Option<usize>,
    rule_logic: fn(&CrabCommand) -> bool,
) -> Box<dyn Fn(&mut CrabCommand, Option<&dyn Shell>) -> bool + 'a> {
    Box::new(move |command, _| {
        if is_app(command, &app_names, at_least) {
            rule_logic(command)
        } else {
            false
        }
    })
}
