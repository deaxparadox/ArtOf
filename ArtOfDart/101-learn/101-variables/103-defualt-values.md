# Default value

Uninitialize variables that have a nullable type have an initial value of `null`. Even variables with numeric types are intially null, because numbers--like everything else in Dart--are objects.

```dart
  int? lineCount;
  assert(lineCount == null);
```

----------

Note: Production code ignores the assert() call. During development, on the other hand, assert(condition) throws an exception if condition is false.

----------


With null safety, you must intialize the values of non-nullable variables before you use them:

```dart
int lineCount = 0;
```

You don't have to intialize a local variable where it's declared, but you do need to assign it a value before it's used. For example, the following code is valid becuase Dart can detect that `lineCount` is non-null by the time it's passed to `print()`:

```dart
int lineCount;

if (weLikeToCount) {
    lineCount = countLines();
} else {
    lineCount = 0;
}

print(lineCount);
```

Top-level and class variables are lazily initialized; the initialization code runs the first time the variable is used.

