# Late Variables

The `late` modifier has two use cases:

- Declaring a non-nullable variable that's initialized after its declaration.
- Lazily intializing a variable.

Often Dart's control flow analysis can detect when a non-nullable variable is set to a non-null value before it's used, but sometimes analysis fails. Two common cases are top-level variables and instance variables: Dart often can't determine whether they're set, so it doesn't try.

If you're sure that a variable is set before it's used, but Dart disagrees, you can fix the error by marking the variable as `late`:

```dart
late String description;

void main() {
  description = 'Feijoada!';
  print(description);
}
```

`If you fail to initialize a late variable, a runtime error occurs when the variable is used.
`

----------


When you mark a variable as `late` but intialize it as its declaration, then the intializer runs the first time the variable is used. This lazy initialization is handy in a couple of cases:

- The variable might not be needed, and intializing it is costly.
- You're initializing an instance variable, and its initializer needs access to `this`.

In the following example, if the `temperature` variable is never used, then the expensive `readThermometer()` function is never called:

```dart
// This is the program's only call to readThermometer().
late String temperature = readThermometer(); // Lazily initialized.
```