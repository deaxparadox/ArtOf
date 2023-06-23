# Functions as Values

A function binding usually simply acts as a name for a specific piece of the program. Such a binding is defined once and never changed. This make it easy to confuse the function and its name.

```js
let launchMissiles = function () {
  console.log(1);
  return 2;
};


console.log(typeof launchMissiles);

// 1
// 2
```