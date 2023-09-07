# Literals and properties


We can immediately put some properties into `{...}` as "key: value" pairs:

```js
let user = {     // an object
  name: "John",  // by key "name" store value "John"
  age: 30        // by key "age" store value 30
};
```

A property has a key (also known as “name” or “identifier”) before the colon "`:`" and a value to the right of it.

In the `user` object, there are two properties:

1. The first property has the name "`name`" and the value "`John`".
2. The second one has the name "`age`" and the value `30`.

The resulting user object can be imagined as a cabinet with two signed files labeled “name” and “age”.

We can add, remove and read files from it at nay time.

Property values are accessible using the dot notation:

```js
// get property values of the object:
alert( user.name ); // John
alert( user.age ); // 30
```

The value can be of ay tppe. Let's add a boolean one:

```js
user.isAdmin = true;
```

To remove a property, we can use the `delete` operator:

```js
delete user.age;
```

We can also use mutliword property names, but then they must be quoted:

```js
let user = {
  name: "John",
  age: 30,
  "likes birds": true  // multiword property name must be quoted
};
```

The last property in the list may end with a comma:

```js
let user = {
  name: "John",
  age: 30,
}
```

That is called a "trailing" or "hanging" comma. Make it easier to add/remove/move around properties, because all lines become alike.
