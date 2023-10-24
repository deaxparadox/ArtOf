// // Parameter type annotation
// function greet(name: string) {
//     console.log("Hello, " + name.toUpperCase() + "!!");
// }

// console.log("Paradox");


const names = ["Alice", "Bob", "Eve"];
 
// Contextual typing for function - parameter s inferred to have type string
names.forEach(function (s) {
  console.log(s.toUpperCase());
});
 
// Contextual typing also applies to arrow functions
names.forEach((s) => {
  console.log(s.toUpperCase());
});

