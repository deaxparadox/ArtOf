class Descriptor:
    def __get__(self, instance, owner):
        print(self, instance, owner, sep="\n")

class Subject:
    attr = Descriptor()
    

def main():
    X = Subject()
    X.attr

    print("-"*20)

    Subject.attr

if __name__ == "__main__":
    main()