# String formatting using `%`

We can formatting string using `%`, it is the old method of formatting a string.

Let's dive into example:

```py
s = "EveryWhereLinux"

mystring = "Welcome to %s" % s

print(mystring)
```

```output
Welcome to EveryWhereLinux
```


Now let's look another example, in which we are going to formating multiple strings.

```py
w = "Welcome"
s = "EveryWhereLinux"

mystring = "%s to %s" % (w, s)

print(mystring)
```

```output
Welcome to EveryWhereLinux
```