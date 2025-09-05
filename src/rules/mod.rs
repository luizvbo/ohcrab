use crate::cli::{command::CorrectedCommand, command::CrabCommand};
use crate::shell::Shell;
use core::fmt;

// Publicly re-export the utils module so other rules can use it
pub mod utils;

// The Rule struct needs to be defined *before* we include the generated
// code that uses it.
pub struct Rule {
    name: String,
    enabled_by_default: bool,
    priority: u16,
    requires_output: bool,
    pub match_rule: fn(&mut CrabCommand, Option<&dyn Shell>) -> bool,
    get_new_command: fn(&mut CrabCommand, Option<&dyn Shell>) -> Vec<String>,
    side_effect: Option<fn(CrabCommand, Option<&str>)>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Rule {
    pub fn new(
        name: String,
        enabled_by_default: Option<bool>,
        priority: Option<u16>,
        requires_output: Option<bool>,
        match_rule: fn(&mut CrabCommand, Option<&dyn Shell>) -> bool,
        get_new_command: fn(&mut CrabCommand, Option<&dyn Shell>) -> Vec<String>,
        side_effect: Option<fn(CrabCommand, Option<&str>)>,
    ) -> Self {
        Self {
            name,
            enabled_by_default: enabled_by_default.unwrap_or(true),
            priority: priority.unwrap_or(1000),
            requires_output: requires_output.unwrap_or(true),
            match_rule,
            get_new_command,
            side_effect,
        }
    }

    fn get_corrected_commands(
        &self,
        command: &mut CrabCommand,
        system_shell: &dyn Shell,
    ) -> Vec<CorrectedCommand> {
        let mut new_commands: Vec<CorrectedCommand> = vec![];
        for (n, new_command) in (self.get_new_command)(command, Some(system_shell))
            .iter()
            .enumerate()
        {
            new_commands.push(CorrectedCommand::new(
                new_command.to_owned(),
                self.side_effect,
                (n as u16 + 1) * self.priority,
            ));
        }
        new_commands
    }
}

// This single line will bring in all `mod` declarations and the `get_rules()` function.
// The compiler will correctly resolve `mod cp_create_destination;` to the file
// `src/rules/cp_create_destination.rs`.
include!(concat!(env!("OUT_DIR"), "/rules.rs"));

// The rest of the functions remain here.
pub fn get_corrected_commands(
    command: &mut CrabCommand,
    system_shell: &dyn Shell,
) -> Vec<CorrectedCommand> {
    let mut corrected_commands: Vec<CorrectedCommand> = vec![];
    for rule in get_rules() {
        if (rule.match_rule)(command, Some(system_shell)) {
            for corrected in rule.get_corrected_commands(command, system_shell) {
                corrected_commands.push(corrected);
            }
        }
    }
    organize_commands(corrected_commands)
}

pub fn organize_commands(mut corrected_commands: Vec<CorrectedCommand>) -> Vec<CorrectedCommand> {
    corrected_commands.sort_by(|a, b| a.priority.cmp(&b.priority));
    corrected_commands.dedup_by(|a, b| a.script.eq(&b.script));
    corrected_commands
}

pub fn match_rule_without_sudo<F>(match_function: F, command: &mut CrabCommand) -> bool
where
    F: Fn(&CrabCommand) -> bool,
{
    if !command.script.starts_with("sudo ") {
        match_function(command)
    } else {
        let new_script = command.script[5..].to_owned();
        match_function(&command.update(Some(new_script), None, None))
    }
}

pub fn get_new_command_without_sudo(
    get_new_command_function: fn(&CrabCommand) -> Vec<String>,
    command: &mut CrabCommand,
) -> Vec<String> {
    if !command.script.starts_with("sudo ") {
        get_new_command_function(command)
    } else {
        let new_script = command.script[5..].to_owned();
        command.script = new_script;
        get_new_command_function(command)
            .iter()
            .map(|cmd| "sudo ".to_owned() + cmd)
            .collect()
    }
}
