# Metaclasses are Subclasses of Type

- *type* is a class that generates user-defined classes.
- Metaclasses are subclasses of the *type* class.
- Class objects are instances of the *type* class, or a subclass thereof.
- Instance objects are generated from a class.

In other words, to control the way classes are created and augment their behavior, all we need to do is specify that a user-defined class be created from a user-defined metaclass instead of the normal *type* class.

Notice that this *type instance* relationship is not quite the same as normal *inheritance*.

- User-defined classes may also have superclasses from which they and their instances inheirt attributes as usual.

[<<< Classes are instances](101-classes-are-instances.md) ... [Class Protocol >>>](103-class-protocol.md)