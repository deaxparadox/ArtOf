# Numbers

The number such as 2,4,20,... have type `int()`, the ones with a fractional part such as 5.0, 1.6 have type `float()`

Let's see some example related to numbers:

```py
>>> 2 + 2
4
>>> 50 - 5*6
20
>>> (50 - 5*6) / 4
5.0
>>> 8 / 5  # division always returns a floating point number
1.6
```

As shown above division always returns a `float()` type or floating point number. 

When we use `/` for division, it's known as `classic division` which returns float.

To do `floor division` you can use `//` operator. `floor division` rounds down to nearest integer. 

Let's see some examples:

```py
>>> 11//4
2
>>> (-11)//4
-3
>>> 17//3
5
>>> 
```

We have addition, subtraction, multiplicaion, division. Now it's time to see how to calculate remainder.

To calculate the remainder use `%` operator:

```py
>>> 17%3
2
>>> 
```

In Python it's possible to calculate power using operator, to calculate square of 5, using `**` operator:

```py
>>> 5**2
25
>>> 
```

or cube of 5:

```py
>>> 5 ** 3
125
>>> 
```

Similarly we can calculate value of any power.

## Variables

It times to create varaible of numbers:

```py
>>> width = 20
>>> height = 5 * 9
>>> width * height
900
```

There is full support for floating point; operators with mixed type operands convert the integer operand to floating point:

```py
>>> 4 * 3.75 - 1
14.0
```

[next](106-Boolean.md)