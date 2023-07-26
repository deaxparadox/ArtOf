class InstState:
    def __get__(self, instance, owner):
        print("Instance get")
        return instance._X * 10
    def __set__(self, instance, value):
        instance._X = value


class CalcAttrs:
    X = InstState()                    # Desciptor class attr
    Y = 3                               # Class attr
    def __init__(self):
        self._X = 2
        self.Z = 4                      # Instance attr


obj = CalcAttrs()
print(obj.X, obj.Y, obj.Z)              # X is compyuted, others are not
obj.X = 5                               # X assignment is intercepted
CalcAttrs.Y = 6                         # Y reassigned in class
obj.Z = 7                               # Z assigned in instance
print(obj.X, obj.Y, obj.Z)

obj2 = CalcAttrs()
print(obj2.X, obj2.Y, obj2.Z)