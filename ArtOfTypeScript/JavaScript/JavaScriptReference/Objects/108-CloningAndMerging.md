# Cloning and merging, Object.assign

We can create a new object and replicate the structure of the existing one, by iterating overits properties and copying them on the primitive level.

```javascript
let user = {
  name: "John",
  age: 30
};

let clone = {}; // the new empty object

// let's copy all user properties into it
for (let key in user) {
  clone[key] = user[key];
}

// now clone is a fully independent object with the same content
clone.name = "Pete"; // changed the data in it

alert( user.name ); // still John in the original object
```

We can also use the method `Object.assign`:


```javascript
Object.assign(dest, ...sources)
```

- The first argument `dest` is a target object.
- further arguments is a list of a source objects.

- This copies properties of all source objects into the target dest, and them returns it an the result.
- For example, we have `user` object, let's added a couple or permissions to it:

```javascript
let user = { name: "John" };

let permissions1 = { canView: true };
let permissions2 = { canEdit: true };

// copies all properties from permissions1 and permissions2 into user
Object.assign(user, permissions1, permissions2);

// now user = { name: "John", canView: true, canEdit: true }
alert(user.name); // John
alert(user.canView); // true
alert(user.canEdit); // true
```

- If the copied property name already exists, it gets overwritten.

```javascript
let user = { name: "John" };

Object.assign(user, { name: "Pete" });

alert(user.name); // now user = { name: "Pete" }
```

- We also can use `Object.assign` to perform a siple object cloning:

```javascript
let user = {
  name: "John",
  age: 30
};

let clone = Object.assign({}, user);

alert(clone.name); // John
alert(clone.age); // 30
```

- Here it copies all properties of `user` into the empty object and returns it.

We can alse cloning using spread operator.

```javascript
> let user = {
name:"John", age:30}
undefined
> 
> let clone = {...user}
>
> clone
{ name: "John", age: 30 }
> 
```

[<<< Reference and copying](107-ReferencesAndCopying.md) ... [Nested Cloning >>>](109-nest-cloning.md)