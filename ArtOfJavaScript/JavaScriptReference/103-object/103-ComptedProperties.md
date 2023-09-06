## Computed properties

We can use square brackets in an object literal, when creating an object. That's called *computed properties*.

```js
let fruit = prompt("Which fruit to buy?", "apple/orange");
let bag = {
    // The name of the property is taken from the variable fruit.
    [fruit]: 5
}

console.log(bag.apple ? bag.apple : bag.orange);
```

```bash
$ deno run computed_properties.js 
Which fruit to buy? [apple/orange] apple
5
$ deno run computed_properties.js 
Which fruit to buy? [apple/orange] orange 
5
$ 
```

We can use more complex expressions inside square brackets:

```js
let fruit = "apple";
let bag = {
    [fruit + 'Computers'] : 5
};
console.log(bag.appleComputers);

// 5
```