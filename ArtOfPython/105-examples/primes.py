import typing

class primes(object):
    __match_args__ = ("start", "stop")
    def __init__(self, start: int = 2, stop: int = 2) -> None:
        self.__start = start
        self.__stop = stop
        if self.__stop == 2:
            self.__stop = self.__start
            self.__start = 2
        
        self.__prime: int = 2

    def __calculate(self):
        p: bool = False
        # print("start:", self.__start)
        for n in range(self.__start, self.__stop):
            for i in range(2, n):
                if n % i == 0:
                    self.__start += 1
                    break
            else:
                self.__prime = self.__start
                self.__start += 1
                return
        return    

        
  
    def __iter__(self):
        return self
    
    def __next__(self, *args, **kwargs):
        self.__calculate()
        if self.__start == self.__stop:
            raise StopIteration
        return self.__prime

    def __enter__(self):
        return self 
    def __exit__(self, *args, **kwargs):
        return 


if __name__ == "__main__":
    for n in range(2, 100):
        for i in primes(n):
            print(i, end=", ")
        print()
    print(list(primes(20)))

    p = primes(5)
    for i in p: print(i)

    with primes(5) as p:
        print("Inside with")
        for j in p:
            print(j)