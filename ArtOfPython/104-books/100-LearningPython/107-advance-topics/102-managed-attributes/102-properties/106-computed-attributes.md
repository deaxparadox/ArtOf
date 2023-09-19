# Computed Attributes

Let's code descriptor to automatically squaure an attribute's value each time it is fetched:

```py
class DescSquare:
    def __init__(self, start):              # Each descriptor has own state
        self.value = start 
    def __get__(self, instance, owner):     # On attr fetch
        return self.value ** 2 
    def __set__(self, instance, value):     # On attr asign
        self.value = value                  # No delete or docs


class Client1:
    X = DescSquare(3)

class Client2:
    X = DescSquare(32)

c1 = Client1()

c2 = Client2()

print(c1.X)
c1.X = 4
print(c1.X)
print(c2.X)
```

```bash
9
16
1024
```