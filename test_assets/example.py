def foo():
    return "baz"


bar = 5


def multiply(a, b):
    return a*b


def create_dict(arg1="a", arg2=None, *others):
    return {
        "first": arg1,
        "second": arg2,
        "others": others
    }


counter = 0


def inc():
    global counter
    counter += 1
    return counter
