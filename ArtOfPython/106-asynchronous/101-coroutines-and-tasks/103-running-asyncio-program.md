# Running an asyncio Program

## **asyncio.run(coro, *, debug=False)

Execute the coroutine coro and return the result

This function runs the passed coroutine, taking care of managing the asyncio event loop, *finalizing asynchornous generators*, and closing the threadpool.

This function cannot be called when another asyncio event loop in the same thread.

If *debug* is `True`, the event loop will be run in debug mode.

This function alwasy creates a new event loop and closes it at the end. It should be used as a main entry pointer for asyncio programs, and should ideally only be called once.

Example:

```py
async def main():
    await asyncio.sleep(1)
    print('hello')

asyncio.run(main())
```

[Awaitables](102-awaitables.md) ... [Creating Tasks](104-creating-tasks.md)