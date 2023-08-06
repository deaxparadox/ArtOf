
def example_1():

    status = 302

    match status:
        case 200:
            print("Successfull")
        case 300 | 301 | 302:
            print("Redirected")
        case _:
            print("Unknown!")


command = input("What are you doing next? ")

# match command.split():
#     case [action, obj]:
#         ...


def multiple_patterns():
    match command.split():
        case [action]:
            print("interpret single-verb action ")
        case [action, obj]:
            print("interpret action, object")


def specific_values():
    match command.split():
        case ['split']:
            print("Goodbye!")
        case ['look']:
            print("Looking at current room.")
        case ['get', obj]:
            print("Get user in current room.")
        case ['get', direction]:
            print("Go in that direction")

specific_values()