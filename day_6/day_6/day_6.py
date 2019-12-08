# -*- coding: utf-8 -*-
"""Main module."""

from collections import defaultdict, deque
from typing import Dict, List


class Map:
    def __init__(self) -> None:
        self.parents: Dict[str, str] = {}
        self.children: Dict[str, List[str]] = defaultdict(list)

    def add(self, parent: str, child: str) -> None:
        assert child not in self.parents
        self.parents[child] = parent
        self.children[parent].append(child)

    def connections(self, node: str) -> List[str]:
        return (([self.parents[node]] if node in self.parents else []) +
                self.children[node])

    def total_orbits(self, root: str = 'COM') -> Dict[str, int]:
        result: Dict[str, int] = {root: 0}
        to_visit = self.children[root].copy()
        while to_visit:
            current = to_visit.pop()
            if current in result:
                continue
            result[current] = result[self.parents[current]] + 1
            to_visit.extend(self.children[current])
        return result

    def shortest_path(self, src: str, dst: str) -> List[str]:
        paths: Dict[str, List[str]] = {src: []}
        # Initial state with empty connections to avoid double counting.
        paths.update({node: [] for node in self.connections(src)})
        to_visit = deque()

        def visit(node: src):
            assert node in paths
            connection_path = paths[node] + [node]
            for other in self.connections(node):
                if other in paths:
                    continue
                paths[other] = connection_path
                to_visit.append(other)

        for node in self.connections(src):
            visit(node)
        while dst not in to_visit:
            current = to_visit.popleft()
            visit(current)
        return paths[dst]


def parse_map(orbits: List[str]) -> Map:
    result = Map()
    for parent, child in (orbit.split(')') for orbit in orbits):
        result.add(parent, child)
    return result
