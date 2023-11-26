import time
from typing import Any

def timer(func) -> Any:
    def wrapper(*args, **kwargs) -> Any:
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"Time elapsed in {func.__name__}: {end-start}")
        return result
    return wrapper

class Timer:
    def __init__(self, func) -> None:
        self.func = func
    def __call__(self, *args: Any, **kwds: Any) -> Any:
        start = time.time()
        result = self.func(*args, **kwds)
        end = time.time()
        print(f"Time elapsed in {self.func.__name__}: {end-start}")
        return result

@Timer
def hw() -> None:
    print("hello world")
    return 

@timer
def count100() -> None:
    for i in range(100):
        print(i, end="\t")
    print()


if __name__ == "__main__":
    hw()
    count100()