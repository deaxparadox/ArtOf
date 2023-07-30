from typing import Any


class Catcher:
    # def __getattr__(self, name):
    #     print("Get: %s" % name)
    def __getattribute__(self, name: str):
        print("Get: %s" % name)
    def __setattr__(self, name, value):
        print("Set: %s %s" % (name, value))

X = Catcher()
X.job
X.pay
X.pay = 99