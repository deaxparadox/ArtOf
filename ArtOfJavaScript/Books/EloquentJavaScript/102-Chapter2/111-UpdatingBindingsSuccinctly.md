# Updating Bindings Succinctly

- Updating values using full syntax, *variable = variable + 1*:

```js
> let counter = 0;
undefined
> counter = counter + 1;
1
> counter = counter + 1;
2
> 
```

- Updating values using short syntax, *variable += 1*:

```js
> let counter = 0;
undefined
> counter += 1;
1
> counter += 1;
2
> 
```

Let's dive in example of short syntax in for loop:

```js
for (let number = 0; number <= 12; number += 2) {
  console.log(number);
}
```

```output
0
2
4
6
8
10
12
```

## Post-increment

Syntax for post increment is *varaible++*, variable value is increased after the use of retaining the value from the variable;

```js
> let counter = 0;
> counter;
0
> counter++;
0
> console.log(`Acutal value: ${counter}`);
Acutal value: 1
> 
```

```js
> counter;
1
> counter++;
1
> console.log(`Acutal value: ${counter}`);
Acutal value: 2
> 
```

As you can whenever we retain a value, immidiately after that value get increased.


## post-decrement

`post-decrement` work the same sa *post-increment*, only behavioural difference is `post-decrement` decreases a value:

```js
> let counter = 10;
> counter--;
10
> counter--;
9
> console.log(`Acutal value: ${counter}`);
Acutal value: 8
> counter
8
> 
```

