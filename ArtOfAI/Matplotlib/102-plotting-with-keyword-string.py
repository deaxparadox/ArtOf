import matplotlib as mpl 
import matplotlib.pyplot as plt 
import numpy as np

import argparse

def argument():
    parse = argparse.ArgumentParser()
    parse.add_argument("-p", type=int, help="Provide the power")
    return parse.parse_args()

def main(iterable, /, *, p: int = 1):
    data = {'a': np.arange(50),
        'c': np.random.randint(0, 50, 50),
        'd': np.random.randn(50)}
    data['b'] = data['a'] + 10 * np.random.randn(50)
    data['d'] = np.abs(data['d']) * 100

    plt.scatter('a', 'b', c='c', s='d', data=data)
    plt.xlabel('entry a')
    plt.ylabel('entry b')
    plt.show()


if __name__ == "__main__":
    arg = argument()
    # print(arg.p, type(arg.p))
    # print(power.__doc__)
    if arg.p:
        main(list(range(10)), p=arg.p)
    else:
        main(list(range(10)))