from typing import Any 
from .logging import logging

BRANCH_LIMIT = 10

class BaseTree(object):
    """
    `BaseTree` class for implementing tree,

    #### :params:

    value: Tree value.
    limit: branch limit (default to BRANCH_LIMIT = 10).
    before_branches: list of Tree, before current Tree.
    after_branches: list of Tree, after current Tree.

    #### :methods:
    is_full(): return True is if length of branches is limit.
    length(): return the length of Tree value.
    weight(): return the weight of Tree value.
 

    """
    def __init__(self, value: Any, limit: int|None = BRANCH_LIMIT):
        self.value: Any = value 
        self.limit: int = limit

        self.before_branches: list[BaseTree] = []
        self.after_branches: list[BaseTree] = []

    # features of before_branches
    
    def before_branch_length(self) -> int:
        """
        Returns the length of `before_branches`.
        """
        return len(self.before_branches)

    def is_before_full(self) -> bool:
        """
        Return `True` is length of `before_branches` is greater than 
        or equal to `limit` (default to BRANCH_LIMIT = 10).
        """
        return len(self.before_branches) >= self.limit

    # features of after_branches
    
    def after_branch_length(self) -> int:
        """
        Returns the length of `after_branches`.
        """
        return len(self.after_branches)

    def is_right_full(self) -> bool:
        """
        Return `True` is length of `after_branches` is greater than 
        or equal to `limit` (default to BRANCH_LIMIT = 10).
        """
        return len(self.after_branches) >= self.limit

    # 

    def length(self) -> int:
        """
        Return the lenght of `Tree` value.
        """
        return len(self.value)
    
    def weight(self) -> int:
        return sum(
            
        )

