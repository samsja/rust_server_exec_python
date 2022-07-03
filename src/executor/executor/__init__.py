import time

class MySlowToLoadExec():

    def __init__(self):
        time.sleep(1) # It simulates a slow Executor initialization like a deep learning model loading.
        # I added this line to be sure that the Executor is not re initialized at each called of the http server.
        # Otherwise each call will take at least one second

    def foo(self):
        print('hello rust world') # this will be printed on the server side
        return 'hello web world' # this will be returned to the client side
