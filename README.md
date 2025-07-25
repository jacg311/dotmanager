# dotmanager
A tool to manage your dotfiles via symlinks and git.

Disclaimer: Currently does not work on Windows. Blame Microsoft :)
## Usage
```sh
dotmanager init <dir> # create a new dotfile repository in the supplied directory (or current working dir). (creates a git directory with a standard config file inside)
dotmanager init <dir> --no-git # only create standard config file, no git repository will be created

dotmanager apply <dir> # create symlinks from files in the current dotfile repository to their configured location in your filesystem (<dir> by default without any further configuration)
dotmanager apply --force # if files already exist in the target repository will be overwritten
dotmanager apply --dry-run # Instead of applying the dotfiles, just log where they would go.
dotmanager apply --copy-files # copy files instead of symlinking

dotmanager collect # collects all files that match the file rules that arent already in the repository
dotmanager collect --force # forcefully collects all files that match the rules and overwrites existing files in the repository

```
### Configuration
Without any configuration `dotmanager` just replicates the file structure of the dotfile repository in the target directory.

However you can configure where files go with config files. Create a file called `.dotdef` within any folder inside the dotfile directory.
```sh
/home/user/.config/ # symlink everything inside the current directory into this directory
../.config/ # relative paths work as expected
~/.config # the last / is optional.
          
*.png:~/Wallpapers # symlink any files with the png extension anywhere into ~/Wallpapers
/niri:~/.config/niri # symlink the /niri directory in your repository to
```