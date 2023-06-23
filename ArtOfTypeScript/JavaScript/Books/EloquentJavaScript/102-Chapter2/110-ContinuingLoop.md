# Continuing Loop

The *continue* keyword isimilar to *break*. When *continue* is encountered in a loop body, control jumps out of the body and continues with the loop's next iteration.

```js
for (let number = 0; number < 10; number = number + 1) {
    if (number == 5) {
        continue
    }
    console.log(number)
}
```

```output
0
1
2
3
4
6
7
8
9
```

We are applying *continue* when *number* becomes 5. Therefore, 5 is not in output, because because *console.log* is executed continue jumps to next iteration.