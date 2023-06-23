from ds.english.alphabet import Alphabet, AlphaWeight  
from ds.node import TextTree

def tree():
    tt = TextTree("Paradox".lower())
    print(tt.value)
    print(tt.branches)
    for i in range(10):
        tt.branches.append(TextTree(str(i)))
    # print(tt.branches)
    for i in tt.branches:
        print(i, i.value, i.is_full(), i.length())
    print(tt.is_full())


def alphabet():
    alphabet = Alphabet()
    aw = getattr(alphabet, 'a')
    print(aw, type(aw))
    print(aw.alpha, aw.weight)

def alphaweight():
    aw = AlphaWeight("a", 1)
    print(aw.alpha)

if __name__ == "__main__":
    
    # tree()
    alphabet()
    # alphaweight()