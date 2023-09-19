class Person:
    def __init__(self, name):
        self._name = name 

    @property
    def name(self):             # name = property(name)
        "name property docs"        
        print("fetch...")
        return self._name 
    
    @name.setter
    def name(self, value):      # name = name.setter(name)
        print("change...")
        self._name = value 
    
    @name.deleter               # name = name.deleter(name)
    def name(self):
        print("remove...")
        del self._name 

def main():
    bob = Person("Bob Smith")   # bob has a managed attribute
    print(bob.name)             # Runs name getter (name 1)
    bob.name = "Robert Smith"   # Runs name setter (name 2)
    print(bob.name)
    del bob.name                # Runs name deleter (name 3)

    print("-"*20)
    sue = Person("Sue Jones")
    print(sue.name)             # sue inherits property too
    print(Person.name.__doc__)  # Or help (Person.name)

if __name__ == "__main__":
    main()