def timer(label=""):
    def decorator(func):
        def onCall(*args):          # Multilevel state retention
            ...                     # args passed to function
            func(*args)             # func retained in enclosing scope
            print(label, ...)       # label retained in enclosing scope
        return onCall
    return decorator                # Returns the actual decorator

@timer("==>")                   # Like listcomp = timer("==>")(listcomp)
def listcomp(N): ...            # listcomp is rebound to new onCall

listcomp(...)                   # Really calls onCall