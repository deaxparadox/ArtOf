# Final and const

If you never intend to change a variable, use `final` or `const`, either instead of `var` or in addition to a type. A final variable can be set only once; a const variable is a compile-time constant. (Const variables are implicitly final).

----------

**Note**: Instance variables can be `final` but not `const`.

----------

Here's an example of creating and setting a `final` variable:

```dart
final name = "Bob";     // Without a type annotation
final Stirng nickname = "Bobby";
```

You can't change the value of final variable:

```dart
name = 'Alice'; // Error: a final variable can only be set once.
```

Use `const` for variables that you want to be **compile-time constants**. If the const variables is at the class level, mark it `static const`. Where you declare the variable, set the value to a compile-time constant such as a number or string literal, a const variable, or the result of an arithmetic operation on constant numbers.

```dart
const bar = 1000000; // Unit of pressure (dynes/cm2)
const double atm = 1.01325 * bar; // Standard atmosphere
```

The `const` keyword isn't just for declaring constant variables. You can also use it to create constant *values*, as well as to declare constructors that *create* constant values. Any variables can have a constant value.

```dart
var foo = const [];
final bar = const [];
const baz = []; // Equivalent to `const []`
```

You can omit `const` from the intializing expression of a `const` declaration, like for `baz` above.

You can change the value of a non-final, non-const variable, even if it used to have a `const` value:

```dart
foo = [1, 2, 3]     // Was const []
```

You can’t change the value of a `const` variable:

```dart
baz = [42]; // Error: Constant variables can't be assigned a value.
```

You can define constants that use `type checks and casts` (`is` and `as`), `collection if`, and `spread operators` (... and ...?):

```dart
const Object i = 3; // Where i is a const Object with an int value...
const list = [i as int]; // Use a typecast.
const map = {if (i is int) i: 'int'}; // Use is and collection if.
const set = {if (list is List<int>) ...list}; // ...and a spread.
```

**Note:** Although a final object cannot be modified, its fields can be changed. In comparison, a const object and its fields cannot be changed: they’re immutable.