class Home:
    def __init__(self):
        self.name = "Super house 1"
    def welcome_message(self):
        return "Welcome to {}".format(self.name)
    
h = Home()
print(h.name)
print(h.welcome_message())
