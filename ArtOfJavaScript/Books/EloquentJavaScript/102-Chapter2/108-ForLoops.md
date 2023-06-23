# `for` Loops 

Let's first see an example:


```js
for (let number=0; number<=12; number=number+2) {
    console.log(number);
}
```

```output
0
2
4
6
8
10
12
```

The parentheses after a *for* keyword contain two semicolons. This part before the first semicolon *initializes* the loop, usually be defining a binding. The second part is the expression that *checks* whether the loop must continue. The final part *updates* the state of the loop after every iteration. 

```js
let result = 1;
for (let counter = 0; counter < 10; counter = counter + 1) {
  result = result * 2;
}
console.log(result);
// → 1024
```