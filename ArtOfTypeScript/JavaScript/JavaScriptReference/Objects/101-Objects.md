# Objects

Objects are used to store keyed collections of various data and more complex entitires. Objects penetrate almost every aspect of the language.

## Syntax

- Object are created with brackets:  `{..}` with an optional list of properties.
- A property is a `key:value` pair, where `key` sis a string (also called a "property name"), and `value` can be anything.

An empty object ("empty cabinet") can be created using one of two syntaxes:

- object constructor syntax

```js
let account = new Object();
```

- object literal syntax, it uses brackets `{...}`

```js
let account = {};
```

## Literals and properties

Property has a `key-value` separated by colon `:`, i.e. `key: value`, and each property is separater by comma `,`, i.e. {key1: value, key2: value}:

Let add some property to our account object:

```js
> account.name = "EveryWhereLinux";
"EveryWhereLinux"
>
> account.os = "linux"
"linux" 
>
> account
{ name: "EveryWhereLinux", os: "linux" }
```

- The first property has key `name` and the value `EveryWhereLinux`.
- The second property has the key `os` and the value `linux`.

