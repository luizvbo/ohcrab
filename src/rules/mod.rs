use crate::shell::Shell;
use core::fmt;

use crate::cli::{command::CorrectedCommand, command::CrabCommand};

mod ag_literal;
mod apt_get;
mod apt_get_search;
mod apt_list_upgradable;
mod apt_upgrade;
mod aws_cli;
mod az_cli;
mod brew_install;
mod brew_link;
mod brew_reinstall;
mod brew_uninstall;
mod brew_update_formula;
mod cargo;
mod cargo_no_command;
mod cat_dir;
mod cd_correction;
mod cd_cs;
mod cd_mkdir;
mod cd_parent;
mod chmod_x;
mod choco_install;
mod composer_not_command;
mod conda_mistype;
mod cp_create_destination;
mod cp_omitting_directory;
mod cpp11;
mod dirty_untar;
mod django_south_ghost;
mod django_south_merge;
mod docker_image_being_used_by_container;
mod docker_login;
mod dry;
mod fix_alt_space;
mod fix_file;
mod git_add;
mod git_add_force;
mod git_bisect_usage;
mod git_branch_0flag;
mod git_branch_delete;
mod git_branch_delete_checked_out;
mod git_branch_exists;
mod git_branch_list;
mod git_checkout;
mod git_clone;
mod git_clone_missing;
mod git_commit_add;
mod git_commit_amend;
mod git_commit_reset;
mod git_diff_no_index;
mod git_diff_staged;
mod git_fix_stash;
mod git_flag_after_filename;
mod git_help_aliased;
mod git_hook_bypass;
mod git_lfs_mistype;
mod git_main_master;
mod git_merge;
mod git_merge_unrelated;
mod git_not_command;
mod git_pull;
mod git_pull_clone;
mod git_pull_uncommitted_changes;
mod git_push;
mod git_push_different_branch_names;
mod git_push_force;
mod git_push_pull;
mod git_push_without_commits;
mod git_rebase_merge_dir;
mod git_rebase_no_changes;
mod git_remote_delete;
mod git_remote_seturl_add;
mod git_rm_local_modifications;
mod git_rm_recursive;
mod git_rm_staged;
mod git_stash;
mod git_stash_pop;
mod git_tag_force;
mod git_two_dashes;
mod go_run;
mod gradle_wrapper;
mod grep_arguments_order;
mod grep_recursive;
mod has_exists_script;
mod heroku_multiple_apps;
mod heroku_not_command;
mod history;
mod hostscli;
mod java;
mod javac;
mod lein_not_task;
mod ln_no_hard_link;
mod ln_s_order;
mod long_form_help;
mod ls_all;
mod ls_lah;
mod man;
mod man_no_space;
mod mercurial;
mod mkdir_p;
mod mvn_no_command;
mod mvn_unknown_lifecycle_phase;
mod nixos_cmd_not_found;
mod no_command;
mod no_such_file;
mod npm_missing_script;
mod npm_run_script;
mod php_s;
mod pip_install;
mod pip_unknown_command;
mod prove_recursively;
mod python_command;
mod python_execute;
mod python_module_error;
mod quotation_marks;
mod rails_migrations_pending;
mod remove_shell_prompt_literal;
mod rm_dir;
mod sl_ls;
mod sudo;
mod sudo_command_from_user_path;
mod tmux;
mod touch;
mod unsudo;

mod utils;

pub fn get_rules() -> Vec<Rule> {
    include!(concat!(env!("OUT_DIR"), "/rules_list.rs"))
}

pub struct Rule {
    name: String,
    enabled_by_default: bool,
    priority: u16,
    requires_output: bool,
    pub match_rule: Box<dyn Fn(&mut CrabCommand, Option<&dyn Shell>) -> bool>,
    get_new_command: fn(&mut CrabCommand, Option<&dyn Shell>) -> Vec<String>,
    side_effect: Option<fn(CrabCommand, Option<&str>)>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Rule {
    fn new(
        name: String,
        enabled_by_default: Option<bool>,
        priority: Option<u16>,
        requires_output: Option<bool>,
        match_rule: Box<dyn Fn(&mut CrabCommand, Option<&dyn Shell>) -> bool>,
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

    // Returns `True` if rule matches the command.
    fn is_match(&self, mut command: CrabCommand, system_shell: &dyn Shell) -> bool {
        let script_only = command.output.is_none();
        if script_only && self.requires_output {
            return false;
        }
        if (self.match_rule)(&mut command, Some(system_shell)) {
            return true;
        }
        false
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

/// Generate a list of corrected commands for the given CrabCommand.
///
/// This function takes a `CrabCommand` as input and iterates through the registered
/// rules, applying each rule's match condition. The list of matching commands is then
/// reorganized and returned.
///
/// * `command`: A `CrabCommand` for which to generate corrected commands.
///
/// # Returns
///
/// A `Vec<CorrectedCommand>` containing the list of corrected commands based on the
/// input `CrabCommand`.
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
