tail +22 < wyniki.txt > /tmp/wyniki-czyste
# TODO: remove the tmp file / do it all in python and pass stdin
python3 - <<'____HERE'
from collections import defaultdict, Counter
with open("/tmp/wyniki-czyste") as f:
    data = defaultdict(list)
    selector = ""
    for line in f.readlines():
        line = line.rstrip()
        if len(line) == 6 and line.isnumeric():
            selector = line
        elif line.endswith("%"):
            pass
        else:
            try:
                data[selector].append(float(line.replace(",", ".")))
            except ValueError:
                pass
    
    
for k, v in data.items():                
    data[k] = v[:-5]

print(data)

for k, v in data.items():                
    data[k] = v[-1]
print(data)

aggregate = Counter(data.values())
labels, values = zip(*sorted(aggregate.items()))

import matplotlib.pyplot as plt
plt.bar(labels, values, width=0.5)
plt.xlabel('score')
plt.ylabel('count')
plt.show()
____HERE

