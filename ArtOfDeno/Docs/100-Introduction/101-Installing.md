# Installtion

Deno works on macOS, Linux, and Windows. Deno is a single binary executable. 

## Download nad install

`deno_install` provides convenience scripts to download and install the binary.

Using shell (macOS and Linux):

```bash
curl -fsSL https://deno.land/x/install/install.sh | sh
```

Build and install from source using `Cargo`:

```bash
cargo install deno --locked
```

Deno binaries can also be installed manually, by downloading a zip file at `github.com/denoland/deno/releases`. These packages contain just a single executable file. You will have to set the executable bit on macOS and Linux.


## Testing your installation

To test your installation, run `deno --version`. It this prints the Deno version to the console the installation was successfull.

Use `deno help` to see help text documenting Deno's flags and usage. Get a detail guide on the CLI `here`.

## Updating

To update a previously install version of Deno, you can run:

```bash
deno upgrade
```

This will fetch the latest release from `github.com/denoland/deno/releases`, unzip it, and replace your current executable with it.

You can also use this utility to install a specific version of Deno:

```bash
deno upgrade --version 1.0.1
```