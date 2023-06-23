# Boolean Values

It is often usefull to have a value that distinguishes between only two possibilities, like "yes" and "no" or "on" and "off". For this purpose, JavaScript has a *Boolean* type, which has just two values, true, and false, which are written as those words.

### Comparison

```js
> console.log(3 > 2)
true
>
``` 

```js
> console.log(3 < 2)
false
> 
```

This *>* and *<* signs are the traditional symbols for "is greater than" and "is less than". They are binary operators. Applying them results in a Boolean value that indicates whether they hold true in this case.

Strings can be compared in the same way.

```js
> console.log("Aardvark" < "Zoroaster")
true
```

Using comparison operator in string is not much of as use case.



Other similar operators are 

```js
// >= (greater than or equal to)
> console.log(3 >= 3)
true
>
> console.log(3 >= 4)
false
>
> console.log(3 >= 2)
true
> 
```


```js
// <= (less than or equal to)
> console.log(3 >= 3)
true
>
> console.log(3 >= 4)
true
>
> console.log(3 >= 2)
false
> 
```

```js
// == (equal to), and != (not equal to)
> console.log("Itchy" != "Scratchy")
true
>
> console.log("Apple" == "Orange")
false
```

There is only one value in JavaScript that is not equal to itself, and that is **NaN** ("not a number").

```js
> console.log(NaN==NaN)
false
```

NaN is supposed to denote the result of a nonsensical computation, and as such, isn't equal to the result of any *other* nonsensical computations.


[<<< Unary Operator](103-UnaryOperators.md) ... [Logical Operators >>>](105-LogicalOperators.md)