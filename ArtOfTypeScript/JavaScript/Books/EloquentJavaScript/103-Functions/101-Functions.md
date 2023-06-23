# Defining a Function

A function definition is a regular binding where the value of the binding is a functino. 

Let' dive example, that defines a *square* to refere to a function that produces the square of a given number:

```js
const square = function(x) {
  return x * x;
};

console.log(square(12));
// → 144
```
 
A function is craeted with an expression that starts with the keyword *function*. Function has a set of *parameters* and a *body*, which contains the statemetns that are to be executed when the function is called. The function body of a function created this way myust always be wrapped in braces, even when it consists of only a single statement.


- function with on parameter:

```js
const makeNoise = function() {
  console.log("Pling!");
};

makeNoise();
// → Pling!
```

- function with multiple parameter:

```js
const power = function(base, exponent) {
  let result = 1;
  for (let count = 0; count < exponent; count++) {
    result *= base;
  }
  return result;
};

console.log(power(2, 10));
// → 1024
```

A *return* statement determines the value the function returns. When control come across suc has statement, it immediately jumps out of the current funciton and gives the returned value to the code that called the function.