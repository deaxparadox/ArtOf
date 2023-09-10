<!-- while -->
<!-- while break -->
<!-- while continue -->

# `while` Loop

The `while` statement is used for repested execution as long as an expression in true:

Syntax 

```py
while (condition):
    # if condition is True
    # code block
```

Execute code block as long as condition remain True.


example_1:

```py
while True:
    print("In a loop")
```

Loop while keep on executing, press `ctrl+c` to stop the loop.


example_2:

```py
count = 0
while count < 10:
    print(count)
    count = count + 1
```

```output
0
1
2
3
4
5
6
7
8
9
```

example_3:

```py

a = True
count = 0
while a:
    if count >= 10:
        a = False
    else:    
        print(count)
        count = count + 1
```

```output
0
1
2
3
4
5
6
7
8
9
```

## Controlling while with `break`

As we have seen in `for` loop, we break a loop using `break` statement if given condition is True.


```py
count = 0
while count < 10:
    if count == 5:
        break
    print(count)
    count = count + 1
```

```output
0
1
2
3
4
```

## Controlling while with `continue`

Let's now see how can we use `continue` statement:

```py

count = 0
while count < 10:
    if count >= 4 and count < 8:
        count = count + 1
        continue

    print(count)
    count = count + 1
```

```output
0
1
2
3
8
9
```