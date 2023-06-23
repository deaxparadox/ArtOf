# Dispatching on a value with Switch

There is a contruct called a *switch* that is intended to express such a "dispatch" is a more direct way.

```js
switch (prompt("What is the weather like?")) {
  case "rainy":
    console.log("Remember to bring an umbrella.");
    break;
  case "sunny":
    console.log("Dress lightly.");
    break
  case "cloudy":
    console.log("Go outside.");
    break;
  default:
    console.log("Unknown weather type!");
    break;
}
```


```output
What is the weather like? sunny
Dress lightly.
```

You may put any number of *case* labels inside t he block opened by *switch*. The program will start executin at the label that corresponds to the value that *switch* was given, or at *default* if not mathcing value is found. It will continue executing, even accroos 