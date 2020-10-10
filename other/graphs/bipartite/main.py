#!/usr/bin/env python3
from collections import defaultdict, namedtuple
from enum import Enum
from itertools import chain


def main():
    graph = Graph.from_input()
    connected_subgraphs = graph.chunk()

    bipartities = [graph.bipartity() for graph in connected_subgraphs]

    if all(bipartities):
        print("The graph is bipartite and the partitions are:")
        print("A: {!r}".format(set(chain.from_iterable(map(lambda x: x[0], bipartities)))))
        print("B: {!r}".format(set(chain.from_iterable(map(lambda x: x[1], bipartities)))))
    else:
        print("The graph is NOT bipartite")


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

    def subgraph(self, vertices):
        """
        Creates a new graph with a subset of vertices
        """

        if not set(self.vertices).issuperset(set(vertices)):
            raise ValueError("Subgraph contains vertices that are NOT in the parent graph")

        partial_adjacency_list = {vertex: neighbours for vertex, neighbours in self.adjacency_list.items() if vertex in vertices}

        subgraph = self.__class__.__new__(self.__class__)
        subgraph.adjacency_list = partial_adjacency_list
        return subgraph

    @property
    def vertices(self):
        return tuple(self.adjacency_list.keys())

    def bipartity(self):
        """
        Returns partitions if self is bipartite or False otherwise

        Requires the graph to be connected, otherwise will check bipartity only for a connected sub-graph
        """

        QueueItem = namedtuple("QueueItem", "origin_color vertices")

        visited = set()
        color_map = VertexColorMap(self)
        queue = [QueueItem(origin_color=Colors.white, vertices=[self.vertices[0]])]

        while queue:
            current_item = queue.pop(0)

            vertices = current_item.vertices
            current_color = Colors.another(current_item.origin_color)

            for vertex in vertices:
                try:
                    color_map.color(vertex, current_color)
                except ValueError:
                    return False

                if vertex not in visited:
                    queue.append(QueueItem(origin_color=current_color, vertices=self.adjacency_list[vertex]))
                    visited.add(vertex)

        partitions = (color_map.get_vertices_of_color(Colors.white), color_map.get_vertices_of_color(Colors.black))
        return tuple(map(list, partitions))

    def chunk(self):
        """
        Chunks the graph into a tuple of connected sub-graphs (each of the sub-graphs is a connected graph).
        """

        def list_connected_vertices(start_vertex):
            queue = [start_vertex]
            vertices = []

            while queue:
                vertex = queue.pop(0)
                vertices.append(vertex)
                queue.extend(filter(lambda v: v not in vertices, self.adjacency_list[vertex]))

            return vertices

        all_unused_vertices = set(self.vertices)
        subgraphs = []

        while all_unused_vertices:
            subgraph_vertices = list_connected_vertices(all_unused_vertices.pop())

            subgraphs.append(self.subgraph(subgraph_vertices))
            all_unused_vertices.difference(subgraph_vertices)

        return subgraphs


class Colors(Enum):
    white = 1
    black = 2
    none = 0

    @staticmethod
    def another(_color):
        color = Colors(_color)

        if color == Colors.white:
            return Colors.black
        elif color == Colors.black:
            return Colors.white
        else:
            raise TypeError("This is not a valid color")


class VertexColorMap:
    def __init__(self, graph):
        self.color_mapping = {vertex: Colors.none for vertex in graph.vertices}

    def color(self, vertex, _color):
        color = Colors(_color)

        if self.color_mapping[vertex] not in (Colors.none, color):
            raise ValueError("This vertex is already colored with different colour")

        self.color_mapping[vertex] = color

    def get_vertices_of_color(self, _color):
        color = Colors(_color)

        return filter(lambda v: self.color_mapping[v] == color, self.color_mapping.keys())


if __name__ == "__main__":
    main()
