function makeUser(name, age) {
  return {
    name: name,
    age: age,
    // ...other properties
  };
}

function shorthand(name, age) {
    return {
        name,
        age,
    }
}


function combined(name, age) {
    return {
        name,
        age: 12,
    }
}

let user = combined("John", 30);
console.log(user.name, user.age); // John
