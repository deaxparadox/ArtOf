type AlphaNum = string | number;

function compare(a: AlphaNum, b: AlphaNum): -1 | 0 | 1 {
    return a === b ? 0 : a > b ? 1 : -1;
}

console.log(compare('1', '2'));
console.log(compare(2, 2));
console.log(compare(3, 2));