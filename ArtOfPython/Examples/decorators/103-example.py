def decorator(C):                       # On @ decoration
    class Wrapper:
        def __init__(self, *args):      # On instance creation: new Wrapper
            self.wrapped = C(*args)     # Embed instance in instance
    return Wrapper

class Wrapper: ...
def decorator(C):                       # On @ decoration
    def onCall(*args):                  # On instance creation: new Wrapper
        return Wrapper(C(*args))        # Embed instance in instance
    return onCall