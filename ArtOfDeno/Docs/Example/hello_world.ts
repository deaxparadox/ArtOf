function capatilize(word: string) {
    return word.charAt(0).toUpperCase() + word.slice(1);
}

function hello(name: string) {
    return "Hello " + capatilize(name);
}


console.log(hello("john"));
console.log(hello("sarah"));
console.log(hello("kei"))