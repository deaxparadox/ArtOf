

status = 302

match status:
    case 200:
        print("Successfull")
    case 300 | 301 | 302:
        print("Redirected")
    case _:
        print("Unknown!")
