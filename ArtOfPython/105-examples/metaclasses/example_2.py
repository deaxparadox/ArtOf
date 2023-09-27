class Eggs: ...

class Meta: ...

class Spam(Eggs, metaclass=Meta):
    data = 1
    def meth(self, arg):
        return self.data + arg