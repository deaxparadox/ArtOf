from __future__ import print_function
import os
import sys



registry = {}
def register(obj):                      # Both class and func decorator
    registry[obj.__name__] = obj        # Add to registry
    return obj                          # Return obj itself, not a wrapper


@register
def spam(x):
    return x ** 2                       # spam = register(spam)

@register
def ham(x):
    return x ** 3

@register
class Eggs:                             # Eggs = register(Eggs)
    def __init__(self, x):
        self.data = x ** 4 
    def __str__(self) -> str:
        return str(self.data)

print("Registry:")
for name in registry:
    print(name, "=>", registry[name], type(registry[name]))

print("\nManual calls:")                                    # Invoke objects mannually
print(spam(2))                                              # Later calls not intercepted
print(ham(2))
X = Eggs(2)
print(X)

print("\nRegistry calls:")
for name in registry:
    print(name, "=>", registry[name](2))                    # Invoke from registry
 
sys.exit(0)