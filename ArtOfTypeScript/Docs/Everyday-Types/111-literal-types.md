# Literal Types

In addition to the general types `string` and `number`, we can refer the *specific* strings and number in type positions.

```ts
let changingString = "Hello World";
changingString = "Olá Mundo";
// Because `changingString` can represent any possible string, that
// is how TypeScript describes it in the type system
changingString;
      
let changingString: string
 
const constantString = "Hello World";
// Because `constantString` can only represent 1 possible string, it
// has a literal type representation
constantString;
      
const constantString: "Hello World"
```

- By themselves, literal types aren't very valuable:

```ts
let x: "hello" = "hello";
// OK
x = "hello";
// ...
x = "howdy";
```

- `Type '"howdy"' is not assignable to type '"hello"'.`

You can express much usefull concept - for example, functions that only accept a certain set of known value:

```ts
function printText(s: string, alignment: "left" | "right" | "center") {
  // ...
}
printText("Hello, world", "left");
printText("G'day, mate", "centre");
```

- `Argument of type '"centre"' is not assignable to parameter of type '"left" | "right" | "center"'.`

Numeric literal types work the same way:

```ts
function compare(a: string, b: string): -1 | 0 | 1 {
  return a === b ? 0 : a > b ? 1 : -1;
}
```

You can combine these with non-literal types: 

```ts
interface Options {
  width: number;
}
function configure(x: Options | "auto") {
  // ...
}
configure({ width: 100 });
configure("auto");
configure("automatic");
```

- `Argument of type '"automatic"' is not assignable to parameter of type 'Options | "auto"'.`

- There are only two boolean literal types, and they are the types `true` and `false`. 
- The type `boolean` itself is actually just an alias for the union `true | false`.

## Literal Inference

```ts
const obj = { counter: 0 };
if (someCondition) {
  obj.counter = 1;
}
```

TypeScript doesn’t assume the assignment of 1 to a field which previously had 0 is an error. 

- Another way of saying this is that obj.counter must have the type number, not 0, because types are used to determine both reading and writing behavior.

- The same applies to strings:

```ts
declare function handleRequest(url: string, method: "GET" | "POST"): void;
 
const req = { url: "https://example.com", method: "GET" };
handleRequest(req.url, req.method);
```

- `Argument of type 'string' is not assignable to parameter of type '"GET" | "POST"'.`