def decorator(O):
    # Save or augment function or class O
    return O

@decorator
def F(): ...            # F = decorator(F)

@decorator
class C: ...            # C = decorator(C)