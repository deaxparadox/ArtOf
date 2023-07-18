function printName(obj: { first: string; last?: string }) {
    console.log(obj.first)    
    if (obj.last != undefined) console.log(obj.last?.toUpperCase())
}
// Both OK
printName({ first: "Bob" });
printName({ first: "Alice", last: "Alisson" });