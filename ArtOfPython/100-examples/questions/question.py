"""

Vlad found a string s
 consisting of n
 lowercase Latin letters, and he wants to make it as short as possible.

To do this, he can remove any pair of adjacent characters from s
 any number of times, provided they are different. For example, if s
=racoon, then by removing one pair of characters he can obtain the strings coon, roon, raon, and raco, but he cannot obtain racn (because the removed letters were the same) or rcon (because the removed letters were not adjacent).

What is the minimum length Vlad can achieve by applying any number of deletions?

Input
The first line of the input contains a single integer t
 (1≤t≤104
) — the number of test cases. Descriptions of the test cases follow.

The first line of each test case contains a single integer n
 (1≤n≤2⋅105
) — the length of the string s
.

The second line of each test case contains the string s
 consisting of n
 lowercase Latin letters.

It is guaranteed that the sum of n
 over all test cases does not exceed 2⋅105
.

Output
For each test case, output a single number—the minimum length of the string s
, after removing pairs of adjacent characters with different values.


inputCopy
10
4
aabc
5
abaca
10
avbvvcvvvd
7
abcdefg
5
dabbb
8
aacebeaa
7
bbbbacc
6
dacfcc
6
fdfcdc
9
dbdcfbbdc


outputCopy
0
1
2
1
1
0
1
0
0
1
"""

def each_char_differ(s: str) -> bool:
    if len(s) == 0:
        return True
    s_list = [x for x in s]
    i = 0
    while (i  != (len(s_list)-1)):
        first, cur, rest = s_list[:i], s_list[i], s_list[i+1:]
        if cur in first or cur in rest:
            return False
        i += 1
    return True
        
        
def diff_adj_char(s: str) -> bool:
    s_list = [x for x in s]
    i = 0
    while i < len(s_list)-1:
        if s_list[i] != s_list[i+1]:
            return True
        i += 1
    return False

def remove_adjacent_diff_char_pairs(s: str) -> str:
    s_list = [x for x in s]
    i = 0
    while i < len(s_list)-1:
        if s_list[i] != s_list[i+1]:
            s_list = s_list[:i] + s_list[i+2:]
        i += 1
    return "".join(s_list)
    
# test_case = int(input())
# while test_case:
#     s_len = int(input())
#     s = input().strip()

#     while diff_adj_char(s):
#         s = remove_adjacent_diff_char_pairs(s)
#     print(len(s))
#     test_case -=1
    
    
def check_cases(l, s):
    while diff_adj_char(s):
        s = remove_adjacent_diff_char_pairs(s)
    print(len(s))

cases = [
    (4, 'aabc'),
    (5, 'abaca'),
    (10, 'avbvvcvvvd'),
    (7, 'abcdefg'),
    (5, 'dabbb'),
    (8, 'aacebeaa'),
    (7, 'bbbbacc'),
    (6, 'dacfcc'),
    (6, 'fdfcdc'),
    (9, 'dbdcfbbdc')
]

for l, s in cases:
    check_cases(l, s)
    
