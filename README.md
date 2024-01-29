# Installation
`cargo install` will install a binary, `_raph_bin`, and a script, `_raph`, to your path. Note the installation is HARD CODED to install to `$HOME/.cargo/bin`, so if you are using an alternate cargo install path, things will not work.

After installation, you must add this line to your shell `.profile` or `.rc` file: 
```
alias raph="source _raph"
```

# Usage
Usage: `raph [profile] [["aws"] command]...`

Arguments:
  `[profile]`     		Specifies the AWS profile to use
  `[["aws"] command]...`  	An 'aws CLI' command to execute with the specified AWS profile, without affecting the current shell's environment

Providing no arguments will activate the interactive profile switcher.

Providing the profile argument will skip interactive switching and perform the switch.

Providing the profile argument and a command will run this command once-off using the provided profile argument. This does NOT change the shell environment. Prefixing the command with "aws" is optional.

#### Examples

`raph profileA` - Uninteractively switch the shell environment to use profileA

`raph profileB aws sso login` - run `aws sso login` using profileB, leaving the `AWS_PROFILE` environment variable intact.

`raph profileB sso login` - same as above. It is not required to prefix the command with 'aws'.