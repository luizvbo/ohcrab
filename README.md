# OhCrab! 🦀

`ohcrab` is a port of the well known CLI tool
[`thefuck`](https://github.com/nvbn/thefuck) to our beloved Rust language.

![ohcrab in action](https://raw.githubusercontent.com/luizvbo/oh-crab/main/resources/ohcrab-example.gif)

## Table of contents

1. [Installation](#installation)
1. [Usage](#usage)
1. [Contributing](#contributing)
1. [Road map](#road-map)

## Installation

You can install `ohcrab` using one of the methods below. The recommended way is using the installer script.

### Installer Script (macOS & Linux)

You can install `ohcrab` by running the following command in your terminal:

```shell
sh -c "$(curl -fsSL https://raw.githubusercontent.com/luizvbo/oh-crab/main/install.sh)"
```

The script will automatically detect your operating system and architecture, download the latest binary, and install it to a local directory.

### Using Cargo

If you have Rust and Cargo installed, you can build and install `ohcrab` from source:

```shell
cargo install ohcrab
```

### Adding ohcrab to your environment

After installation, you need to add the `ohcrab` alias to your shell's configuration file. We currently support `bash` and `zsh`.

Run the appropriate command for your shell:

-   For **bash**:
    ```shell
    eval $(ohcrab --shell bash)
    ```

-   For **zsh**:
    ```shell
    eval $(ohcrab --shell zsh)
    ```

To make the alias available in all your terminal sessions, add the `eval` command to your `.bash_profile`, `.bashrc`, `.zshrc`, or other startup script.

### Changing the alias

The default alias is `crab`. You can change it by using the `--alias` flag when generating the shell function. For example, to use `shinycrab` in `zsh`, run:

```shell
eval $(ohcrab --shell zsh --alias shinycrab)
```

## Usage

In the terminal, after typing the wrong command, type `crab` (or the alias you chose). It will show a menu to choose the correct command from.

## Contributing

If you like `ohcrab` and/or want to learn `rust`, you can contribute by adding
new rules or improving the crate.

## Road map

- [x] Add `sudo` support
- [x] Distribute binaries for Linux, MacOs and Windows.
- [x] Make a short screen record of its functioning
- [ ] Inform the user which shell type is being used when the `ohcrab` shell
      function is generated.
- [ ] Add support to user shell aliases.
- [ ] Add support to PowerShell
- [ ] Reduce number/size of dependencies
- [ ] Make it available via package managers
- [ ] Benchmark against thefuck
- [ ] Add an interactive menu to setup ohcrab (see issue
      [#74](https://github.com/luizvbo/oh-crab/issues/74))
- [ ] Add integration tests (see issue [#81](https://github.com/luizvbo/oh-crab/issues/81))

### Rules

<details>
  <summary>Implemented</summary>

- [x] ag_literal
- [x] apt_get
- [x] apt_get_search
- [x] apt_list_upgradable
- [x] apt_upgrade
- [x] aws_cli
- [x] az_cli
- [x] brew_install
- [x] brew_link
- [x] brew_reinstall
- [x] brew_uninstall
- [x] brew_update_formula
- [x] cargo
- [x] cargo_no_command
- [x] cat_dir
- [x] cd_correction
- [x] cd_cs
- [x] cd_mkdir
- [x] cd_parent
- [x] chmod_x
- [x] choco_install
- [x] composer_not_command
- [x] conda_mistype
- [x] cp_create_destination
- [x] cp_omitting_directory
- [x] cpp11
- [x] dirty_untar
- [x] django_south_ghost
- [x] django_south_merge
- [x] docker_image_being_used_by_container
- [x] docker_login
- [x] dry
- [x] fix_alt_space
- [x] git_add
- [x] git_add_force
- [x] git_bisect_usage
- [x] git_branch_0flag
- [x] git_branch_delete
- [x] git_branch_delete_checked_out
- [x] git_branch_exists
- [x] git_branch_list
- [x] git_checkout
- [x] git_clone_git_clone
- [x] git_clone_missing
- [x] git_commit_add
- [x] git_commit_amend
- [x] git_commit_reset
- [x] git_diff_no_index
- [x] git_diff_staged
- [x] git_fix_stash
- [x] git_flag_after_filename
- [x] git_help_aliased
- [x] git_hook_bypass
- [x] git_lfs_mistype
- [x] git_main_master
- [x] git_merge
- [x] git_merge_unrelated
- [x] git_not_command
- [x] git_pull
- [x] git_pull_clone
- [x] git_pull_uncommitted_changes
- [x] git_push
- [x] git_push_different_branch_names
- [x] git_push_force
- [x] git_push_pull
- [x] git_push_without_commits
- [x] git_rebase_merge_dir
- [x] git_rebase_no_changes
- [x] git_remote_delete
- [x] git_remote_seturl_add
- [x] git_rm_local_modifications
- [x] git_rm_recursive
- [x] git_rm_staged
- [x] git_stash
- [x] git_stash_pop
- [x] git_tag_force
- [x] git_two_dashes
- [x] go_run
- [x] gradle_wrapper
- [x] grep_arguments_order
- [x] grep_recursive
- [x] has_exists_script
- [x] heroku_multiple_apps
- [x] heroku_not_command
- [x] history
- [x] hostscli
- [x] java
- [x] javac
- [x] lein_not_task
- [x] ln_no_hard_link
- [x] ln_s_order
- [x] long_form_help
- [x] ls_all
- [x] ls_lah
- [x] man
- [x] man_no_space
- [x] mercurial
- [x] mkdir_p
- [x] mvn_no_command
- [x] mvn_unknown_lifecycle_phase
- [x] nixos_cmd_not_found
- [x] no_command
- [x] no_such_file
- [x] npm_missing_script
- [x] npm_run_script
- [x] php_s
- [x] pip_install
- [x] pip_unknown_command
- [x] prove_recursively
- [x] python_command
- [x] python_execute
- [x] python_module_error
- [x] quotation_marks
- [x] rails_migrations_pending
- [x] remove_shell_prompt_literal
- [x] rm_dir
- [x] sudo
- [x] sudo_command_from_user_path
- [x] tmux
- [x] unsudo

</details>

<details>
  <summary>To be Implemented</summary>

- [ ] adb_unknown_command
- [ ] apt_invalid_operation
- [ ] brew_cask_dependency
- [ ] brew_unknown_command
- [ ] dirty_unzip
- [ ] dnf_no_such_command
- [ ] docker_not_command
- [ ] fab_command_not_found
- [ ] fix_file
- [ ] gem_unknown_command
- [ ] go_unknown_command
- [ ] gradle_no_task
- [ ] grunt_task_not_found
- [ ] gulp_not_task
- [ ] ifconfig_device_not_found
- [ ] missing_space_before_subcommand
- [ ] npm_wrong_command
- [ ] omnienv_no_such_command
- [ ] open
- [ ] pacman
- [ ] pacman_invalid_option
- [ ] pacman_not_found
- [ ] path_from_history
- [ ] port_already_in_use
- [ ] react_native_command_unrecognized
- [ ] remove_trailing_cedilla
- [ ] rm_root
- [ ] scm_correction
- [ ] sed_unterminated_s
- [ ] sl_ls
- [ ] ssh_known_hosts
- [ ] switch_lang
- [ ] systemctl
- [ ] terraform_init
- [ ] terraform_no_command
- [ ] test
- [ ] touch
- [ ] tsuru_login
- [ ] tsuru_not_command
- [ ] unknown_command
- [ ] vagrant_up
- [ ] whois
- [ ] workon_doesnt_exists
- [ ] wrong_hyphen_before_subcommand
- [ ] yarn_alias
- [ ] yarn_command_not_found
- [ ] yarn_command_replaced
- [ ] yarn_help
- [ ] yum_invalid_operation

</details>
