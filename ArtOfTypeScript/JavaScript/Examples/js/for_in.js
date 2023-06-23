const user = {
  name: "John",
  age: 30,
  isAdmin: true,
};

for (const key in user) {
  // key value
  console.log(key, user[key]); 
  // name "John"
  // age 30 
  // isAdmin true
}
