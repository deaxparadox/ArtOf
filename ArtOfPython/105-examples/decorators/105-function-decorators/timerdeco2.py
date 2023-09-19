import time, sys 
force = list if sys.version_info[0] == 3 else (lambda X: X)

def timer(label="", trace=True):                            # On decorator args: retain args
    class Timer:
        def __init__(self, func):                           # On @: retain decorated func
            self.func = func 
            self.alltime = 0
        def __call__(self, *args, **kwargs):                # On calls: call original
            start = time.time()
            result = self.func(*args, **kwargs)
            elapsed = time.time() - start 
            self.alltime += elapsed
            if trace:
                format = "%s %s: %.5f, %.5f"
                values = (label, self.func.__name__, elapsed, self.alltime)
                print(format % values)
            return result
    return Timer
    
@timer(label="[CCC]==>")
def listcomp(N):
    return [x * x for x in range(N)]

@timer(trace=True, label="[MMM]==>")
def mapcall(N):
    return force(map((lambda x: x * 2), range(N)))

result = listcomp(5)            # Time for this call, alls, return value
listcomp(50000)
listcomp(500000)
listcomp(1000000)
print(result)
print("allTime = %s" % listcomp.alltime)        # Total time for all listcomp calls                


print("")
result = mapcall(5)
mapcall(50000)
mapcall(500000)
mapcall(1000000)
print(result)
print('allTime = %s' % mapcall.alltime)         # Total time for all mapcall calls

print("\n**map/comp = %s" % round(mapcall.alltime / listcomp.alltime, 3))