from typing import Any


traceMe = False
def trace(*args):
    if traceMe: print("[" + "".join(map(str, args)) + "]")

def accessControl(failIf):
    def onDecorator(aClass):
        class onInstance:
            def __init__(self, *args, **kwargs):
                self.__wrapped = aClass(*args, **kwargs)
            def __getattr__(self, attr):
                trace("get: ", attr)
                if failIf(attr):
                    raise TypeError("private attribute fetched: " + attr)
                else:
                    return getattr(self.__wrapped, attr)
            def __setattr__(self, attr, value) -> None:
                trace('set:', attr, value)
                if attr == "_onInstance__wrapped":
                    self.__dict__[attr] = value
                elif failIf(attr):
                    raise TypeError("private attribute change: " + attr)
                else:
                    setattr(self.__wrapped, attr, value)
        return onInstance
    return onDecorator

def Private(*attributes):
    return accessControl(failIf=(lambda attr: attr in attributes))

def Public(*attributes):
    return accessControl(failIf=(lambda attr: attr not in attributes))


@Private('get')
class Person:
    def __init__(self, name, age):
        self.name = name 
        self.age = age

