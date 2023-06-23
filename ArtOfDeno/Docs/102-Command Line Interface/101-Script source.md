# Script source

Deno can grab the scripts from multiple sources, a filename, a url, and '-' to read the file from stdin. This latter is usefull for integration with other applications:

```bash
deno run main.ts
deno run https://mydomain.com/main.ts
cat main.ts | deno run -
```