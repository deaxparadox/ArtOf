# `match` Statements

A `match` statement takes an expression and compares its value to successive patterns as one or more case blocks. 

```python
status = 200

match status:
    case 200:
        print("Successfull")
    case 300:
        print("Redirected")
    case _:
        print("Unknown!")

# output
SuccessFull
```

- You can combine several literals in a single pattern using `|` ("or"):

```python
status = 302

match status:
    case 200:
        print("Successfull")
    case 300 | 301 | 302:
        print("Redirected")
    case _:
        print("Unknown!")
```

## Examples

There are more usefull cases where we can use `match` statement. Let's write a simple program.

In our program user enters text commands to interact with fictional world and recevies text descriptors of what happens. Commands will be simplified forms of natural language like `get sword`, `attack dragon`, `go north`, `enter shop` or `buy cheese`.

Your main loop will need to get input from the user and split it into words, let’s say a list of strings like this:

```py
command = input("What are you doing next? ")
# analyze the result of command.split()
```

The next step is to interpret the words. Most of our commands will have two words: an action and an object. 

```
[action, obj] = command.split()
... # interpret action, obj
```

The match statement evaluates the **“subject”** (the value after the `match` keyword), and checks it against the **pattern** (the code next to `case`). A pattern is able to do two different things:

- Verify that the subject has certain structure. In your case, the `[action, obj]` pattern matches any sequence of exactly two elements. This is called **matching**.
- It will bind some names in the pattern to component elements of your subject. In this case, if the list has two elements, it will bind `action = subject[0]` and `obj = subject[1]`.

### Matching multiple patterns


```py
match command.split():
    case [action]:
        print("interpret single-verb action ")
    case [action, obj]:
        print("interpret action, object")
```

let's run our program:

```sh
$
$ python match.py 
What are you doing next? attack
interpret single-verb action 
$
$ python match.py 
What are you doing next? attack dragon
interpret action, object
```

when we specific action only we get *"interpret single-verb-action"* and when we specify action and object we get *'interpret action, object"*.


### Matching specific values

