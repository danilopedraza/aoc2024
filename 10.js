function set(initial) {
    const s = new Set();
    const add = (val) => {
        s.add(val.toStr());
    }
    const has = (val) => s.has(val.toStr());

    for (let val of initial) {
        add(val);
    }

    return {
        add,
        has,
    };
}

function cell(row, col) {
    const y = row;
    const x = col;

    return {
        y,
        x,
        withinBounds: (height, width) => {
            return 0 <= y
                && y < height
                && 0 <= x
                && x < width;
        },
        inList: (list) => list.some((p) => p.y === y && p.x === x),
        toStr: () => `${y},${x}`
    };
}

function grid(mat) {
    const m = mat;
    const height = m.length;
    const width = m[0].length;
    const at = (p) => m[p.y][p.x];
    const neighbors = (c) => {        
        return [
            cell(c.y - 1, c.x),
            cell(c.y + 1, c.x),
            cell(c.y, c.x - 1),
            cell(c.y, c.x + 1),
        ].filter(
            (d) => d.withinBounds(height, width)
                && at(d) - at(c) === 1
        );
    };
    const trailheads = () => {
        let res = [];
        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                if (m[i][j] === 0) {
                    res.push(cell(i, j));
                }
            }
        }

        return res;
    };

    const _score = (p, visited) => {
        let res = 0;

        for (let neighbor of neighbors(p)) {
            if (!visited.has(neighbor)) {
                visited.add(neighbor);
                res += at(neighbor) === 9 ? 1 : 0;
                res += _score(neighbor, visited);
            }
        }

        return res;
    };

    const score = (p) => _score(p, set([p]));

    const scoreSum = () => trailheads()
        .map(score)
        .reduce((acc, cur) => acc + cur);

    return {
        scoreSum,
    };
}

const fs = require('node:fs');
const data = fs.readFileSync('input10', 'utf8');

const mat = data
    .split('\n')
    .map((row) => row.split('')
    .map((d) => parseInt(d, 10)));

console.log(grid(mat).scoreSum());
