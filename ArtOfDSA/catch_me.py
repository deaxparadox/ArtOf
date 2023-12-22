from time import sleep
import sys
import signal

def message_on_int(*args, **kwargs):
    global breaker
    print(args, kwargs)
    print("Program was interrupted")
    breaker = False
    sys.exit(130)
    

breaker = True


signal.signal(signal.SIGINT, message_on_int)


if __name__ == "__main__":
    while breaker:
        print("Looping")
        sleep(.3)
