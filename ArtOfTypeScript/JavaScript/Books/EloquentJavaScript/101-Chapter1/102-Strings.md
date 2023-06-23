# Strings

Strings are used to represent text. They are written by enclosing their content in quotes.

```js
// using `` (backticks)
> `Down on the sea`
"Down on the sea"

// using "" (double quotes)
> "Lie on the ocean"
"Lie on the ocean"

// using '' (single quotes)
> 'Float on the ocean'
"Float on the ocean"
```

You can use single quotes, double quotes, or backticks to mark strings, as long as the quotes at the start and the end of the string match.

And we have also added a line start with *//* before every string. These are called comments. Comments are placeholder text for describing you code, like we have used.

### `escaping` the character

Whenever a backlast *(\)* is found inside quoted text, it indicates that the character after it has a special meaning. This is called *escaping* the character. 

When an **n** character occurs after a backlash, it is interpreted as a newline.

```js
> "This is first line\nAnd this is the new line"
"This is first line\nAnd this is the new line"
> 
> console.log("This is first line\nAnd this is the new line")
This is first line
And this is the new line
> 
```

We have used `\n` character in string. Then we have used `console.log()` function to format the string. It is also used to display output in the terminal.

Similary, a **t** after a backlash means a tab character. 

```js
> "This is first line\tAnd this is the new line"
"This is first line\tAnd this is the new line"
> 
>
> console.log("This is first line\tAnd this is the new line")
This is first line	And this is the new line
> 
```

### Concatenation

*Concatenation* means the *+* operation can be used on strings:

```js
> "con" + "cat" + "e" + "nate"
"concatenate"
> 
```

### *backtick`

*Backtick` quoted strings, usually called *template literals*, can do a few more tricks. Apart from being able to span lines, they can also embed other values.

```js
> `half of 100 is ${100/2}`
"half of 100 is 50"
> 
```


[<<< Numbers](101-Numbers.md) ... [Unary Operators >>>](103-UnaryOperators.md)