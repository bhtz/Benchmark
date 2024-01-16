from tkinter.messagebox import NO


class User(object):
    
    id = None
    firstname = None
    lastname = None
    email= None

    def __init__(self, id, firstname, lastname, email):
        self.id = id
        self.firstname = firstname
        self.lastname = lastname
        self.email = email