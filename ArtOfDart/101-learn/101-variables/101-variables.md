# Variables

An example of creating a variable and initializing it:

```dart
var name = "Bob";
```

Variables store references. The variable called `name` contains a reference to a `String` object with a value of “Bob”.

The type of the `name` variable is inferred to be `String`, but you can change that type by specifying it. 

----------

If `int` (integer) in assign to name: 

```dart
void main() {
  var name = "Bob";

  <!-- // error -->
  name = 23341;     

  print(name);
}
```

will display error: 

```bash
Error: A value of type 'int' can't be assigned to a variable of type 'String'.
```

### Dynamic Variables

If an object isn’t restricted to a single type, specify the `Object` type (or dynamic if necessary).

```dart
void main() {
  Object name = "Bob";
  name = 123;
  print(name);
}
```

Another way to create `dynamic` type variables:

```dart
    dynamic obj2 = "This is string2";
    obj2 = 123;
    print(obj2);
```

----------

### Strict type Variable

To explicitly declare the type that would be inferred:

```dart
  String name2 = "Bob";
```