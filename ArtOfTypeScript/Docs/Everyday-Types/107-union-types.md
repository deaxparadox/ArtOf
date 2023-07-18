# Union Types

TypeScript's type system allows you to build new  types out of existing ones using a large variety of operators.

## Defining a Union Type

The first way to combine types you might see is a *union* type.

- A union type is a type formed from two or more other types representing values that may be any one of those types.
- We refere to each of these types as  the union's members.

Let's write a function that can operate on strings or numbers:

```ts
function printId(id: number | string) {
  console.log("Your ID is: " + id);
}
// OK
printId(101);
// OK
printId("202");
// Error
printId({ myID: 22342 });

Argument of type '{ myID: number; }' is not assignable to parameter of type 'string | number'.

```

## Working with Union Types

TypeScript will only allow an operation if it is valid for every member of the union.

- For example, if you have the union `string | number`, you can't use methods that are only available on `string`:

```ts
function printId(id: number | string) {
  console.log(id.toUpperCase());
}
```

`Property 'toUpperCase' does not exist on type 'string | number'. Property 'toUpperCase' does not exist on type 'number'.`

The solution is to `narrow` the union with code, the same as you would is JavaScript without type annotations. *Narrowing` occurs when TypeScript can deduce a more specific type for a value based on the structure of the code.

- For example, TypeScript knows that only a `string` value will have a `typeof` value `"string"`:

```ts
function printId(id: number | string) {
  if (typeof id === "string") {
    // In this branch, id is of type 'string'
    console.log(id.toUpperCase());
  } else {
    // Here, id is of type 'number'
    console.log(id);
  }
}
```
