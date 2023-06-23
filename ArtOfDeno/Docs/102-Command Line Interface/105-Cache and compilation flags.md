# Cache and compilcation flags

Affect commands which can populate the cache: `deno cache`, `deno run`, `deno test`, `deno doc`, and `deno compile`. As well as the flags above, this includes those which affect module resolution, compilation configuratoin etc.

```bash
--config <FILE>               Load configuration file
--import-map <FILE>           Load import map file
--no-remote                   Do not resolve remote modules
--reload=<CACHE_BLOCKLIST>    Reload source code cache (recompile TypeScript)
--unstable                    Enable unstable APIs
```

