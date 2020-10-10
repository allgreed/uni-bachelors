import networkx as nx
import numpy as np


def aquire_edges():
    """
    Prompts and processess user input as a list of Edges
    """

    print("Please input newline seperate list of edges. When you're done enter a blank line.")

    while True:
        input_data = input()

        if not input_data:
            break

        yield (input_data.split())    


edges = list(aquire_edges())

from itertools import chain

vertices = list(set(chain.from_iterable(edges)))

incidence_matrix = [[0] * len(edges)] * len(vertices)

#for i, edge in enumerate(edges):
#    for vertex in edge:
#        incidence_matrix[i][vertices.index(vertex)] = 1

print(incidence_matrix)
connect = [[1,0],[1,0]]
pos = np.random.rand(len(vertices), 2) #coordinates, (x, y) for 10 nodes

#creation of the graph
graph = nx.Graph()
#adding nodes/connections in the graph
for node in range(len(pos)):
    graph.add_node(node)

graph.add_edges_from(connect)

#plot of the nodes using the (x,y) pairs as coordinates
import matplotlib.pyplot as plt
nx.draw(graph, [(x,y) for x,y in pos], node_size=50)

#draw graph
plt.show()
