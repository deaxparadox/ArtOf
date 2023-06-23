# Bindings and scopes

Each bindings has a *scope*, which is the part of the program in which the bindings is visible.

Bindings declared with *let* and *const* are in fact local to the *block* that they are declared in, so if you create one those inside of a loop, the code before and after the loop cannot "see".

In pre-2015 JavaScript, only functions ceated new scopes, so old style bindings, created with the *var* keyword, are visible throughout the whole function that they appear in--or throughout the global scope, if they are not in a function.

```js
let x = 10;
if (true) {
  let y = 20;
  var z = 30;
  console.log(x + y + z);
  // → 60
}
// y is not visible here
console.log(x + z);
// → 40
```

## global

For bindings defiend outside of any function or block, the scope of the whole program -- you can refer to such bindings whenever you want. These are called *global*.



## local

Bindings created for function parameters or declared inside a function can be reference only in that function, so they are known as *local* bindings. Every time the function is called, new instances of these bindings are created.



```js

// visible in glocal scope
// it can be access anywhere in program
const x = 10;

let f = function() {
  // y is local to function
  let y = 20;

  // x can be access in function
  return x + y;
}

// y is not visible in this scope,
// because it is local to function
console.log(`f() => ${f()}`)
console.log(`x => ${x}`)


try {
  // it will raise error because y is local to function f
  console.log(`y => ${y}`)
} catch(e) {
  // we are catching error 
  // and display it.
  console.error(e);
}
```