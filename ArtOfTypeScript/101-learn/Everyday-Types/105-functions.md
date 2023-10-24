# Functions

Functions are the primary means of passing data around in JavaScript. 

- TypeScript allows you to specify the types of both the input and output values of functions.

## Parameter Type Annotations

When you declare a function, you can add type annotations after each parameter to declare what types of parameters the function accepts.

- Parameter type annotations go after the parameter name:

```ts
// Parameter type annotation
function greet(name: string) {
  console.log("Hello, " + name.toUpperCase() + "!!");
}
```

- When a parameter has a type annotation, arguments to that function will be checked:

```ts
// Would be a runtime error if executed!
greet(42);
// Argument of type 'number' is not assignable to parameter of type 'string'.
```

`Even if you don’t have type annotations on your parameters, TypeScript will still check that you passed the right number of arguments.`

## Return Type Annotations

You can also add return type annotations.

- Return type annotations appear after the parameter list:

```ts
function getFavoriteNumber(): number {
  return 26;
}
```

- Myst like variable type annotations, you usually don’t need a return type annotation because TypeScript will infer the function’s return type based on its `return` statements

## Annonymous Functions

Anonymous functions are a little bit different from function declarations.

- When a function appears in a place where TypeScript can determine how it’s going to be called, the parameters of that function are automatically given types.

```ts
const names = ["Alice", "Bob", "Eve"];
 
// Contextual typing for function - parameter s inferred to have type string
names.forEach(function (s) {
  console.log(s.toUpperCase());
});
 
// Contextual typing also applies to arrow functions
names.forEach((s) => {
  console.log(s.toUpperCase());
});
```

- Even though the parameter `s` didn't have a type annotation, TypeScript used  the types of the `forEach` function, along with inferred type of the array, to determine the type `s` will have.
- This process is called *contextual typing* because the *content* that the function occured within informs what type it should have.

