function set(initial) {
    const s = new Set();
    const add = (val) => {
        s.add(val.toString());
    }
    const has = (val) => s.has(val.toString());
    const union = (other) => {
        let res = [];
        for (let val of s) {
            res.push(val);
        }
        for (let val of other.s) {
            res.push(val);
        }

        return set(res);
    };
    const map = (fn) => Array.from(s).map(fn);
    const size = () => s.size;

    for (let val of initial) {
        add(val);
    }

    return {
        s,
        add,
        has,
        map,
        size,
        union,
    };
}

function cell(row, col) {
    const y = row;
    const x = col;
    const equals = (p) => p.x === x && p.y === y;
    const toString = () => `${y},${x}`;

    return {
        y,
        x,
        equals,
        toString,
        withinBounds: (height, width) => {
            return 0 <= y
                && y < height
                && 0 <= x
                && x < width;
        },
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
                && at(d) === at(c)
        );
    };

    const connectedComponent = (c) => {
        let res = set([c]);
        let stack = [c];

        while (stack.length > 0) {
            const cur = stack.pop();
            res.add(cur);

            for (let neighbor of neighbors(cur)) {
                if (!res.has(neighbor)) {
                    stack.push(neighbor);
                }
            }
        }

        return res;
    };

    const connectedComponents = () => {
        let res = [];
        let visited = set([]);

        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                if (!visited.has(cell(i, j))) {
                    const cc = connectedComponent(cell(i, j));
                    res.push(cc);
                    visited = visited.union(cc);
                }
            }
        }

        return res;
    };

    const regionPrice = (reg) => {
        const perimeter = reg
            .map((c) => 4 - neighbors(c).length)
            .reduce((acc, cur) => acc + cur);
        
        const area = reg.length;
        return perimeter * area;
    };

    const gridPrice = () => connectedComponents()
        .map((cc) => regionPrice(cc.map((s) => cell(...s.split(',').map((c) => parseInt(c, 10))))))
        .reduce((acc, cur) => acc + cur);

    return {
        gridPrice,
    };
}

const fs = require('node:fs');
const data = fs.readFileSync('input12', 'utf8');

const mat = data
    .split('\n')
    .map((row) => row.split(''));

const res = grid(mat).gridPrice();
console.assert(res === 1431316);
console.log(res);
