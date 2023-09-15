class tracer(object):
    def __init__(self, func):                       # On @ decorator
        self.calls = 0                              # Save func for later call
        self.func = func 
    def __call__(self, *args, **kwargs):            # On call to original func
        self.calls += 1
        print("calls %s to %s" % (self.calls, self.func.__name__))
        return self.func(*args, **kwargs)
    def __get__(self, instance, owner):             # On method fetch
        def wrapper(*args, **kwargs):               # Retain both inst
            return self(instance, *args, **kwargs)  # Runs __call__
        return wrapper
    

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
    sue.giveRaise(.10)                          
    print(int(sue.pay))
    print(bob.lastName(), sue.lastName())       