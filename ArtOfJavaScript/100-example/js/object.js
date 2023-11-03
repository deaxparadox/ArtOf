let user = {};


user.name = "John";
user.age = 30;

if (user.name === undefined) {
    console.log("property doesn't exists");
} else {
    if (delete user.name) {
        console.log("delete name")
    } else {
        console.log("unable to delete name")
    }
}