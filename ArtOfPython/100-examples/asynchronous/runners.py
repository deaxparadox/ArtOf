import asyncio


async def main():
    print("Start...")
    await asyncio.sleep(1)
    
    event_loop: asyncio.AbstractEventLoop | None = None
    try:
        event_loop = asyncio.get_event_loop()
        print(event_loop)
    except ValueError as e:
        print(e)
    # finally:
    #     event_loop.close()
    
    await asyncio.sleep(1)
    print("Over...")
    return 

if __name__ == "__main__":
    asyncio.run(main())