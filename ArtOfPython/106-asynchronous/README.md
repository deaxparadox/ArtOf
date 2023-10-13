# Asynchronous I/O

```py
import asyncio

async def main():
    print("Hello ...")
    await asyncio.sleep(1)
    print("... World!")

asyncio.run(main())
```


asyncio is a library to write **concurrent** code using the **async/await** syntax.

asyncio is used as a foundation for multiple Python asynchronous frameworks that provide high-performance network and web-servers, database connection libraries, distributed task queues, etc.

asyncio is often a perfect fit for IO-bound and high-level structured network code.

asyncio provides a set of **high-level** API to:

- run Python coroutines concurrently and have full over their execution.
- perform network IO and IPC.
- control subprocess;
- distribute tasks via queues;
- synchronize concurrent code.

Additionally, there are **low-level** APIs for *library* and 