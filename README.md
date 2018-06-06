save-zsh-history
================

`save-zsh-history` is a command saving only useful commands from .zsh_history.

## Installation

```console
> git clone https://github.com/r6eve/save-zsh-history.git
> cd save-zsh-history
# And, in src/main.rs, set unsave commands using regular expression.

> mv ~/.zsh_history .
# And, in ~/.zshrc, add the following line.
HISTFILE=$HOME/this/repository/path/save-zsh-history/.zsh_history

> rm -rf .git
> git init && git add . && git commit -m 'initial commit'
```

Then, host this git repository to public or private repository (I recommend private repository).

## Usage

Execute following commands when you save .zsh_history,

```console
> cargo run  # save only useful commands from .zsh_history, delete others.
> git commit -am 'some commits'
> git push local-somewhere remote-somewhere
```

## Misc

`do_plot.sh` is a subtle, useful tool to plot a time series graph about insertions and deletions.
