class Descriptors:
    """docstring goes here"""
    def __get__(self, instance, owner): ...         # Return attr value
    def __set__(self, instance, value): ...         # Return nothing (None)
    def __delete__(self, instance): ...             # Return nothing (None)