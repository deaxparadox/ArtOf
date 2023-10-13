
def singleton(aClass):                                  # On @ decoration
    def onCall(*args, **kwargs):                        # On instance creation
        if onCall.instance == None:
            onCall.instance = aClass(*args, **kwargs)   # One function per class
        return onCall.instance
    onCall.instance = None
    return onCall


class singleton:                    
    def __init__(self, aClass):                             # On @ decoration
        self.aClass = aClass
        self.instance = None
    def __call__(self, *args, **kwargs):                    # On instance creation
        if self.instance == None:   
            self.instance = self.aClass(*args, **kwargs)    # One instance per class
        return self.instance