"""
Condition based while loop
"""
# count = 0
# while count <=5:
#     print(count)
#     count += 1

"""
break statement
"""
# count = 0
# while count <= 5:
#     if count == 3:
#         break 
#     print(count)
#     count += 1

"""
continue Statement
"""
# count = 0
# while count <= 5:
#     if count == 3:
#         count += 1
#         continue
#     print(count)
#     count += 1


"""
`else` clauses
"""
count = 0
breaker = True
while breaker:
    if count >= 5:
        breaker = False
    print(count)
    count += 1
else:
    print("Counter is greator than 5")