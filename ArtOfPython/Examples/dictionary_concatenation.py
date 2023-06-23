import typing


class adict(dict):
    def __add__(self, other: dict):
        return {**self, **other}

a = adict(a=1, b=2)
print(a)

b = adict(c=1, d=2)
d = a + b
print(d)

