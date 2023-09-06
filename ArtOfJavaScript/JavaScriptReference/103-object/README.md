# Object: the basics

1. Objects
2. Object references and copying
3. Garbage collection
4. Object methods, "this"
5. Constructor, operator "new"
6. Optional chaining '?.'
7. Symbol type
8. Object to primitive conversion


## Objects

There are eight data types in JavaScript. Seven of them are called "primitive", because their values contain only a single thing (be it a string or a number or whatever).

In contrast, objects are used to store keyed collections of various data and more complex entities. In JavaScript, objects penetrate almost every aspect of the language. So we must understand them first before going in-depth anywhere else.

An object can be created with figure brackets `{...}` with an optional list of *properties*. A property is a "key: value" pair, where `key` is a string (also called a "property name"), and `value` can be anything.


----------

An empty object can be created using one of two syntaxes:

```js
let user = new Object(); // "object constructor" syntax
let user = {};  // "object literal" syntax
```

Usually, the figure brackets {...} are used. That declaration is called an object literal.

