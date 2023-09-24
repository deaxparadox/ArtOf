from typing import Any


class Descriptor:
    def __set_name__(self, owner, name):
        self.public_name = name
        self.private_name = "__" + name

    def __get__(self, obj, objtype=None):
        value = getattr(obj, self.private_name)
        return value

    def __set__(self, obj, value) -> None:
        setattr(obj, self.private_name, value)
        return 
    
    def __delete__(self, obj) -> None:
        raise NotImplementedError

class Desc(object):
    def __init__(self) -> None:
        self.__calls = 3

    def __get__(self, obj, objtype=None):
        if self.__calls == 0:
            raise RuntimeError("Runtime expires, create object again")
        self.__calls -= 1
        return obj._port
    
    def __set__(self, obj, value):
        obj._port = value

class Desc2(object):
    def __init__(self) -> None:
        self.__calls = 3
        
    def __get__(self, obj, objtype=None):
        if self.__calls == 0:
            raise RuntimeError("Runtime expires, create object again")
        self.__calls -= 1
        return obj._port
    
    def __set__(self, obj, value):
        obj._port = value
        if obj._port:
            # print("property already set")
            # obj._port = value
            raise AttributeError("Can't set")
        obj._port = value


class A:
    address = Descriptor()

    def __init__(self, port: str, address: str) -> None:
        self.port = port
        self.address = address

    port = Desc2()

if __name__ == "__main__":
    a = A("10000", "192.168.1.1")
    print(a.port)
    print(a.address)

    # for _ in range(4):
    #     print(a.port)