# Breaking out of a Loop

*break* statement has the effect of immediately jumping out of the enclosing loop.

Let's dive in example: it finds the first number that is both greater than or equal to 20 and divisible by 7:

```js
for (let current = 20; ; current = current + 1) {
  if (current % 7 == 0) {
    console.log(current);
    break;
  }
}
// → 21
```

If you were to remove the *break* statement or yo uaccidentally wirte an end condition that always produces *true*, your program would get stuch in an *inifinite* loop. 