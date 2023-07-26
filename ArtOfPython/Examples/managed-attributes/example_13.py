class DescBoth:
    def __init__(self, data):
        self.data = data 
    def __get__(self, instance, owner):
        return "%s %s" % (self.data, instance.data)
    def __set__(self, instance, value):
        instance.data = value 

class Client:
    def __init__(self, data):
        self.data = data 
    managed = DescBoth("spam")

I = Client("eggs")
print(I.managed)            # Show both data sources
I.managed = "SPAM"
print(I.managed)            # Change instance data