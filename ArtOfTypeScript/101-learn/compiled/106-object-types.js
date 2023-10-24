"use strict";
function printName(obj) {
    var _a;
    console.log(obj.first);
    if (obj.last != undefined)
        console.log((_a = obj.last) === null || _a === void 0 ? void 0 : _a.toUpperCase());
}
// Both OK
printName({ first: "Bob" });
printName({ first: "Alice", last: "Alisson" });
