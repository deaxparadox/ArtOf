# Running an asyncio Program

## **asyncio.run(coro, *, debug=False)

Execute the coroutine coro and return the result

This function runs the passed coroutine, taking care of managing the asyncio event loop, *finalizing asynchornous generators*, and closing the threadpool.

This function cannot be called when another asyncio event loop in the same thread.