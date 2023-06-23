import typing
from ..logging import logging

alpha_weight: list[tuple[str, int]] = [
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5),
    ("f", 6),
    ("g", 7),
    ("h", 8),
    ("i", 9),
    ("j", 10),
    ("k", 11),
    ("l", 12),
    ("m", 13),
    ("n", 14),
    ("o", 15),
    ("p", 16),
    ("q", 17),
    ("r", 18),
    ("s", 19),
    ("t", 20),
    ("u", 21),
    ("v", 22),
    ("w", 23),
    ("x", 24),
    ("y", 25),
    ("z", 26),
]


class AlphaWeight(object):
    def __init__(self, alpha: str, weight: int) -> None:
        self.__alpha: str = alpha
        self.__weight: int = weight

    @property
    def alpha(self) -> str:
        return self.__alpha
    
    @property
    def weight(self) -> int:
        return self.__weight

class Alphabet(object):
    def __init__(self) -> None:
        self.__set()

    def __set(self):
        for alpha, weight in alpha_weight:
            setattr(
                self, 
                alpha,
                AlphaWeight(alpha, weight)
            )
        return 