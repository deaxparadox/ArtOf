# While and Do loops

*Loop* is a form of piece of code that run multiple times. Looping control f low allows us to go back to some point in the program where we were before and repeat it with our current program state.

Let's dive in example:

```js
let number = 0;
while (number <= 12) {
  console.log(number);
  number = number + 2;
}

// → 0
// → 2
//   … etcetera
```

# `while`

A statement starting with the keyword *while* creates a loop. This word *while* is followed by an expression in parentheses and then a statement, much like *if*. The loop keepps entering that statement as long as the expression produces a value that gives *tru* when converted to Boolean.

Let's see an another example:

```js
let result = 1;
let counter = 0;
while (counter < 10) {
  result = result * 2;
  counter = counter + 1;
}
console.log(result);
```

```ouptut
1024
```




