# Nested cloning 

Until now we assumed that all properties of `user` are primitive. 

- But properties can be references to other objects.

```javascript
let user = {
  name: "John",
  sizes: {
    height: 182,
    width: 50
  }
};

alert( user.sizes.height ); // 182
```

Now it's not enough to copy `clone.sizes == use.sizes`, because `user.sizes` is an object and will be copied by reference, so `clone` and `user` will share the same sizes.


```javascript
let user = {
  name: "John",
  sizes: {
    height: 182,
    width: 50
  }
};

let clone = Object.assign({}, user);

console.log( user.sizes === clone.sizes); // true, same object

// user and clone share sizes
user.sizes.width = 60;    // change a property from one place
console.log(clone.sizes.width); // 60, get the result from the other one
```

- To fix that make `user` and `clone` truly separate object, we should use a cloning loop that examines each value of `user[key]` and it it's an object, then replicate its structure as well.
- That is called "deep clonning" or "structure cloning".


[<<< Cloning and Merging in loop](108-CloningAndMerging.md) ... [Structured Clone >>>](110-structured-clone.md)