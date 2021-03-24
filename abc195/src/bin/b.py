a,b,w = map(int,input().split())

def possible(i):
    x = w * 1000 / i
    return a <= x and x <= b

ns = [i for i in range(1, 1000001) if possible(i)]
if len(ns) > 0:
    print(min(ns), max(ns))
else:
    print("UNSATISFIABLE")
"""
min(ns) = m
W = w * 1000
W / m <= b
max(ns) = M
W / M >= a  <=> W >= a * M
y = W // a 
"""
