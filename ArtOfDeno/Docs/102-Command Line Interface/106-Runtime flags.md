# Runtime flags

Affect commands which execute user code: `deno run` and `deno test`. These include all of the above as well as the following.


## Type checking flags

You can type-check your code (without executing it) using the command:

```bash
deno check main.ts
```

You can also type-check your code before execution by usnig the `--check` argument to deno run:

```bash
deno run --check main.ts
```

This flag affects `deno run`, `deno eval`, `deno repl`, and `deno cache`.

## Other runtime flags

More flags which affect the execution environment:

```bash
--cached-only                Require that remote dependencies are already cached
--inspect=<HOST:PORT>        activate inspector on host:port ...
--inspect-brk=<HOST:PORT>    activate inspector on host:port and break at ...
--inspect-wait=<HOST:PORT>   activate inspector on host:port and wait for ...
--location <HREF>            Value of 'globalThis.location' used by some web APIs
--prompt                     Fallback to prompt if required permission wasn't passed
--seed <NUMBER>              Seed Math.random()
--v8-flags=<v8-flags>        Set V8 command line options. For help: ...
```