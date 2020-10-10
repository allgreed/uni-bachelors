from collections import defaultdict
from heapq import _heapify_max as make_max_heap, _heappop_max as pop_from_heap
import sys
from sys import argv


class ColorMap(defaultdict):
    def __str__(self):
       return "[ {} ]".format(", ".join([str(v) for _,v in sorted(self.items())]))


def main():
    graph = Graph.from_input()

    color_map = ColorMap(lambda: None)
    processing_queue = [(graph.degree(vertex), vertex) for vertex in graph.vertices] 
    make_max_heap(processing_queue)

    while processing_queue:
        _, vertex = pop_from_heap(processing_queue)

        i = 0
        while color_map[vertex] is None:
            if all((color_map[neighbour] != i for neighbour in graph.adjacency_list[vertex])):
                color_map[vertex] = i
            i += 1

    coloring = color_map
    chromatic_number = len(set(color_map.values()))

    try:
        command = sys.argv[1]
        if command == "-c":
            print(chromatic_number)
        elif command == "-m":
            print(coloring)
        else:
            raise ValueError("No such command")

    except KeyError:
        print(coloring)
        print(chromatic_number)


class Graph:
    def __init__(self, edges):
    
        self.adjacency_list = defaultdict(list)

        for edge in edges:
            for vertex in edge:
                self.adjacency_list[vertex] += edge - {vertex}

    @classmethod
    def from_input(cls):
        """
        Acquires list of edges from stdin.
        """

        def line_by_line_input_reader():
            try:
                while True:
                    vertex_pair = input().split()
                    yield set(vertex_pair)
            except EOFError:
                raise StopIteration()

        return cls(line_by_line_input_reader())

    @property
    def vertices(self):
        return list(self.adjacency_list.keys())

    def degree(self, vertex):
        return len(self.adjacency_list[vertex])


if __name__ == "__main__":
    main()
