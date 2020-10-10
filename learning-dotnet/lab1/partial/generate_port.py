ft = """
using System;

namespace partial
{{
    partial class Port
    {{
        {}
    }}
}}
"""

mt = """
public string {0}()
{{
    return "{0}";
}}
"""

def make_method(name):
    return mt.format(name)

ILE_METOD = 2
names = map(lambda n: "metoda" + str(n), range(1, ILE_METOD + 1))
methods = "\n".join(list(map(make_method, names)))

content = ft.format(methods)

with open("Port.cs", "w") as f:
    f.write(content)
