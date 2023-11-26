# Timer Decorator

We are going to build a `timer` decorator, which calculator the function run time.

```py
import time
import typing

def timer(func) -> typing.Any:
    def wrapper(*args, **kwargs) -> typing.Any:
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"Time elapsed in {func.__name__}: {end-start}")
        return result
    return wrapper

@timer
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
```

```bash
hello world
Time elapsed in hw: 2.09808349609375e-05
0       1       2       3       4       5       6       7       8       9       10      11      12      13  14       15      16      17      18      19      20      21      22      23      24      25      26      27  28       29      30      31      32      33      34      35      36      37      38      39      40      41  42       43      44      45      46      47      48      49      50      51      52      53      54      55  56       57      58      59      60      61      62      63      64      65      66      67      68      69  70       71      72      73      74      75      76      77      78      79      80      81      82      83  84       85      86      87      88      89      90      91      92      93      94      95      96      97  98       99
Time elapsed in count100: 0.00011730194091796875
```