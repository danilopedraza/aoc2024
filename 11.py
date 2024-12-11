with open('input11', 'r') as f:
    data = f.read()

nums = data.split()

def stonesFrom(stone):
    if stone == '0':
        return ['1']
    elif len(stone) % 2 == 0:
        return [str(int(stone[:len(stone)//2])), str(int(stone[len(stone)//2:]))]
    else:
        return [str(int(stone)*2024)]

def length(stones, iterations):
    if iterations == 0:
        return len(stones)
    
    return sum(length(stonesFrom(stone), iterations - 1) for stone in stones)

res = length(nums, 25)
assert res == 228668
print(res)
