def isSafe(report):
    monotone = (report == sorted(report)) or (report == sorted(report)[::-1])
    notThatFar = all(1 <= abs(report[i] - report[i+1]) <= 3 for i in range(len(report) - 1))

    return monotone and notThatFar

def isLessSafe(report):
    return any(isSafe(report[:i] + report[i+1:]) for i in range(len(report)))

with open('input2', 'r') as f:
    reports = f.read()

reports = [[int(num) for num in report.split()] for report in reports.splitlines()]

res = sum(1 if isSafe(report) else 0 for report in reports)
assert res == 236
print(res)

res = sum(1 if isLessSafe(report) else 0 for report in reports)
assert res == 308
print(res)
