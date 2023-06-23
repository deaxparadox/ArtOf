# `do-while` loop

A *do* loop is a control structure similar to a while loop. It differs only on one point: a do loop always execute its body at least once, and it starts testing whether it should stop only after that first execution. To reflect this, the test appears after the body of the loop.


```js
let yourName;
do {
  yourName = prompt("Who are you?");
} while (!yourName);
console.log(yourName);
```

```output
Who are you? paradox
paradox
```

This programm will ask you to enter a name. It will ask again and again until it gets something t hat is not an emtpy string.

Applying the `!` operator will convert a value to Boolean type before negating it, and all strings except "" convert to *true*. This means the loop continues going round until you provide a non-empty name.