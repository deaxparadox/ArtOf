def gen():
    for i in range(10):
        X = yield i
        print(X, end=", ")

G = gen()
print(next(G))

for i in range(20, 40):
    print(G.send(i))