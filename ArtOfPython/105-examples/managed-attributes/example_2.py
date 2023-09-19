class PropSquare:
    def __init__(self, start) -> None:
        self.value = start 
    def getX(self):
        return self.value ** 2
    def setX(self, value):
        self.value = value 
    X = property(getX, setX)

def main():
    P = PropSquare(3)
    Q = PropSquare(32)

    print(P.X)
    P.X = 4
    print(P.X)
    print(Q.X)

if __name__ == "__main__":
    main()