import matplotlib as mpl 
import matplotlib.pyplot as plt 
import numpy as np

import argparse

def argument():
    parse = argparse.ArgumentParser()
    parse.add_argument("-p", type=int, help="Provide the power")
    return parse.parse_args()

def power(iterable, /, *, p: int = 1):
    """
    For plotting graph of number and its square
    """
    x, y = [], []
    for i in iterable:
        x.append(i)
        y.append(i ** p)
    plt.plot(x, y, 'ro')
    plt.ylabel("Some numbers along y")
    plt.xlabel("Some number along x")
    plt.title("Simple plot")
    plt.show()


if __name__ == "__main__":
    arg = argument()
    # print(arg.p, type(arg.p))
    # print(power.__doc__)
    if arg.p:
        power(list(range(10)), p=arg.p)
    else:
        power(list(range(10)))