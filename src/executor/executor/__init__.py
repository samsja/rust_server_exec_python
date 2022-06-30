import time

class MySlowToLoadExec():

    def __init__(self):
        time.sleep(1)

    def foo(self):
        print('hello rust world')
        return 'hello web world'

def executor_foo():
    print('hello rust world')
