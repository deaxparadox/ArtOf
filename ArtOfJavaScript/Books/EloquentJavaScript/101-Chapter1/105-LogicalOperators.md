# Logical Operators 

There are also some operations that can be applied to Boolean values themselves. JavaScript supports three logical operators: *and*, *or*, and *not*. These can be used to "reason" about Booleans:

### `&&` operator

The ** operator represents logical *and*. It is a binary operator, and its result is true only both the values given to it are true.

```js
> console.log(true && true)
true
>
> console.log(false && false)
false
> 
> console.log(false && true)
false
```


### `||` operator

The || operator denotes logical *or*. It produces true if either of the values given to it is true.

```js
> console.log(true && true)
true
>
> console.log(false && false)
false
> 
> console.log(false && true)
true
```

### `!` operator

*Not* is written as an exclamation mark (!). It is a unary operator that flips the value given to it -- !true produces false, and !false given true.

```js
> console.log(!true)
false
>
> console.log(!false)
true
> 
```

[<<< Boolean](104-BooleanValues.md) ... [Conditional Operator >>> ](106-ConditionOperator.md)