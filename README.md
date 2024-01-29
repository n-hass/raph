# Installation
`cargo install` will install a binary, `_raph_bin`, and a script, `_raph`. Note the installation is HARD CODED to install the `_raph` executor script to `$HOME/.cargo/bin`, so if you are using an alternate cargo install path, things will not work.

After installation, add this line to your shell `.profile` or `.rc` file: 
```
alias raph="source _raph"
```

# Usage
Usage: `raph [OPTIONS] [profile] [command]...`

Arguments:

- `[profile]`     		Specifies the AWS profile to use
- `[command]...`  	An 'aws CLI' command to execute with the specified AWS profile, without affecting the current shell's environment

Options:

- `-n, --no-auto-prefix`  Disables automatic prefixing of the 'aws' command

Providing no arguments will activate the interactive profile switcher.

Providing the profile argument will skip interactive switching and perform the switch.

Providing the profile argument and a command will run this command once-off using the provided profile argument. This does NOT change the shell environment. If providing a command which does not start with 'aws', it will be automatically prefixed with 'aws'. This can be disabled with the `-n` flag to run scripts.

#### Examples

`raph profileA` - Uninteractively switch the shell environment to use profileA

`raph profileB aws sso login` - run `aws sso login` using profileB, leaving the `AWS_PROFILE` environment variable intact.

`raph profileB sso login` - same as above. It is not required to prefix the command with 'aws'.

`raph profileB ~/my_script.sh --opt1 foo --opt2 bar extraopts` - will run `~/my_script.sh --opt1 foo --opt2 bar extraopts` verbatim, with AWS_PROFILE=profileB environment variable.