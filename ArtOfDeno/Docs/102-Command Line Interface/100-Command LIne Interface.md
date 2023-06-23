# Command Line Interface

Deno is command line program. Yu should be familiar with some simple commands having followed the examples thus far and already understand the basic of shell usage.

For deno help text:

```bash
# Using the subcommand.
deno help

# Using the short flag -- outputs the same as above.
deno -h

# Using the long flag -- outputs more detailed help text where available.
deno --help
```

Deno's CLI is subcommand-based. the above commands should show you a list of subcommands support, such as `deno compile`. To see subcommand-specific help, for example for `compile`, you similarly run: 

```bash
deno help compile
deno compile -h
deno compile --help
```