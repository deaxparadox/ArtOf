# Closure


In the following exmaple. It defines a function, *wrapValue*, that creates a local binding. It then returns a function that accesses and returns tthis local binding:


```js
function wrapValue(n) {
  let local = n;
  return () => local;
}

let wrap1 = wrapValue(1);
let wrap2 = wrapValue(2);
console.log(wrap1());
// → 1
console.log(wrap2());
// → 2
```

This is allowed and works as you'd hope--both instances of the binding can still be accessed. This situation is a good demonstration of the fact that local bindings are created anew for everycall.

This feature--being able to reference a specific instance of a local binding in an enclosing scope--is called *closure*. 

- A function that references bindings from local scopes around it is called a closure.

Another example of closure:

```js
function multiplier(factor) {
  return number => number * factor;
}

let twice = multiplier(2);
console.log(twice(5));
// → 10
```