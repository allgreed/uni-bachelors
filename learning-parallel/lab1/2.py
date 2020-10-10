import os
import sys


def shell(cmd):
    os.system(cmd)


def main():
    path = sys.argv[1]
    

    while True:
        print("> ", end="", flush=True)
        # TODO: dehardcode this
        c = input()  

        def read():
            os.system("cat %s" % path)

        def write():
            m = input()
            os.system("echo {} > {}".format(m, path))
    
        choices = {
            "0": read,
            "1": write,
            "2": exit,
        }

        def err():
            print("Nie pykło")

        f = choices.get(c, err)
        f()

if __name__ == "__main__":
    main()
