# JavaScript Fundamentals: Code Structure

### Statements

Statements are syntax constructs and commands that performs actions. 

We’ve already seen a statement, `alert('Hello, world!')`, which shows the message “Hello, world!”.

We can have as many statements in our code as we want. Statements can be separated with a semicolon.

For example, here we split “Hello World” into two alerts:

```js
alert('Hello'); alert('World');
```

Usually, statements are written on separate lines to make the code more readable:

```js
alert('Hello');
alert('World');
```

### Semicolons

A semicolons may be omitted in most cases when a line break exits.

This would also work:

```js
alert('Hello')
alert('World')
```

**In most cases, a newline implies a semicolon. But “in most cases” does not mean “always”!**

There are caess when a newline does not mean a semicolon. For example:

```js
alert(3 +
1
+ 2);
```

The code outputs `6` because JavaScript does not insert semicolons here. It is intuitively obvious that if the line ends with a plus `"+"`, then it is an “incomplete expression”, so a semicolon there would be incorrect. And in this case, that works as intended.

### Comments

As time goes on, programs become more and more complex. It becomes necessary to add comments which describe what the code does and why.

Comments can be put into any place of a script. They don’t affect its execution because the engine simply ignores them.

**One-line comments start with two forward slash characters** `//`.

The rest of the line is a comment. It may occupy a full line of its own or follow a statement.

```js
// This comment occupies a line of its own
alert('Hello');

alert('World'); // This comment follows the statement
```

**Multiline comments start with a forward slash and an asterisk `/*` and end with an asterisk and a forward slash `*/`.**

```js
/* An example with two messages.
This is a multiline comment.
*/
alert('Hello');
alert('World');
```