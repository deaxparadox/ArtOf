class Stringer:
    def __init__(self, num):
        self.num = str(num) 
    def __repr__(self):
        return f"Stringable object: {self.num}"
    

# s = Stringer(12)
# print(repr(s))

print(
    "%s is Great" % "Nitish"
)