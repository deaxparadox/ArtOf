# Property existence test, "in" operator

A notable featuer in objects in JavaScript, compared to many other languages, is that it's possible to access any property. There will be no error if the property doesn't exist!.

Reading a non-existing property just returns `undefined`. SO we can easitly test whether the property exists:

```js
let user = new Object();
console.log(user.noSuchProperty);
console.log(user.noSuchProperty === undefined)

// undefined
// true
```

- There's also a special operator "in" for that.

```js
let user = new Object();

console.log("noSuchProperty" in user);
// output: false

// adding new property
user.name = "EveryWhereLinux";

console.log("name" in user);
// output: true
```

Return false mean property doesn't exist, while true is considered as property exists.