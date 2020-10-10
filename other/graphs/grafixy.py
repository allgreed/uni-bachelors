#!/usr/bin/env python3
from collections import defaultdict


def main():
    """
    Demonstrating that a graph programme can be writen
    with little to None graph knowledge
    """

    degrees = defaultdict(int)

    # via the handshaking lemma 
    for edge in aquire_edges():
        for vertex in edge:
            degrees[vertex] += 1

    output_results(degrees.values(), max(degrees.values()))


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


def output_results(degrees, graph_degree):

    graphical_series = tuple(sorted(degrees, reverse=True))
    print("Graphical series: {}".format(graphical_series))
    print("Graph degree: {}".format(graph_degree))


if __name__ == "__main__":
    main()
