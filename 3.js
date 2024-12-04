const fs = require('node:fs');
const data = fs.readFileSync('input3', 'utf8');

let matches = data.match(/mul\([0-9]+,[0-9]+\)/g);

let res = matches.reduce(
    (acc, cur) => acc + cur.match(/[0-9]+/g).reduce((acc, cur) => acc * cur, 1),
    0
);

console.assert(res === 155955228);
console.log(res);

matches = data.match(/(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))/g);

res = 0;
shouldAdd = true;

for (let match of matches) {
    if (match === 'do()') {
        shouldAdd = true;
    } else if (match === "don't()") {
        shouldAdd = false;
    } else {
        if (shouldAdd) {
            res += match.match(/[0-9]+/g).reduce((acc, cur) => acc * cur, 1);
        }
    }
}

console.assert(res === 100189366);
console.log(res);
