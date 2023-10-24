# `declare` Keyword

`declare` is used to tell the compiler "this thing (usually a variable) exists already, and therefore can be reference by other code, also there is no need to compile this statement into any JavaScript".

## Common Use Case:

You add a reference to your web page to a JavaScript file that the compiler knows nothing about.

- maybe it is a script coming from some other domain like 'foo.com'. 
- when evaluated the script will create an object with some useful API methods and assign it to the identifier 'fooSdk' on the global scope.

You want your TypeScript code to be able to call `fooSdk.doSomething()`, but since your compiler does not know `fooSdk` variable exists, you will get a compilation error.

You then use the `declare` keyword as a way of telling the compiler "trust me, this variable exists and has this type". The compiler will use this statement to statically check other code but will not trans-compile it into any JavaScript in the output.

```ts
declare const fooSdk = { doSomething: () => boolean }
```

Newer TypeScript version require a slightly different syntax:

```ts
declare const fooSdk : { doSomething: () => boolean }
```