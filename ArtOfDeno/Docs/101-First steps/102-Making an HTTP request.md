# Making an HTTP request

Let's write a small program that fetchs a file and prints it contents out to the terminal. Just like in  the browser you can use the web standard `fetch` API to make HTTP calls:

```ts
const res = await fetch("https://deno.land");
const body = await res.text();
console.log(body);
```

Let's walk through what this application does:

- We make a request to the `https://deno.land`, await the response, and store it in the `res` constant.
- We parse the response body as a text and store in the `body` constant.
- We write the contents of the `body` constant to the console.

If we try to run:

```bash
deno run first_steps.ts
```

We will network permission error, because Deno is a runtime that is secure by default.

Now if you run this, you program will be sucessfull.

```bash
deno run --allow-net=deno.land first_steps.ts
```