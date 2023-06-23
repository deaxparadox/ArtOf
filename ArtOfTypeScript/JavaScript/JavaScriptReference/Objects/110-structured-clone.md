# structured Clone

The call `structuredClone(object)` clones the `object` with all nested properties 

- Here's how we can use it in our example:

```javascript
let user = {
  name: "John",
  sizes: {
    height: 182,
    width: 50
  }
};

let clone = structuredClone(user);

console.log( user.sizes === clone.sizes ); // false, different objects

// user and clone are totally unrelated now
user.sizes.width = 60;    // change a property from one place
console.log(clone.sizes.width); // 50, not related
```

- The `structuredClone` method can clone most data types, such as objects, arrays, primitive values.
- it also supports circular refernces, when an object property refernces the object itself (directly or via a child or refrences).

- For instance:

```javascript
let user = {};
// let's create a circular reference:
// user.me references the user itself
user.me = user;

let clone = structuredClone(user);
console.log(clone.me === clone); // true
```

- As you can see, `clone.md` refrences the `clone`, not the `user`! So the circular refrences was cloned correctly as well.

- Although, there are cases when `structuredClone` fails.

- For instance, when an object has a function property:

```javascript
// error
structuredClone({
  f: function() {}
});
```

- Function properties aren't suported.