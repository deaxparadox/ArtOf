# Bindings

## `let` keywords

- Syntax:

```js
let name = <value> or <expression>
```

- To catch and hold values, JavaScript provides a thing called a binding, or variable:

```js
let caught = 5 * 5;
```

The `let` keyword indiciates that this sentence is going to define a binding.

- After a binding has been defined, its name can be used as an expresssion. The value of usch an expression is the value the binding currently holds. Here's an example:

```js
let ten = 10;
console.log(ten * ten)
```

- The `=` operator can be used at any time on existing bindings to disconnect them from their current value and have them point to a new one. While doing the rebinding let operator is not required:

```js
> let mood = "light";
> console.log(mood);
light
> mood = "dark";
"dark"
> console.log(mood);
dark
```

Another example:

```js
let luigisDebt = 140;
luigisDebt = luigisDebt - 35;
console.log(luigisDebt);

// 105
```

- When you define a binding without giving it a value. If you ask for the value of an emtpy binding, you'll get the value `undefined`.


- A single `let` statement may define multiple bindings. The definitions must be separated by commas:

```js
let one = 1, two = 2;
console.log(one + two);

// 3
```

# `var` keyword

The first `var` (short for "varaible"), is the way bindings were declared in pre-2015 JS.


# `const` keyword

The keyword `const` stands for contant. It defines a constant binding, which points at the same value for as long as it lives.

```js
const a = 5;
console.log(a);

// changing a constant binding value
a = 7

// error: Uncaught TypeError: Assignment to constant variable.
```

we will get `TypeError`.

[>>>](103-BuiltinFunctions.md)