# -*- coding: utf-8 -*-
"""Main module."""

from typing import Dict, List
from collections import defaultdict


class Map:
    def __init__(self) -> None:
        self._parents = {}  # type: Dict[str, str]
        self._children = defaultdict(list)  # type: Dict[str, List[str]]

    def add(self, parent: str, child: str) -> None:
        assert child not in self._parents
        self._parents[child] = parent
        self._children[parent].append(child)

    def total_orbits(self, root: str = 'COM') -> Dict[str, int]:
        result = {root: 0}  # Dict[str, int]
        to_visit = set(self._children[root])
        while to_visit:
            current = to_visit.pop()
            if current in result:
                continue
            result[current] = result[self._parents[current]] + 1
            to_visit.update(self._children[current])
        return result


def parse_map(orbits: List[str]) -> Map:
    result = Map()
    for parent, child in (orbit.split(')') for orbit in orbits):
        result.add(parent, child)
    return result
