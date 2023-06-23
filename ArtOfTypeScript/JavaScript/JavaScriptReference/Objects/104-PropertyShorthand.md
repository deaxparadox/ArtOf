# Property value shorthand

In real code, we often use existing variables as value sfor property names.

```js
function makeUser(name, age) {
    return {
      name: name,
      age: age,
      // ...other properties
    };
  }
  
let user = makeUser("John", 30);
console.log(user.name); 

// John
```

- There's a special *property value shorthand* to make it shorter:

```js
function makeUser2(name, age) {
    return {
        name,
        age,
    }
}

let user = makeUser2("John", 30);
console.log(user.name); 

// John
```


- We can use both normal properties and shorthands in the same object.

```js
function combined(name, age) {
    return {
        name,
        age: 12,
    }
}

let user = combined("John", 30);
console.log(user.name, user.age); // John

// John 12
```