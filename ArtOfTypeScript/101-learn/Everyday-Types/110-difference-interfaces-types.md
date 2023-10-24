# Difference between Type Aliases and Intefaces

Type aliases and intefgaces are very similar, and in many cases you can choose between them freely. 

- Almost features of an `interface` are available in `type`, the key distinction is that a type cannot be re-opened to add new properties vs an interface which is extendable.

### Interfaces

- Extending an interface

```ts
interface Animal {
  name: string;
}

interface Bear extends Animal {
  honey: boolean;
}

const bear = getBear();
bear.name;
bear.honey;
```

- Adding new fields to an existing interface

```ts
interface Window {
  title: string;
}

interface Window {
  ts: TypeScriptAPI;
}

const src = 'const a = "Hello World"';
window.ts.transpileModule(src, {});
        
```

### Type Aliases

- Extending a type via intersections

```ts
type Animal = {
  name: string;
}

type Bear = Animal & { 
  honey: boolean;
}

const bear = getBear();
bear.name;
bear.honey;
```

- A type cannot be changd after being created

```ts
type Window = {
  title: string;
}

type Window = {
  ts: TypeScriptAPI;
}

 // Error: Duplicate identifier 'Window'.
     
```