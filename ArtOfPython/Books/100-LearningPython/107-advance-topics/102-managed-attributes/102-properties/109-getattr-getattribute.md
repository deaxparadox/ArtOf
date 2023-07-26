# __getattr__ and __getattribute__

Attribute fetches interception comes in two flavors, coded with two different methods:

- `__getattr__` jis run for *undefined* attributes--because it is run only for attributes not stored on an instance or inherited from one of its classes, its use is straightforward.
- `__getattribute__` is run for *every* attribute--because it is all-inclusive, you must be cautious when using this method to avoid rercursive loops by attibute accesses to a superclass.

These methods are representatives of a set of attribute interception methods that also include `__setattr__` and `__delattr__`.

These methods are part of Python's general *operator overloading* protocol--specially named methods of a class, inherited by subclasses, and run automatically when instances are used in the implied built-in operation.

- Like all normal methods of a class, they each receive a first **self** argument when called, giving access to any required instance state information as well as other methods of class in which they appear.

They can be used to intercept access to any (or even all) instance attribute fetches, not just a single specific name. Because of this, these two methods are well suited to general *delegation*-based coding pattern--they can be used to implement wrapper (a.k.a proxy) objects that manage all attribute accesses for an embebbed object.

They intercept attribute fetches only, not  assignments:

- to also catch attribute changes by assignment, we must code a `__setattr__` method--an operator overloading method run for every attribute fetch, which must take care to avoid recursive loops by routing attribute assigments through the instance namespace dictionary or a superclass method.
- we can also code `__delattr__` overloading method (which must avoid looping in the same way) to intercept attribute deletions.