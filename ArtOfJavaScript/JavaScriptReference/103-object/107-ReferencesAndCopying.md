# Object references and copying

One of the fundamental differences of objects vesus primitives is the objects are stored and copied "by reference", whereas primitive values: strings, numbers, booleans, etc are always copied "as a whole value".

Let's understand with an example:

```js
let message = "Hello!"
let phrase = message;
```

As a result we have two independent variables, each one storing the string "Hello!".

Object are not like that.

**A variable assgined to an object stores not the object itself, but its *address in memory* - in other words *a reference* to it**.

Let's look at an example of such a varaible:

```js
let user = {
    name: "John"
}
```

We have two varaibles, each storing a reference to the same object:

We can use eitehr varaible to access the object modify its contents.

```js
let user = { name: 'John' };

let admin = user;

admin.name = 'Pete'; // changed by the "admin" reference

consle.log(user.name); // 'Pete', changes are seen from the "user" reference
```

## Comparism by reference

Two objects are equal only if they are the same object:

For instance, here `a` and `b` reference the same object, thus they are equal:

```js
let a = {};
let b = a; // copy the reference

// true, both variables reference the same object
console.log( a == b ); 

// true
console.log( a === b ); 
```

For comparisms like `obj1 > obj2` or for a comparism against a primitive `obj==5`, objects are converted to primitives. 

[<<< For in loop](106-ForInLoop.md) ... [Cloning and Merging >>>](108-CloningAndMerging.md)