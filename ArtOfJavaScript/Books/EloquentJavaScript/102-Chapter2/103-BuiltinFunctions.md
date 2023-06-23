<!-- prompt -->
<!-- console.log -->

# Functions: Builtin functions

- A function is a container block, which contains a piece of code, which can be executed anywhere after it's defined.

- Executing a function is called *invoking*, *calling*, or *applying* it.

### `prompt` function

The binding `prompt` holds a function that shows little dialog box asking for user input.

```js
const input = prompt("Enter passcode: ")
console.log(input);

// Enter passcode:  something
// something
```

We are accepting user input, using *prompt()* and display the user value in *terminal* or *browser console*.

Remember one thing, we are using `deno` runtime for running JS file.


### The `console.log()` function

- The `console.log()` function is used to output value in the `terminal` or `browser console`.

Let's look at previous example:

```js
const input = prompt("Enter passcode: ")
console.log(input);
```

```output
Enter passcode:  something
something
```

We are accepting user input, using *prompt()* and display the user value in *terminal* or *browser console*.


Let's look at another example:

```js
const a = 10;
console.log(a);
```

```output
10
```

[<<<](102-Bindings.md) ... [>>>](104-ControlFlow.md)