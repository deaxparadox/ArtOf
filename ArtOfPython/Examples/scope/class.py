def main1():
        
    class tester:
        def __init__(self, start):
            self.start = start 

        def nested(self, label):
            print(label, self.start)
            self.start += 1

    F = tester(0)
    F.nested("spam")
    F.nested("ham")

def main2():
    class tester:
        def __init__(self, start) -> None:
            self.start = start
        def __call__(self, label):
            print(label, self.start)
            self.start += 1

    H = tester(99)
    H("juice")                          # Invokes __call__
    H("pancakes")


def main3():
    def tester(start):
        def nested(label):
            print(label, nested.state)
            nested.state += 1
        nested.state = start 
        return nested

    F = tester(0)
    F("spam")
    F("ham")
    print(F.state)

main3()