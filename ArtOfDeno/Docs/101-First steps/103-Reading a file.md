# Reading a file

Deno also provides APIs that do not come from the web. These are all contained in 
the Deno global. You can find documentation for these built-in APIs here at `/api`.

Filesystem APIs for example do not have a web standard form, so Deno provides its own API.

In this program, each command-line argument is assumed to be a filename, the file is opened, and printed to stdout.

```ts
const filenames = Deno.args;
for (const filename of filenames) {
  const file = await Deno.open(filename);
  await file.readable.pipeTo(Deno.stdout.writable, { preventClose: true });
}
```

Run program using:

```bash
deno run --allow-read https://deno.land/std@0.187.0/examples/cat.ts /etc/hosts
```