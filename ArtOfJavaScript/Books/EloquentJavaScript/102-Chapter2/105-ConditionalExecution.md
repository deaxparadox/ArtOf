# Conditional Execution

Conditional execution is created with the `if` keyword in JavaScript.

```js
let theNumber = Number(prompt("Pick a number"));

if (!Number.isNaN(theNumber)) {
  console.log(
    "Your number is the square root of " +
      theNumber * theNumber,
  );
}
```

If you enter "parrot", no output is shown.

The `if` keyword executes or skips a statement depending on the value of a Boolean expression. The deciding expression is written after the keyword, between parentheses,followed by the statement to execute.

The *Number.isNaN* function is a standard function that returns true only if the argument is given is NaN. The *Number* function happens to return NaN when you given it a string that doesn't reptresent a valid number. Thus the condition translates to "unless theNumber is not-a-number, do this".

The statement after the if is wrapped in braces ({ and }). The braces container is also called *block* which contain the codes after the condition is true.

You can use the *else* keyword, together with *if*, to create alternate block, which execute on false condition.

```js
let theNumber = Number (prompt("Pick a number: "))
if (!Number.isNaN(theNumber)) {
    console.log("Your number is the square root of " + theNumber * theNumber);
} else {
    console.log("Hey. Why don't you give me a number?");
}
```

```output
Pick a number:  sdf
Hey. Why don't you give me a number?
```

You can "chain" multiple *if/else* pairs together:

```js
let num = Number(prompt("Pick a number"));

if (num < 10) {
  console.log("Small");
} else if (num < 100) {
  console.log("Medium");
} else {
  console.log("Large");
}
```


```output
$ deno run test.js
Pick a number 3
Small
```

```ouptut
$ deno run test.js 
Pick a number 435
Large
```