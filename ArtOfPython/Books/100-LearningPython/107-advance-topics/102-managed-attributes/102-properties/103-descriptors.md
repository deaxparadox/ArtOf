# Descriptors

Descriptros propvide an alternative way to inercept attribute access; they are strongly related

- the **property** built-in is just a simplified way to create a specific type of descriptor that runs methods function on attribute accesses.

The descriptor protocol allows us to route a specific attribute get, set, and delete operations to methods of a separate class's instance object that we provide.

- This allows us to insert code to be run automatically on atribute fetches and assignments, intercept attribute deletions, and provide documentation for the attributes if desired.

### Descriptors:

- created as independent *classes*
- assigned to class atributes just like method functions.
- they are inherited by sub classes and instance.
- their access-interception methods are provided with both a **self** for the descriptor instance itself, as well as the instance of the client calss whose attribute references the descriptor object. Because of this, they can retain and use state information of their own, as well as state information of the subject instance.


