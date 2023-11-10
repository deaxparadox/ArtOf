# Null safety

The Dart language enforces sound null safety.

Null safety prevents an error that results from unintentional access of variables set to `null`. The error is called a null dereference error. A null dereference error occurs when you access a property or call a method on an expression that evaluates to null. An exception to this rule is when `null` supports the property or method, like `toString()` or `hashCode`. With null safety, the Dart compiler detects these potential errors at compile time.

For example, say you want to find the absolute value of an `int` variable `i`. If `i` is `null`, calling `i.abs()` causes a null dereference error. In other languages, trying this could lead to a runtime error, but Dart’s compiler prohibits these actions. Therefore, Dart apps can’t cause runtime errors.

Null safety introduces three key changes:

1. When you specify a type for a variable, parameter, or another relevant component, you can control whether the type allows `null`. To enable nullability, you add a `?` to the end of the type declaration.

```dart
String? name  // Nullable type. Can be `null` or string.

String name   // Non-nullable type. Cannot be `null` but can be string.
```

2. You must initialize variables before using them. Nullable variables default to `null`, so they are initialized by default. Dart doesn’t set initial values to non-nullable types. It forces you to set an initial value. Dart doesn’t allow you to observe an uninitialized variable. This prevents you from accessing properties or calling methods where the receiver’s type can be null but null doesn’t support the method or property used.

3. You can’t access properties or call methods on an expression with a nullable type. The same exception applies where it’s a property or method that null supports like `hashCode` or `toString()`.

Sound null safety changes potential **runtime errors** into **edit-time** analysis errors. Null safety flags a non-null variable when it has been either:

- Not initialized with a non-null value.
- Assigned a null value.

This check allows you to fix these errors *before* deploying your app.