# Square brackets

For multiword properties, the dot access doesn't work:

```js
// this would give a syntax error
user.likes birds = true
```

Javascript doesn't understand that. It thinks that we address `user.like`, and then gives a syntax error when comes across unexpected `brids`.

The dot requries the key to be a valid variable indentifire. That implies: contains no spaces, doesn’t start with a digit and doesn’t include special characters (`$` and `_` are allowed).

There’s an alternative “square bracket notation” that works with any string:

```js
let user = {};

// set
user["likes birds"] = true;

// get
alert(user["likes birds"]); // true

// delete
delete user["likes birds"];
```

Now everything is fine. Please note that the string inside the brackets is properly quoted (any type of quotes will do).

Square brackets also provide a way to obtain the properly name as the result of any expression--as opposed a literal string--like from a variable as follows:

```js
let key = "likes birds";

// same as user["likes birds"] = true;
user[key] = true;
```

Here, the variable `key` may be calculated at run-time or depend on the user input. And then we use it to access the property. That gives us a great deal of flexibility.