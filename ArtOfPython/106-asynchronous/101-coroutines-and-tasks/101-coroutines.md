# Coroutines


Coroutines declared with the `async/await` syntax is the preferred way of writing asyncio applications. For example, the following snippet of code prints “hello”, waits 1 second, and then prints “world”:

```py
import asyncio

async def main():
    print('hello')
    await asyncio.sleep(1)
    print('world')

asyncio.run(main())


```

Note that simply calling a coroutine will not schedule it to be executed:

```py
main()
```

----------

### To actually run a coroutine, asyncio provides three main mechanisms:

#### 1. The `asyncio.run()` function to run the top-level entry pointer "main()" function.

As show above.

#### 2. Awaiting on coroutine. The following snippet code will print "hello" after waitign for 1 second, and then print "world" after waiting for *another* 2 seconds:

```py
import asyncio
import time

async def say_after(delay, what):
    await asyncio.sleep(delay)
    print(what)

async def main():
    print(f"started at {time.strftime('%X')}")

    await say_after(1, 'hello')
    await say_after(2, 'world')

    print(f"finished at {time.strftime('%X')}")

asyncio.run(main())
```

Expected output:

```
started at 17:13:52
hello
world
finished at 17:13:55
```

#### 3. The `asyncio.create_task()` function to run coroutines concurrently as asyncio Tasks.

Lets modify the above example and run two `say_after` coroutines *concurrently`:

```py
async def main():
    task1 = asyncio.create_task(
        say_after(1, 'hello'))

    task2 = asyncio.create_task(
        say_after(2, 'world'))

    print(f"started at {time.strftime('%X')}")

    # Wait until both tasks are completed (should take
    # around 2 seconds.)
    await task1
    await task2

    print(f"finished at {time.strftime('%X')}")
```

Note that expected output now shows that the snippet runs 1 second faster than before:

```py
started at 17:14:32
hello
world
finished at 17:14:34
```


[<<< home](README.md) ... [Awaitables >>>](102-awaitables.md)