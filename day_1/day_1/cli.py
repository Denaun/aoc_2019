"""Console script for day_1."""
import sys
from typing import List

import click

from day_1.day_1 import fuel_requirement


@click.command()
@click.argument('mass', nargs=-1, type=int)
def main(mass: List[int]) -> int:
    """Console script for day_1."""
    click.echo(sum(fuel_requirement(m) for m in mass))
    return 0


if __name__ == "__main__":
    sys.exit(main())  # pragma: no cover
