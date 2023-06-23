let user = new Object();

console.log("noSuchProperty" in user);

user.name = "EveryWhereLinux";
console.log("name" in user);