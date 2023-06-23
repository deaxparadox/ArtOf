# Control Flow

- When your programm contains more than one statement, the statements are executed from top to bottom.

```js
let theNumber = Number(prompt("Pick a number"));
console.log("Your number is the square root of " +
            theNumber * theNumber);
```

```output
Pick a number 15 
Your number is the square root of 225
```

- Step

    - The function takes input, using *prompt* function, **`prompt("Pick a number")`**
    
    - which is wrapped in side *Number* function, which convert the user input to number, **`Number(prompt("Pick a number"))`**
    
    - *Number* returns values is assigned to *theNumber*, **`let theNumber = Number(prompt("Pick a number"));**`

    - *theNubmer* perform multiplication to self, **`theNumber * theNumber`**
    
    - and preform automatic conversion to type *string*, to be concatenated with *string*, **`"Your number is the square root of " + theNumber * theNumber`**
    
    - which will be display on terminal, using *console.log*, **`console.log("Your number is the square root of " + theNumber * theNumber);`**

[<<<](103-BuiltinFunctions.md) ... [105](105-ConditionalExecution.md)