const fs = require('node:fs');
const data = fs.readFileSync('input4', 'utf8');

const mat = data
    .split('\n')
    .map((row) => row.split(''))
    .filter((row) => row.length > 0);

function transpose(mat) {
    return mat[0].map((_, i) => mat.map(row => row[i]));
}

function upwardDiagonals(mat) {
    const height = mat.length;
    const width = mat[0].length;
    let res = [];

    for (let i = 0; i < height; i++) {
        res.push(upwardDiagonal(mat, i, 0));
    }

    for (let j = 1; j < width; j++) {
        res.push(upwardDiagonal(mat, height - 1, j))
    }

    return res;
}

function upwardDiagonal(mat, i, j) {
    const height = mat.length;
    const width = mat[0].length;
    const withinBoundaries = (i, j) => 0 <= i && i < height && 0 <= j && j < width;

    let res = [];
    while (withinBoundaries(i, j)) {
        res.push(mat[i][j]);
        i--;
        j++;
    }

    return res;
}

function downwardDiagonals(mat) {
    const height = mat.length;
    const width = mat[0].length;
    let res = [];

    for (let i = 0; i < height; i++) {
        res.push(downwardDiagonal(mat, i, 0));
    }

    for (let j = 1; j < width; j++) {
        res.push(downwardDiagonal(mat, 0, j))
    }

    return res;
}

function downwardDiagonal(mat, i, j) {
    const height = mat.length;
    const width = mat[0].length;
    const withinBoundaries = (i, j) => 0 <= i && i < height && 0 <= j && j < width;

    let res = [];
    while (withinBoundaries(i, j)) {
        res.push(mat[i][j]);
        i++;
        j++;
    }

    return res;
}

function reverse(mat) {
    return mat.map((row) => row.reverse());
}

function allMatches(mat) {
    return matches(mat)
         + matches(reverse(mat))
         + matches(transpose(mat))
         + matches(reverse(transpose(mat)))
         + matches(upwardDiagonals(mat))
         + matches(reverse(upwardDiagonals(mat)))
         + matches(downwardDiagonals(mat))
         + matches(reverse(downwardDiagonals(mat)));
}

function matches(mat) {
    return mat
        .map((row) => {
            const res = row.join('').match(/XMAS/g);
            return res ? res.length : 0;
        })
        .reduce((acc, cur) => acc + cur, 0);
}

let res = allMatches(mat);
console.assert(res === 2458);
console.log(res);

function actualMatches(mat) {
    const height = mat.length;
    const width = mat[0].length;

    let res = 0;
    const regex = /(MAS)|(SAM)/;
    for (let i = 0; i + 2 < height; i++) {
        for (let j = 0; j + 2 < width; j++) {
            const diag1 = downwardDiagonal(mat, i, j)
                .join('')
                .slice(0, 3);
            const diag2 = upwardDiagonal(mat, i + 2, j)
                .join('')
                .slice(0, 3);
            
            if (regex.test(diag1) && regex.test(diag2)) {
                res++;
            }
        }
    }

    return res;
}

res = actualMatches(mat);
console.assert(res === 1945);
console.log(res);
