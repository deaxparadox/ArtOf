# Arrays

JavaScript provides a data type specifically for strong sequences of values. It is called an *array* and is written as a list of values between square brackets, separated by commas:

```js
let listOfNumbers = [2, 3, 5, 7, 11];
```

To access the value from *array* using index, indexing is by default start from 0.

- to access the first element:

```js
console.log(listOfNumbers[0]);
// → 2
```

- to access the third element:

```js
console.log(listOfNumbers[2]);
// → 5
```

- to calculate the index of nth element, formula is *index = n-1*, for example, if we want to calulate the index of 5th element, 5-1 is 4, therefor index is 4:

