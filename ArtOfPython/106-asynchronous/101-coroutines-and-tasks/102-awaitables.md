# Awaitables

We say that an object is an **awaitable** object if it can be used in an await expression. Many asyncio APIs are designed to accept awaitables.

There are three main types of awaitables object:

1. coroutines
2. Tasks
3. Futures



## 1. Coroutines

Python coroutines are *awaitables* and therefore can be awaited from other coroutines:

```py
import asyncio

async def nested():
    return 42

async def main():
    # Nothing happens if we just call "nested()".
    # A coroutine object is created but not awaited,
    # so it *won't run at all*.
    nested()

    # Let's do it differently now and await it:
    print(await nested())  # will print "42".

asyncio.run(main())
```

**Important**: The term "coroutine" can be used for two closely related concept:

- a *coroutine funciton*: async def function;
- a *coroutine object*: an object returned by calling a *coroutine function*.

## 2. Tasks

Tasks are used to schedule coroutine *concurrently*.

When a coroutine is wrapped into a Task with function like `asyncio.create_task()` the coroutine is automatically schedule to run soon:

```py
import asyncio 

async def nested():
    return 42

async def main():
    # Schedule nested() to run soon concurrently
    # with "main()".
    task = asyncio.create_task(nested())

    # "task" can now be used to cancel "nested()", or
    # can simply be awaited to wait until it is complete:
    await task

asyncio.run(main())
```

## 3. Futures

A Futures is a special **low-level** awaitable object that represents an **eventual result** of an asynchronous operation. 

When a Future object is *awaited* it means that the coroutine will wait until the Future is resolved in some other place.

Future objects in asyncio are needed to allow callback-based code to be used with async/await.

Normally **there is no need** to create Future objects at the application level code.

Future objects sometimes exposed by libraries and some asyncio APIs, can be await:

```py
async def main():
    await function_that_returns_a_future_object()

    # this is also valid:
    await asyncio.gather(
        function_that_returns_a_future_object(),
        some_python_coroutine()
    )
```

[<<< Coroutines](101-coroutines.md) ... [Running an asyncio Program >>>](103-running-asyncio-program.md)