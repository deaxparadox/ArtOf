# Why Use Classes ?

In simple terms, class are just a way to define new sorts of stuff; reflecting real objects in a program's domain.

For instance, support we decide to implement that hypothetical pizza-making robot.

Two aspect of OOP:

1. *Inheritence*: Pizza-making robots are kinds of robots, so they possess the usual robot-y properties. In OOP terms, we say they “inherit” properties from the general category of all robots. These common properties need to be implemented only once for the general case and can be reused in part or in full by all types of robots we may build in the future.

2. *Composition*: For instance, for our robot to be  successful, it might need arms to roll dough, motors to maneuver to the oven, and so on. In OOP parlance,  our robot is an example of composition; it contains other objects that it activates to do  its bidding. Each component might be coded as a class, which defines its own behavior and relationships.


classes also define new namespaces.

classes have three critical distinction that make them more usefull when it comes to building new objects:

1. *Multple instances*: Classes are essentially  factories for generating one or more objects. Every  time we call a class, we generate a new object with  a distinct namespace. Each object generated from a  class has access to the class’s attributes and gets a namespace of its own for data that varies per object.

2. *Composition via inheritence*: Classes also  support the OOP notion of inheritance; we can extend a class by redefining its attributes outside the class itself in new software components coded as subclasses

3. *Operator overloading*: By providing special protocol methods, classes can define objects that respond to the sorts of operations we saw at work on built-in types. For instance, object made with classes can be sliced, concatenated, indexed, and so on. Python provides hooks that classes can use to intercept and implement any built-in type operation.

The mechanism of OOP in Python is largely just *two bits of magic*: a special first argument in functions (to receive the  subject of a call) and inheritence attribute search (to support programming by customization).