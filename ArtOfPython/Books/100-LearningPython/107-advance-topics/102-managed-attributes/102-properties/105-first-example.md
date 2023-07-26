# A First Example 

Let's defines a descirptor  that intercepts access to an attribute named **named** in its clients. 

- Its methods use their **instance** argument to access state information in the subject instance, where the name string is actually stored.

```py
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

def main():
    bob = Person("Bob Smith")
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
```

```bash
fetch...
Bob Smith
change...
fetch...
Robert Smith
remove...
--------------------
fetch...
Sue Jones
name descirptor docs
```

## Inheriting descriptor attribute

Our descirptor class instance is a class attribute and thus *inherited* by all instances of the client class and any subclasses.

- If we change the **Person** class in our example to the following, the output of the script is same:

```py
class Person:
    def __init__(self, name):
        self._name = name 

    name = Name()

class Super(Person):
    pass
```

## Descriptor inside class

When a descriptor class is not usefull outside the client class we can embed the desciptor's definition inside its client syntactically:


```py

class Person:
    def __init__(self, name):
        self._name = name 

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


    name = Name()

def main():
    bob = Person("Bob Smith")
    print(bob.name)
    bob.name = "Robert Smith"
    print(bob.name)
    del bob.name 

    print("-"*20)
    sue = Person("Sue Jones")
    print(sue.name)
    print(Person.Name.__doc__)

if __name__ == "__main__":
    main()
```

- when coded this way, **Name** becomes a local varaible in the scope of the **Person** class statement.