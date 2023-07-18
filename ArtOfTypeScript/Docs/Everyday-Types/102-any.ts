let obj: any = {x: 0};

// None of the following lines of code will throw compiler errors.
// Using `any` disables all further type checking, and it is assumed 
// you know the environment better than TypeScript.

/*  
    Accessing attributes which is not present.
    It will result in TypeError: obj.foo is not a function
*/
// obj.foo();

/*  
    Accessing attributes which is not present.
    It will return undefined.
*/
obj.foo;

/*  
    print the obj
*/
console.log(obj);

/*  
    Adding attribute to `obj`
*/
obj.bar = 100;
console.log(obj.bar);


obj = "hello";
console.log(obj)

const n: number = obj;
console.log(n);