# Watch mode

You can supply the `--watch` flag to `deno run`, `deno test`, `deno compile`, and `deno fmt` to enable the build-in file watcher. This files that are watched depend on the subcomand used:

- for `deno run`, `deno test`, and `deno compile` the entry point, and all local files the entrypoint(s) statically import(s) will be watched.
- for `deno fmt` all local files and directories specified as command line arguments (or the working directory if no specific files/directories is passed) are watched.

Whenever one of the watched files is changed on disk, the program will automatically be restart/ formatted/ tested/ bundled.

```bash
deno run --watch main.ts
deno test --watch
deno fmt --watch
```

