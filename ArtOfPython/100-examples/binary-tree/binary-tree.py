from tkinter import NO
from typing import Any


class Node:
    def __init__(self, data: int) -> None:
        self.data = data
        self.left = None
        self.right = None
        
class BinaryTree:
    def __init__(self) -> None:
        self.__root = None
    
    def __call__(self, *args: Any, **kwds: Any) -> Node:
        return self.__root    
    
    @property
    def root(self) -> Node:
        return self.__root
    
    def insert(self, data: int):
        if not self.__root:
            self.__root = Node(data)
        else:
            self.__insert(self.__root, data)
    def __insert(self, root: Node, data: int):
        if data < root.data:
            if not root.left:
                root.left = Node(data)
            else:
                self.__insert(root.left, data)
        else:
            if not root.right:
                root.right = Node(data)
            else:
                self.__insert(root.right, data)
                
def pre_order(root: Node | None) -> None:
    if root:
        print(root.data)
        pre_order(root.left)
        pre_order(root.right)
    return

def in_order(root: Node | None) -> None:
    if root:
        in_order(root.left)
        print(root.data)
        in_order(root.right)
    return
        
def post_order(root: Node | None) -> None:
    if root:
        post_order(root.left)
        post_order(root.right)
        print(root.data)
    return

def height_of_tree(root: Node, target: int) -> int:
    h = 1
    if target == root.data:
        return 
        
def main():
    bt = BinaryTree()
    bt.insert(5)
    bt.insert(10)
    bt.insert(3)
    bt.insert(4)
    print("\nPre order")
    pre_order(bt.root)
    
    print("\nIn order")
    in_order(bt.root)
    
    print("\nPost order")
    post_order(bt())
    
if __name__ == '__main__':
    main()