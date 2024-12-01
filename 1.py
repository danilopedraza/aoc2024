with open('input1', 'r') as f:
    pairs = f.read()

pairs = [pair.split('   ') for pair in pairs.splitlines()]
pairs = [(int(pair[0]), int(pair[1])) for pair in pairs]
left = [x for x, _ in pairs]
right = [y for _, y in pairs]
left.sort()
right.sort()

res = sum(abs(x - y) for x, y in zip(left, right))
assert res == 1110981
print(res)

leftRightFreq = {val:0 for val in left}
for val in right:
    if val in leftRightFreq:
        leftRightFreq[val] +=1

res = sum(val*leftRightFreq[val] for val in left)
assert res == 24869388
print(res)
