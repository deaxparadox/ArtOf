# HTTP server

Create a new file and copy the following code:

```ts
import { serve } from "https://deno.land/std@0.187.0/http/server.ts";

const handler = async (request: Request): Promise<Response> => {
    const resp = await fetch("https://api.github.com/users/denoland", {
        headers: {
            accept: "application/json",
        },
    });
    
    return new Response(resp.body, {
        status: resp.status,
        headers: {
            "content-type": "application/json",
        },
    });
};

serve(handler);
```

Let's walk through what this program does.

1. Import the http server from std/http (standard library)
2. HTTP servers need a handler function. This function is called for every request that comes in. It must return a `Response`. The handler function can be asynchronous (it may return a `Promise`).
3. Use `fetch` to fetch the url.
4. Return the GitHub response as a response to the handler.
5. Finally, to start the server on the default port, call serve with the handler.

Now run the server. Note that you need to give network permissions.

```bash
deno run --allow-net http_server.ts
```

Visite `http://localhost:8000`, you will get a JSON Response from the Deno Github page.