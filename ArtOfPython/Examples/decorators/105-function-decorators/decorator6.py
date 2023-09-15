class tracer:                                   # On @ decorator
    def __init__(self, func):                   # Save func for later call
        self.calls = 0
        self.func = func 
    def __call__(self, *args, **kwargs):        # On call to original function
        self.calls += 1
        print("calls %s to %s" % (self.calls, self.func.__name__))
        return self.func(*args, **kwargs)
    
@tracer
def spam(a, b, c):                  # spam = tracer(spam)
    print(a + b + c)                # Triggers tracer.__init__



# in terminal
# spam(1, 2, 3)                       # Runs tracer.__call__
# spam(a=4, b=5, c=6)


class Person:
    def __init__(self, name, pay):
        self.name = name 
        self.pay = pay 
    
    @tracer
    def giveRaise(self, percent):           # giveRaise = tracer(giveRaise)
        self.pay *= (1.0 + percent)

    @tracer 
    def lastName(self):                     # lastName = tracer(lastName)
        return self.name.split()[-1]
    

# bob = Person("Bob Smith", 50000)    # tracer remembers method funcs
# bob.giveRaise(.25)                  # Run tracer.__call(???, .25)
# print(bob.lastName())