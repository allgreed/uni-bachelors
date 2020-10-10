#!/usr/bin/env python3
S1 = "ąężźćńółśĄĘŻŹĆŃÓŁŚ,"
S2 = "aezzcnolsAEZZCNOLS."

with open("./dane_do_zestawu_25.csv") as f:
    for line in f.readlines():
        for c, r in zip(S1, S2):
            line = line.replace(c, r)  

        print(line, end="")
