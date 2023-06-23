# String formatting using `format()`

We can use string `format()` function, to format a string:

Create a string:

```py
s = "EveryWhereLinux"

mystring = "{} is awesome".format(s)

print(mystring)
```

```output
EveryWhereLinux is awesome
```


## `format()` with Indexing

String can also be formatted using, using index position of element inside format function


### Formatting without indexing.


```py
s = "EveryWhereLinux"
a = "awesome"

mystring = "{} is {}".format(s, a)

print(mystring)
```

```output
EveryWhereLinux is awesome
```

### Formatting with indexing.

Example 1:

```py
s = "EveryWhereLinux"
a = "awesome"

mystring = "{0} is {1}".format(s, a)

print(mystring)
```

```output
EveryWhereLinux is awesome
```

Example 2:


```py
s = "EveryWhereLinux"
a = "awesome"

mystring = "{1} is {0}".format(s, a)

print(mystring)
```

```output
awesome is EveryWhereLinux
```

