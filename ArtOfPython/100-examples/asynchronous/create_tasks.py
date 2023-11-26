import asyncio

async def coroutine(i):
    print("Start...")
    await asyncio.sleep(i)
    print("End...")

async def main():
    background_tasks = set()

    for i in range(10):
        task = asyncio.create_task(coroutine(i))

        background_tasks.add(task)

        task.add_done_callback(background_tasks.discard)

if __name__ == "__main__":
    asyncio.run(main())