const fs = require('node:fs');
const data = fs.readFileSync('input5', 'utf8');

function newUpdate(update) {
    return {
        mainValue: () => update[Math.floor(update.length / 2)],
        valid: (rules) => rules.every(
            (rule) => rule.satisfiedByUpdate(update)
        ),
        pendingRules: (rules) => rules.filter(
            (rule) => rule.involved(update)
        ),
        failedRules: (rules) => rules.filter((rule) => !rule.satisfiedByUpdate(update)),
        fix: function(rules) {
            return newUpdate(graph(update, this.pendingRules(rules)).toposort());
        }
    };
}

function newRule(left, right) {
    return {
        involved: (update) => update.includes(left) && update.includes(right),
        includes: (val) => left === val || right === val,
        isSon: (val) => left === val,
        every: (prop) => prop(left) && prop(right),
        satisfiedByUpdate: function(update) {
            return !this.involved(update)
            || (
            this.involved(update)
         && update.indexOf(left) < update.indexOf(right));
        },
    };
}

const [rules, updates] = (() => {
    const [rules, updates] = data.split('\n\n');

    return [
        rules.split('\n').map((rule) => {
            const [left, right] = rule
                .split('|')
                .map((str) => parseInt(str, 10));
            
            return newRule(left, right);
        }),
        updates
            .split('\n')
            .filter((update) => update.length > 0)
            .map((update) => update
                .split(',')
                .map((val) => parseInt(val, 10)
            ))
            .map(newUpdate),
    ];
})();

function graph(vertices, pairs) {
    return {
        vertices,
        pairs,
        roots: () => vertices.filter(
            (vertex) => {
                return pairs.filter((pair) => pair.isSon(vertex)).length === 0;
            }
        ),
        withoutRoots: function() {
            return graph(
                this.vertices.filter((vertex) => !this.roots().includes(vertex)),
                this.pairs.filter(
                    (pair) => pair.every(
                        (vertex) => !this.roots().includes(vertex)
                    )
                )
            );
        },
        toposort: function() {
            if (this.vertices.length === 0) {
                return [];
            }
            return this.withoutRoots().toposort().concat(this.roots());
        }
    }
}


let res = updates
    .map(
        (update) => update
            .valid(rules) ? update.mainValue() : 0
    )
    .reduce((acc, cur) => acc + cur, 0);

console.assert(res === 5374);
console.log(res);

res = updates
    .filter((update) => update.failedRules(rules).length > 0)
    .map((update) => update.fix(rules).mainValue())
    .reduce((acc, cur) => acc + cur, 0);
console.assert(res === 4260);
console.log(res);
