
matrix = [[1,2,3],
         [4,5,6],
         [7,8,9]]

def isSquareMatrix(x):
    l = len(x[0])
    c = 1
    for i in x:
        if c > l: return False
        elif len(i) == l: c += 1; continue
        else: return False
    return True


print(isSquareMatrix(matrix))