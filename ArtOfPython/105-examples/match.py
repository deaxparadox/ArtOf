
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
        case ['quit']:
            print("Goodbye!")
        case ['look']:
            print("Looking at current room.")
        case ['get', obj]:
            print("Get user in current room.")
        case ['get', direction]:
            print("Go in that direction")

def multiple_values(): 
    match command.split():
        case ["drop", *objects]:
            print(objects)
        case default:
            print("Universal case")


def example_2():
    subject: str = 'go'
    direction: list[str] = ['north', 'south', 'east', 'west']

    
    match command.split():
        case ['go', direction as i] :
            print("we can go in {}".format(i))
        case ['get', ('down'| 'up')]:
            print("Action specified")
        case ['go', *direction]:
            print("We can go in any direction.")
        case _:
            print("Unable to procceed.")

def example_3(): 
    match command.split():
        case ['north'] | ['go', 'north']:
            print("North direction")
        case ["get", obj] | ["pick", "up", obj] | ["pick", obj, "up"]:
            print("Action {}".format(obj))
        case _:
            print("Unable to proceed")

example_3()