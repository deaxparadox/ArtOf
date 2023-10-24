# Classes

## Class Memebers

An Empty one:

```ts
> class Point {}
```

This class isn't very useful yet, so let's start adding some members.

### Feilds

A field declaration creates a public writeable property on  a class:

```ts
class Point {
    x: number;
    y: number;
}

const pt = new Point();
pt.x = 0;
pt.y = 0;
```

As with other locations, the type annotations is optional, but will be an implicit `any` if not specified.

Fields can also have *initializers*; there will run automatically when the class is instantiated:

```ts
class Point {
    x: number;
    y: number;
}

const pt = new Point();
pt.x = 0;
pt.y = 0;

console.log(`Point: ${pt.x}, ${pt.y}`);
```

```
Point: 0, 0
```

Just like with `const`, `let`, and `var`, the initializer of a class property will be used to infer its type:

```ts
```