# if, if..else, Nested if, if-elif Statments

There comes situations in real life when we need to do some specific task and based on some specific conditions and, we decide what should we do next. 

Similarly there comes a situation in programming where a specific task is to be performed if a specific condition is `True`. In such cases, conditional statements can be used. 

The following are  the conditional statements provided by Python:

# if Statement

Simple Example:

```py
if 10 > 5:
    print("10 greather than 5")
```

output :

```opt
10 greather than 5
```



# if..else Statement

In conditional if Statement the additional block of code is merged as else statement which is performed when if condition is false.

Syntax:

```py
if (condition):
    # Executes this block if 
    # condition is true
else:
    # Executes this block if 
    # condition is false
```

example_1:

```py
a = 10
b =  15
if a > b:
    print("a is greator than b")
else:
    print("a is less than b")
    
print("Exiting...")
```

```output
a is less than b
Exiting...
```



example_4:

```py
a = "linux"

if a == "linux":
    print("My OS is Linux.")
else:
    print("My OS is unkonwn.")

print("Exiting")
```

```output
My OS is Linux.
Exiting
```




# if-elif-else Statements

Syntax:

```py
if (condition1):
    # if condition1 is True
    # Execute this block
elif (condition2):
    # if condition2 is True
    # Execute this block
else:
    # all condition is False
    # then, execute this block
```

example_1:

```py
letter = "A"
 
if letter == "B":
    print("letter is B")
 
elif letter == "C":
    print("letter is C")
 
elif letter == "A":
    print("letter is A")
 
else:
    print("letter isn't A, B or C")
```

```output
letter is A
```

You can remove `else`, it's not compulsary. You can also next `if-elif-else` statements.