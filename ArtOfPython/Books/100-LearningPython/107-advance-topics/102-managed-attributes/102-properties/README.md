# Properties

The property protocol allows us to route a specific attribute's get, set and delete operations function or methods we provide, enabling us to insert code to be run automatically on attribute access, intercept attribute deletions, and provide documentation for the attribute if desired.

Properties are created with the **property** built-in and are assigned to class attributes just like method functions.

- they are inherited by subclasses and instances like any other class attributes.
- their access-interception functions are provided with the **self** instance argument, which grants access to state information and class attributes available on the subject instance.

A property manages a single, specific attribute; 

- although it can't catch all attribute accesses generically, it allows us to control both fetch and assignment accesses and enables us to change an attribute from simple data to computation freely, without breaking existing code.