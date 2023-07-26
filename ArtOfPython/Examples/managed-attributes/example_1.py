class Person:
    def __init__(self, name):
        self._name = name 

    def getName(self):
        print("fetch...")
        return self._name
    
    def setName(self, value):
        print("change...")
        self._name = value 
    
    def delName(self):
        print('remove...')
        del self._name 

    name = property(getName, setName, delName, "name property docs")

class Lower(Person):
    pass 

def main():
    bob = Lower("Bob Smith")
    print(bob.name)
    bob.name = "Robert Smith"
    print(bob.name)
    del bob.name 

    print("-"*20)
    sue = Lower("Sue Jones")
    print(sue.name)
    print(Person.name.__doc__)

if __name__ == "__main__":
    main()