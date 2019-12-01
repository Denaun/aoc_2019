"""Main module."""

from typing import Iterator, List


def fuel_requirement(mass: int) -> int:
    """Base fuel requirements for a single module."""
    return (mass // 3) - 2


def additional_fuel_requirements(fuel_mass: int) -> Iterator[int]:
    """Generator for the intermediate fuel requirements of a module."""
    step = fuel_requirement(fuel_mass)
    while step > 0:
        yield step
        step = fuel_requirement(step)


def full_fuel_requirement(mass: int) -> int:
    """Complete fuel requirements for a single module."""
    base_fuel = fuel_requirement(mass)
    return base_fuel + sum(additional_fuel_requirements(base_fuel))
