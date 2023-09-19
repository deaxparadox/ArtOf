class D:
    def __get__(*args): print("get")

class C:
    a = D()             # Attribute a is descirptor instance

X = C()
X.a                     # Runs inherited descriptor __get__ 

C.a

X.a = 99                # Stored on X, hiding C.a!
print(X.a)

print(list(X.__dict__.keys()))

Y = C()
Y.a 
C.a