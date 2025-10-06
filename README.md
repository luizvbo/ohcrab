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

- For **bash**:

  ```shell
  eval $(ohcrab --shell bash)
  ```

- For **zsh**:
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

| Rule Name                              | Description                                                                                                         |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| `ag_literal`                           | Adds `-Q` to `ag` commands for literal string searches when a regex error occurs.                                   |
| `apt_get`                              | Suggests installing a command with `apt-get` if it's not found.                                                     |
| `apt_get_search`                       | Corrects `apt-get search` to `apt-cache search`.                                                                    |
| `apt_list_upgradable`                  | Suggests running `apt list --upgradable` after `apt update` shows available upgrades.                               |
| `apt_upgrade`                          | Suggests running `apt upgrade` after listing upgradable packages with `apt list --upgradable`.                      |
| `aws_cli`                              | Corrects misspelled AWS CLI commands based on the suggestions provided.                                             |
| `az_cli`                               | Corrects misspelled Azure CLI commands.                                                                             |
| `brew_install`                         | Corrects misspelled formula names for `brew install`.                                                               |
| `brew_link`                            | Suggests using `--overwrite --dry-run` when a `brew link` fails due to existing symlinks.                           |
| `brew_reinstall`                       | Suggests `brew reinstall` when trying to install a formula that is already installed.                               |
| `brew_uninstall`                       | Suggests using `--force` to uninstall all versions of a formula when multiple are present.                          |
| `brew_update_formula`                  | Corrects `brew update <formula>` to `brew upgrade <formula>`.                                                       |
| `cargo`                                | Suggests `cargo build` when `cargo` is run without any arguments.                                                   |
| `cargo_no_command`                     | Corrects misspelled Cargo subcommands (e.g., `buid` to `build`).                                                    |
| `cat_dir`                              | Replaces `cat` with `ls` when used on a directory.                                                                  |
| `cd_correction`                        | Corrects typos in directory names when using `cd`.                                                                  |
| `cd_cs`                                | Corrects the common typo `cs` to `cd`.                                                                              |
| `cd_mkdir`                             | Creates a directory with `mkdir -p` and then `cd`s into it if it doesn't exist.                                     |
| `cd_parent`                            | Corrects `cd..` to `cd ..`.                                                                                         |
| `chmod_x`                              | Adds execute permissions (`chmod +x`) to a script that fails with a permission error.                               |
| `choco_install`                        | Corrects Chocolatey package names by suggesting the `.install` suffix.                                              |
| `composer_not_command`                 | Corrects misspelled Composer commands based on suggestions.                                                         |
| `conda_mistype`                        | Corrects misspelled conda commands based on suggestions.                                                            |
| `cp_create_destination`                | Creates the destination directory with `mkdir -p` before moving or copying files into it.                           |
| `cp_omitting_directory`                | Adds the `-a` (or `-r`) flag to `cp` when attempting to copy a directory.                                           |
| `cpp11`                                | Adds the `-std=c++11` flag to `g++` or `clang++` when C++11 support is required.                                    |
| `dirty_untar`                          | Prevents extracting a tarball into the current directory by first creating a new directory named after the archive. |
| `django_south_ghost`                   | Adds the `--delete-ghost-migrations` flag to a failing Django South migration.                                      |
| `django_south_merge`                   | Adds the `--merge` flag to a failing Django South migration with dependency conflicts.                              |
| `docker_image_being_used_by_container` | Suggests removing the container that is using an image before trying to remove the image.                           |
| `docker_login`                         | Suggests running `docker login` before a command that fails due to an access-denied error.                          |
| `dry`                                  | Removes a duplicated command at the beginning of the script (e.g., `git git status`).                               |
| `fix_alt_space`                        | Fixes commands that use a non-breaking space (Alt+Space) instead of a regular space.                                |
| `git_add`                              | Suggests running `git add` on a file that is not tracked by Git before committing or updating it.                   |
| `git_add_force`                        | Adds `--force` to `git add` when trying to add a file that is ignored by `.gitignore`.                              |
| `git_bisect_usage`                     | Corrects misspelled `git bisect` subcommands (e.g., `strt` to `start`).                                             |
| `git_branch_0flag`                     | Corrects `git branch` flags where a `0` was used instead of a `-` (e.g., `git branch 0d` to `git branch -d`).       |
| `git_branch_delete`                    | Suggests using `git branch -D` to delete a branch that is not fully merged.                                         |
| `git_branch_delete_checked_out`        | Suggests checking out another branch before deleting the currently checked-out branch.                              |
| `git_branch_exists`                    | Suggests actions when trying to create a branch that already exists, such as checking it out.                       |
| `git_branch_list`                      | Corrects `git branch list` to `git branch`.                                                                         |
| `git_checkout`                         | Corrects a misspelled branch name or suggests creating a new branch for `git checkout`.                             |
| `git_clone`                            | Fixes duplicated `git clone` in the command (e.g., `git clone git clone ...`).                                      |
| `git_clone_missing`                    | Prepends `git clone` when a git repository URL is entered directly into the terminal.                               |
| `git_commit_add`                       | Suggests using `-a` or `-p` with `git commit` when there are no staged changes.                                     |
| `git_commit_amend`                     | Suggests `git commit --amend` to amend the previous commit.                                                         |
| `git_commit_reset`                     | Suggests `git reset HEAD~` to undo the last commit.                                                                 |
| `git_diff_no_index`                    | Adds `--no-index` to `git diff` when comparing two files that are not in the git index.                             |
| `git_diff_staged`                      | Suggests `git diff --staged` to show staged changes instead of unstaged ones.                                       |
| `git_fix_stash`                        | Corrects misspelled `git stash` subcommands (e.g., `opp` to `pop`).                                                 |
| `git_flag_after_filename`              | Fixes commands where a git flag was incorrectly placed after a filename.                                            |
| `git_help_aliased`                     | Shows help for the original command when an alias is used with `git help`.                                          |
| `git_hook_bypass`                      | Adds `--no-verify` to `git commit`, `push`, or `am` to bypass pre-commit and pre-push hooks.                        |
| `git_lfs_mistype`                      | Corrects misspelled `git lfs` commands.                                                                             |
| `git_main_master`                      | Switches between `main` and `master` when a branch with one of those names is not found.                            |
| `git_merge`                            | Corrects misspelled branch names in `git merge`.                                                                    |
| `git_merge_unrelated`                  | Adds the `--allow-unrelated-histories` flag to `git merge` when histories are unrelated.                            |
| `git_not_command`                      | Corrects misspelled git commands (e.g., `git comit` to `git commit`).                                               |
| `git_pull`                             | Sets the upstream branch for `git pull` or `git push` when it's not set.                                            |
| `git_pull_clone`                       | Suggests `git clone` instead of `git pull` when not in a git repository.                                            |
| `git_pull_uncommitted_changes`         | Suggests stashing local changes before a pull or rebase.                                                            |
| `git_push`                             | Sets the upstream branch for `git push` when it's not set.                                                          |
| `git_push_different_branch_names`      | Fixes `git push` when the local and remote branch names differ.                                                     |
| `git_push_force`                       | Suggests using `--force-with-lease` when a `git push` is rejected due to remote changes.                            |
| `git_push_pull`                        | Suggests `git pull` before `git push` when the remote has changes that you don't have locally.                      |
| `git_push_without_commits`             | Suggests creating an initial commit before pushing an empty repository.                                             |
| `git_rebase_merge_dir`                 | Suggests how to proceed with an existing rebase (`--continue`, `--abort`, or `--skip`).                             |
| `git_rebase_no_changes`                | Suggests `git rebase --skip` when a rebase patch has no changes.                                                    |
| `git_remote_delete`                    | Corrects `git remote delete` to `git remote remove`.                                                                |
| `git_remote_seturl_add`                | Suggests `git remote add` instead of `git remote set-url` for a remote that doesn't exist.                          |
| `git_rm_local_modifications`           | Suggests using `--cached` or `-f` with `git rm` for files that have local modifications.                            |
| `git_rm_recursive`                     | Adds the `-r` flag to `git rm` when trying to remove a directory.                                                   |
| `git_rm_staged`                        | Suggests using `--cached` or `-f` with `git rm` for files that have staged changes.                                 |
| `git_stash`                            | Suggests stashing local changes before a command that would overwrite them (e.g., `cherry-pick`).                   |
| `git_stash_pop`                        | Suggests a safe way to apply a stash when there are conflicting local changes.                                      |
| `git_tag_force`                        | Adds the `--force` flag to `git tag` when the tag already exists.                                                   |
| `git_two_dashes`                       | Corrects single-dash flags to double-dash flags (e.g., `-patch` to `--patch`).                                      |
| `go_run`                               | Appends the `.go` extension to the filename when using `go run`.                                                    |
| `gradle_wrapper`                       | Replaces `gradle` with `./gradlew` when the Gradle wrapper is available in the current directory.                   |
| `grep_arguments_order`                 | Fixes the argument order for `grep` when the pattern is mistaken for a file.                                        |
| `grep_recursive`                       | Adds the `-r` flag to `grep` when used on a directory.                                                              |
| `has_exists_script`                    | Prepends `./` to a script in the current directory that is not in the `PATH`.                                       |
| `heroku_multiple_apps`                 | Suggests specifying an app with `--app` when multiple Heroku apps are configured in git remotes.                    |
| `heroku_not_command`                   | Corrects misspelled Heroku commands.                                                                                |
| `history`                              | Suggests a command from your shell history that is similar to the mistyped command.                                 |
| `hostscli`                             | Corrects misspelled `hostscli` commands.                                                                            |
| `java`                                 | Removes the `.java` extension when running a compiled class with the `java` command.                                |
| `javac`                                | Appends the `.java` extension to the filename when compiling with `javac`.                                          |
| `lein_not_task`                        | Corrects misspelled Leiningen tasks.                                                                                |
| `ln_no_hard_link`                      | Replaces `ln` with `ln -s` when trying to create a hard link to a directory.                                        |
| `ln_s_order`                           | Fixes the order of arguments for `ln -s` (source and destination).                                                  |
| `long_form_help`                       | Replaces the short-form help flag (`-h`) with the long-form (`--help`).                                             |
| `ls_all`                               | Suggests `ls -A` to show hidden files when the output of `ls` is empty.                                             |
| `ls_lah`                               | Replaces `ls` with `ls -lah` for a more detailed, human-readable output.                                            |
| `man`                                  | Suggests different man sections (e.g., 2 or 3) or the `--help` flag if a man page isn't found.                      |
| `man_no_space`                         | Adds a space between `man` and the command (e.g., `mandiff` to `man diff`).                                         |
| `mercurial`                            | Corrects misspelled Mercurial (hg) commands.                                                                        |
| `mkdir_p`                              | Adds the `-p` flag to `mkdir` to create parent directories as needed.                                               |
| `mvn_no_command`                       | Suggests common goals like `clean package` or `clean install` when `mvn` is run without any goals.                  |
| `mvn_unknown_lifecycle_phase`          | Corrects misspelled Maven lifecycle phases.                                                                         |
| `nixos_cmd_not_found`                  | Suggests installing a missing command on NixOS using `nix-env`.                                                     |
| `no_command`                           | Corrects a misspelled command based on available executables in your `PATH`.                                        |
| `no_such_file`                         | Creates the destination directory with `mkdir -p` before a `mv` or `cp` command.                                    |
| `npm_missing_script`                   | Corrects misspelled npm script names based on the `package.json` file.                                              |
| `npm_run_script`                       | Adds `run-script` to the command when trying to run an npm script directly.                                         |
| `php_s`                                | Corrects `php -s` to `php -S` for running the built-in web server.                                                  |
| `pip_install`                          | Suggests using `--user` or `sudo` when `pip install` fails due to permission errors.                                |
| `pip_unknown_command`                  | Corrects misspelled pip commands (e.g., `instatl` to `install`).                                                    |
| `prove_recursively`                    | Adds the `-r` (recursive) flag to `prove` when it's run on a directory.                                             |
| `python_command`                       | Prepends `python` to a Python script that is not executable.                                                        |
| `python_execute`                       | Appends the `.py` extension to the filename when using the `python` command.                                        |
| `python_module_error`                  | Suggests installing a missing Python module using `pip` when a `ModuleNotFoundError` occurs.                        |
| `quotation_marks`                      | Fixes mismatched single and double quotation marks in a command.                                                    |
| `rails_migrations_pending`             | Suggests running pending Rails migrations before executing the original command.                                    |
| `remove_shell_prompt_literal`          | Removes a leading `$` from a command that was copied and pasted from a tutorial or documentation.                   |
| `rm_dir`                               | Adds the `-rf` flag to `rm` when trying to remove a directory.                                                      |
| `sl_ls`                                | Corrects the classic typo `sl` (Steam Locomotive) to `ls`.                                                          |
| `sudo`                                 | Prepends `sudo` to a command that fails with a permission error.                                                    |
| `sudo_command_from_user_path`          | Fixes `sudo` commands that fail because a command is in the user's `PATH` but not in the root's `PATH`.             |
| `tmux`                                 | Corrects ambiguous tmux commands by suggesting from a list of possibilities.                                        |
| `touch`                                | Creates the parent directory with `mkdir -p` before touching a file within it.                                      |
| `unsudo`                               | Removes `sudo` from a command that should not be run as root.                                                       |

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
- [ ] ssh_known_hosts
- [ ] switch_lang
- [ ] systemctl
- [ ] terraform_init
- [ ] terraform_no_command
- [ ] test
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
