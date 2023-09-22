import logging 

logging.basicConfig(
    level=logging.INFO
)

class LoggedAgeAccess:
    def __set_name__(self, owner, name):
        self.public_name = name
        self.private_name = "_" + name

    def __get__(self, obj, objtype=None):
        value = getattr(obj, self.private_name)
        logging.info("Accessing %r giving %r", self.public_name, value)
        return value 
    def __set__(self, obj, value):
        logging.info("Updating %r to %r", self.public_name, value)
        setattr(obj, self.private_name, value)

class Person:
    age = LoggedAgeAccess()                         # First descriptor instance
    name = LoggedAgeAccess()                         # Second descriptor instance

    def __init__(self, name, age) -> None:
        self.name = name                            # Calls the first descriptor
        self.age = age                              # Calls the second descriptor
    
    def birthday(self):
        self.age += 1                               # Calls both __get__() and __set__()