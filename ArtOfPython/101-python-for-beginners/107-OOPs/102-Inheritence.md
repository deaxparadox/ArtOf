<!-- Inheritence -->
<!-- accessing attribute -->
<!-- super() function -->
<!-- multiple inheritence -->

# Inheritence

Python support the idea of inheritence where one class inherit another class, and it get access to the attribute of inherited class. Here the Inherited class is called parent class, and inheriting class is child class.

Let's understand with example:


```py
class A:
    pass
```

Now we are going to inherit this is parent class in child *class B*:

```py
class (B):
    pass
```

Here we have created two empty class parent `A` and child `B`. Therefore child class `B` is heriting parent class `A`.


### Accessing Attribute on inheritence


Now we are going to accessing parent attribute in child. Create a parent class `A` as shown:


```py
class A:
    c_a = "I'm from A"
    def __init__(self) -> None:
        self.a = "a"
```

Now we are going to inherit above parent class `A` in chlid class `B`. 

```py
class B(A):
    def __init__(self) -> None:
        print(self.c_a)

```

As you can see we have trying to print *class variable* `c_a` of parent `A` in constructor function of child  class `B`. Now let's create instance of child `B` so our constructor  method can run:

```py
b = B()
```

```output
# output
I'm from A
```

In output we are able to access the *class variable* `c_a` of parent class `A`. 

### `super()` 

Now we will try to access the *instance variable* `a` of parent class. 

```py
# subclass
class B(A):
    def __init__(self) -> None:
        print(self.c_a)

        print(self.a)


b = B()
```

```output
I'm from A
Traceback (most recent call last):
  File "/home/creator/Documents/Paradox/Github/ArtOfPython/Examples/main.py", line 14, in <module>
    b = B()
  File "/home/creator/Documents/Paradox/Github/ArtOfPython/Examples/main.py", line 11, in __init__
    print(self.a)
AttributeError: 'B' object has no attribute 'a'
```

We are getting `AttributeError`, it means python is not able to find `a`.

To solve this issue we have to have the parent class `A` constructor method, inside the constructor method of `B`. After that we can access `a`. Let's see how we can do that:

```py
class B(A):
    def __init__(self) -> None:
        print(self.c_a)

        super().__init__()
        
        print(self.a)


b = B()
```

```output
# output 

I'm from A
a
```

For running any method from the parent class in child class we have to use `super()` function, followed by method you want to run.

Let's see another example of this:

```py
class A:
    def __init__(self):
        pass

    def method_a(self):
        print("I am method of parent A")

class B(A):
    def __init__(self):
        super().__init__()

    def method_b(self):
        super().method_a()
        print("I am method of parent B")


b = B()
b.method_b()
```

```output
# output 

I am method of parent A
I am method of parent B
```

### Multiple inheritance

Now we will look at multiple inheritence. Multiple inheritence is a technique of inheriting more than one parent class:

Let's see an example of it:

```py
class A:
    pass 

class B:
    pass 


class Child(A, B):
    pass 

c = Child()
```

In this way you can access multiple parent class in child class.