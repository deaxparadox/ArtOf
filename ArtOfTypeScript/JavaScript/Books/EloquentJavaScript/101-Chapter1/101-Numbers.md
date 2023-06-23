# Numbers

Values of the *number* type are, numeric values. 

### Integers

Regular integers in mathematics are actual data type in JavaScript, such as -3, -2, -1, -, 1, 2, etc.

```
13
```

### Floats 

Floats are decimal integers.

Fractional numbers are written by using dot.

```js
9.81
```

For very bit or very small numbers, you may also use scientific notation by adding an *e* (for exponent), followed by the exponent of the number.

```js
2.998e8
```

That is 2.999 x 10^8 = 299,800,000.

## Arithmetic Operations

### Addition 

Uses *+* operator:

```js
100 + 4
```

### Muliplication

Uses *\** operator:

```js
100 * 4
```

### Subtraction

Uses *-* operator:

```js
100 - 4
```

### Division

Uses */* operator:

```js
100/4
```

### Remainder

Uses *%* operator:

```js
100%3
```

### Order of *precedence*

In JavaScript arithmetic *\** and */* are of same precedence, which mean they are are execute before applied together with *+* and *-*.

Here *\** is exeuted first, then after that *+*:

```js
100 + 4 * 11
```

Here */* is executed first, then *+*, you can also replace *+* with *-*:

```js
100 + 4 / 11
```

And where */* and *\** are applied together, they executed from left to right.


```js
> 5 * 3 / 2
7.5
```

```js
> 5 / 3 * 2
3.3333333333333335
```

There is also the way to by pass the order of precedence using *parentheses* `()`.

```js
// without parentheses
> 100 + 4 / 11
100.36363636363636
```

```js
// with parentheses
> (100 + 4) / 11
9.454545454545457
> 

```


### Special Numbers

There are three special values in JavaScript that are considered number but don't behave like normal numbers.

The first two are **Infinity** and **-Infinity**, which represent positive and negative infinities.

```js
> Infinity - 1
Infinity
```

**NaN** stands for *not a number*, event though it is value of number tyep.

```js
> 0 / 0
NaN
```

[<<< Values](100-Values.md) ... [Strings >>>](102-Strings.md)