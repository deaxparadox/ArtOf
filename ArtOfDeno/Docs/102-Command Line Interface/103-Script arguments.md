# Script arguments

Separately from the Deno runtime flags, you can pass user-space arguments to the script you are running by specifying then **after** the script name:

```bash
deno run main.ts a b -c --quite
```

```ts
// main.ts
console.log(Deno.args);
```
