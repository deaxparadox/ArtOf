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