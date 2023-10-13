
def singleton(aClass):                  # On @ decoration
    instance = None
    def onCall(*args, **kwargs):        # On instance creation
        nonlocal instance              # 3.X and later nonloca
        print(instance)
        if instance == None:     
            instance = aClass(*args, **kwargs)     # One scope per class
        return instance
    return onCall


@singleton                                              # Person = singleton(Person)
class Person:                                           # Rebinds Person to onCall
    def __init__(self, name, hours, rate) -> None:      # onCall remembers Person
        self.name = name 
        self.hours = hours
        self.rate = rate
    def pay(self):
        return self.hours * self.rate
    
@singleton                          # Spam = singleton(Spam)
class Spam:                         # Rebinds Spam to onCall
    def __init__(self, val):        # onCall remembers Spam
        self.attr = val

bob = Person("Bob", 40, 10)         # Really calls onCall
print(bob.name, bob.pay())

bob = Person("Bob", 40, 10)         # Really calls onCall
print(bob.name, bob.pay())

sue = Person("Sue", 50, 20)         # Same, single object
print(sue.name, sue.pay())

X = Spam(val=42)                    # One Person, one Spam
Y = Spam(99)
print(X.attr, Y.attr)