# Arrow Functions

In this notation, it uses an arrow (=>) made up of an equal sign and a greater-than character.


```js
const power = (base, exponent) => {
  let result = 1;
  for (let count = 0; count < exponent; count++) {
    result *= base;
  }
  return result;
};

console.log(power(2, 3));

// 8
```

The arrow comes *after* the list of parameters an is followed by the function's body.

Let's see another of arrow fucntion with one argument:

```js
const square = (x) => {
  return x * x;
}

console.log(square(25));

// 625
```

You can also create above square as:

```js
const square = (x) => x * x;

console.log(square(25));

// 625
```

- Arrow function accepting no argument:

```js
const horn = () => {
  console.log("Toot");
};

horn();

// Toot
```