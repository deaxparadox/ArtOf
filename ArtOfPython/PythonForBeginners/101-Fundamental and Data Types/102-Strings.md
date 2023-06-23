# Data Types

Strings in python are surrounded by either `single quotation` marks or `double quotation` marks.

Let create a string using `single quotation` mark:

```py
>>> 'Welcome to World of Python!'
'Welcome to World of Python!'
>>> 
```

Now let's do the same thing using `double quotation` mark:

```py
>>> "Welcome to World of Python!"
'Welcome to World of Python!'
>>> 
```

## Variables

Assigning a string to a variable is done with the variable name followed by an equal sign and the string:

```py
>>> a = 'Welcome to World of Python!'
>>> print(a)
Welcome to World of Python!
>>> 
```

`Backword slash \` can used to escape quotes:

```py
>>> a = 'let\'s go'
>>> 
>>> print(a)
let's go
>>> 
```

or

```py
>>> b = "Hello \"python\""
>>> 
>>> print(b)
Hello "python"
>>> 
```

<!--  -->
<!-- Advance -->
<!--  -->
### Escape Characters

Adding a new to the string using `\n`: 

```py
>>> 
>>> a = "line \n newline"
>>> 
>>> print(a)
line 
 newline
>>> 
```

Adding a tab space to the string using `\t`:

```py
>>> a = "line \t newline"
>>> 
>>> print(a)
line 	 newline
>>> 
```

## Length of string

Length of the string can be calculated using `len()` method:

```py
>>> a = "length of string"
>>> 
>>> print(len(a))
16
>>> 
```

`len()` method can used to calculate length of `string`, `list`, `tuple`, etc.


## Multi-line string

One way is using triple-quotes: `"""..."""` or `'''...'''`

```py
>>> a = """Multi
... line 
... string"""
>>>
>>> print(a)
Multi
line 
string
>>> 
```

In above case `EOF (or end-of-line)` is automatically. That's why while `print` function is display string on different line. 

To prevent `EOF` use `escape` character:
 

```py
>>> a = """Multi \
... line \
... string."""
>>> 
>>> print(a)
Multi line string.
>>> 
```


## Concatenation

Strings can be concatenated  with the `+` operator, and repeated with `*`:

Let's first concatenate string:

```py
>>> a = "Hello, " + "Python!"
>>> 
>>> print(a)
Hello, Python!
>>> 
```

Now let's repeated string with `*` operator:

```py
>>> a = "Hello" * 3
>>> 
>>> print(a)
HelloHelloHello
>>> 
```





## Basic string formatting

String can is used to place another string later, in the code:

Let's first create a basic string to format, using `{}` curly braces:

```py
>>> a = "my name is {}"
>>> a
'my name is {}'
>>> 
```

when we use `format()` method of variable `a`, curly braces will be replace with the another string.

Let's create another string in variable `name`, which will be formated in string `a`:

```py
>>> name = "python"
```

Now let's format the string:

```py
>>> a.format(name)
'my name is python'
>>> 
```

Using `format` method, formats the `name` string in string `a`.


[next](103-IndexingWithStrings.md)