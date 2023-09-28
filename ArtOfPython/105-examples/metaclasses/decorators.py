import logging

logging.basicConfig(
    level=logging.INFO,
    format='%(levelname)s:%(message)s:%(asctime)s',
    encoding="utf-8"
)

class Decorator:
    def __init__(self, func):
        self.calls = 0
        self.func = func 
    def __call__(self, *args, **kwargs):
        logging.info(f"Called to {self.func.__name__} {self.calls} times")
        self.calls += 1
        return self.func(*args, **kwargs)
    def __get__(self, instance, owner):
        return Wrapper(self, instance)
    
class Wrapper:
    def __init__(self, desc, subj):
        self.desc = desc
        self.subj = subj
    def __call__(self, *args, **kwargs):
        return self.desc(self.subj, *args, **kwargs)
    

@Decorator
def spam(a, b, c):                  # spam = tracer(spam0)
    print(a + b + c)                # Uses __call__ only                    

class Cspam:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

    @Decorator
    def show(self):
        print(self.a + self.b + self.c)

if __name__ == "__main__":
    spam(1, 2, 3)
    spam(1, 2, 3)
    spam(1, 2, 3)

    cs = Cspam(1, 2 ,3 )
    cs.show()
    cs.show()