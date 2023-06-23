import asyncio

r = range(10)

async def f():
    for i in range(10):
        yield i

async def main():
    a = aiter(f())

    breaker = True
    while breaker:
        try:
            print(await anext(a))
        except:
            breaker = False

asyncio.run(main())