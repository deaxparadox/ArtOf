mod = __import__("pkg.mod", globals(), locals(), ["sqaure"], 0)

print(mod.square(3))