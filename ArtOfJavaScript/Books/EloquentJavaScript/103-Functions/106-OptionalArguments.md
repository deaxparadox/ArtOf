# Optional Argument

The following code is allowed and executes without any problem:

```js
const square = (x) => {
  return x * x;
}
console.log(square(25));
// → 625
```

We defined *square* with only one parameter. Yet when we call it with three, the language doesn't complain. It ignores the extra arguments and computes the square of the first one.

- If you pass to many, the extra ones are ignored.

```js
const square = (x) => {
  console.log(x);
}

square(5, true, "Rohit");

// 5
```

- If you pass to few, the missing parameters get assigned the value *undefined*.

```js
const square = (x) => {
  console.log(x);
}

square();

// undefined
```