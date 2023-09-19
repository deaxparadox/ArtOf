class tracer(object):                           # A decorator + descriptor
    def __init__(self, func):                   # On @ decorator
        self.calls = 0                          # Save func for later call
        self.func = func 
    def __call__(self, *args, **kwargs):        # On call to original func
        self.calls += 1
        print("call %s to %s" % (self.calls, self.func.__name__))
        return self.func(*args, **kwargs)
    def __get__(self, instance, owner):         # On method attribute fetch
        return wrapper(self, instance)
    
class wrapper:
    def __init__(self, desc, subj) -> None:     # Save both instances
        self.desc = desc                        # Route calls back to deco/desc
        self.subj = subj
    def __call__(self, *args, **kwargs):
        return self.desc(self.subj, *args, **kwargs)        # Runs tracer.__call__
    

@tracer
def spam(a, b, c):                  # spam = tracer(spam0)
    print(a + b + c)                # Uses __call__ only                    


class Person:
    def __init__(self, name, pay):
        self.name = name 
        self.pay = pay 
    
    @tracer
    def giveRaise(self, percent):           # giveRaise = tracer(giveRaise)
        self.pay *= (1.0 + percent)         # Makems giveRaise a descriptor

    @tracer 
    def lastName(self):                     # lastName = tracer(lastName)
        return self.name.split()[-1]
    

if __name__ == "__main__":
    
    spam(1, 2, 3)
    spam(a=4, b=5, c=6)

    print("\nmethods...")
    bob = Person("Bob Smith", 50000)
    sue = Person("Sue Jones", 100000)
    print(bob.name, sue.name)
    sue.giveRaise(.10)                          # Runs __get__ then __call__
    print(int(sue.pay))
    print(bob.lastName(), sue.lastName())       