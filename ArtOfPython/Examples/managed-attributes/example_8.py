class Name:
    "name descirptor docs"
    def __get__(self, instance, owner):
        print("fetch...")
        return instance._name
    def __set__(self, instance, value):
        print("change...")
        instance._name = value 
    def __delete__(self, instance):
        print("remove...")
        del instance._name

class Person:
    def __init__(self, name):
        self._name = name 

    name = Name()

class Super(Person):
    pass

def main():
    bob = Super("Bob Smith")
    print(bob.name)
    bob.name = "Robert Smith"
    print(bob.name)
    del bob.name 

    print("-"*20)
    sue = Person("Sue Jones")
    print(sue.name)
    print(Name.__doc__)

if __name__ == "__main__":
    main()