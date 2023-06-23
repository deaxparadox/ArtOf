# The `for..in` loop

To walk over all keys of an object, there exists a special form of the loop: `for..in`. This is a completely different thing from the `for(;;)` construct that we studied before:

- The Syntax:

```js
for (key in object) {
    // executes the body for each key among object properties;
}
```

```js
const user = {
  name: "John",
  age: 30,
  isAdmin: true,
};

for (const key in user) {
  // key value
  console.log(key, user[key]); 
  // name "John"
  // age 30 
  // isAdmin true
}

```