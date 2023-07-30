# Computed Properties

The following creates a virtual attribute X that runs a calculation when fetched:


```py
class AttrSquare:
    def __init__(self, start):
        self.value = start                      # Triggers __setattr___!

    def __getattr__(self, attr):                # On undefined attr fetch
        if attr == 'X':
            return self.value ** 2              # value is not undefined
        else:
            raise AttributeError(attr)
        
    def __setattr__(self, attr, value):         # On all attr assignments
        if attr == 'X':
            attr = 'value'
        self.__dict__[attr] = value

A = AttrSquare(3)
B = AttrSquare(32)

print(A.X)
A.X = 4
print(A.X)
print(B.X)
```


```bash
9
16
1024
```


## Using `__getattribute__`

