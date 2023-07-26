# Inserting Code to Run on Attribute Access

That’s one of the main roles of managed attributes—they  provide ways to add attribute accessor logic after the fact

Specifically, you will see *four* accessor:

- The **__getattr__** and **__setattr__** methods, for rouing undefined attribute fetches and all attribute assignments to generic handler methods.

- The **__getattribute__** method, for routing all attribute fetches  to a generic handler method.

- The **property** built-in, for routing specific attribute access to get and set handler functions.

- The *descriptor protocol*, for routing specific attribute accesses to instances of classes with arbitrary get and set handler methods, and  the basis  for other tools such as properties and slots.

The last two techinques listed here apply to *specific* attributes, whereas the first two are generic enough to be used by delegation-based proxy classes that must route *arbitrary* attributes to wrapped objects.