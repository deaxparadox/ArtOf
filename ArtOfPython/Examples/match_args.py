
class Click:
    __match_args__ = ("pos", 'btn')
    def __init__(self, pos, btn):
        self.position = pos
        self.button = btn

class ClickAgain:
    __match_args__ = ('btn', "pos")
    def __init__(self, pos, btn):
        self.position = pos
        self.button = btn

        

def example_1():
    c = Click(180, 100)
    print(c.position, c.button)

    a = ClickAgain(180, 100)
    print(a.position, a.button)

def main():
    example_1()

if __name__ == "__main__":
    main()