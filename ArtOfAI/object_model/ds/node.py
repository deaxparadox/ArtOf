import typing

from .logging import logging



BRANCH_LIMIT = 10


class Descriptor:
    def __set_name__(self, obj, name):
        self.public_name = name 
        self.private_name = "_" + name 

    def __get__(self, obj, objtype=None):
        logging.debug("Getting attribute... %s" % (self.public_name))
        return getattr(obj, self.private_name)
    
    def __set__(self, obj, value):
        logging.debug("Setting attribute... %s" % (self.public_name))
        setattr(obj, self.private_name, value)
    
    def __delete__(self, obj):
        raise RuntimeError("Not implemented")
    


class ListDescriptor:
    def __set_name__(self, obj, name):
        self.public_name = name 
        self.private_name = "_" + name 
    

    def __get__(self, obj, objtype=None):
        logging.debug("Getting attribute... %s" % (self.public_name))
        return getattr(obj, self.private_name)
    
    def __set__(self, obj, value):
        # raise RuntimeError("Not implemented")
        logging.debug("Setting attribute... %s" % (self.public_name))
        setattr(obj, self.private_name, value)
    
    def __delete__(self, obj):
        raise RuntimeError("Not implemented")




class TList(list):
    def length(self):
        return len(self)


Self = typing.TypeVar("Self", bound="TextTree")

class TextTree(object):
    """
    `TextTree` node,
    
    :param:

    value = store the node value
    """
    value = Descriptor()
    branches = ListDescriptor()

    def __init__(
        self, 
        value: typing.Any, 
        limit: int|None = BRANCH_LIMIT
    ) -> None:
        self.value = value 
        self.limit = limit
        self.branches: TList[TextTree] = TList()


    def is_full(self) -> bool:
        return  self.branches.length() >= BRANCH_LIMIT
    
    def length(self) -> int:
        return len(self.value)

    def weight(self) -> int:
        raise NotImplemented
