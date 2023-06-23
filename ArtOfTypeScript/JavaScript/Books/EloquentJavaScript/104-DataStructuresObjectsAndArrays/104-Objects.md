# Ojbects

Values of the type *object* are arbitrary collections of properties. One way to create an object is by using braces as an expression.

```js
let day1 = {
  squirrel: false,
  events: ["work", "touched tree", "pizza", "running"]
};
console.log(day1.squirrel);
// → false
console.log(day1.wolf);
// → undefined
day1.wolf = false;
console.log(day1.wolf);
// → false
```

- Inside the braces, there is a list of properties separated by commas. 
- Each property has a name followed by a colon and a value.

- Properties whose name aren't valid binding names or valid numbers have to be quoted:

```js
let descriptions = {
  work: "Went to work",
  "touched tree": "Touched a tree"
};
```

Another way to fetch properties is:

```js
> descriptions["touched tree"]
"Touched a tree"
```

- Property that doesn't exist will give you the value *undefined*.

# Assign a new value to properties

To assign new value to properties:

- Create a object:


```js
> let descriptions = {
  work: "Went to work",
  "touched tree": "Touched a tree"
};

> descriptions.work
"Went to work"
> descriptions['touched tree']
"Touched a tree"
> 
```

- way to change assign a new value:

```js
> descriptions.work = "This is the new work";
"This is the new work"
>
> descriptions.work
"This is the new work"
> 
```

- another way to assign a new value:

```js
> descriptions['touched tree'] = "This is a newly planted tree."
"This is a newly planted tree."
>
> descriptions['touched tree']
"This is a newly planted tree."
> 
```

## Deleting value from property

Using previous created object:

```js
> delete descriptions["touched tree"]
true
> descriptions
{ work: "This is the new work" }
> 
```

- delete statement return *true*, which means it deleted the property. It we try to access delete value, we will get undefined:

```js
> descriptions['touched tree']
undefined
> 
```

- Since *touch tree* contain name that is why we are using this method to *assign* and *delete* properties. Otherwise you can *dot* method for everything.

## Operation on Object

- To check weather an object contains the a properties, we can use *in* operator:

```js
// deleted this property earlier
> "touched tree" in descriptions
false

> "work" in descriptions
true
> 
```

- To find out what properties an object has, you can use the *Object.keys* function:

```js
> Object.keys(descriptions)
[ "work" ]
```

We use *Object.keys* function, and an object as parameter to get all the keys.

- To copy all properties from one object into another.

