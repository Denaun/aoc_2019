# -*- coding: utf-8 -*-
"""Console script for day_6."""
import sys
from typing import List

import click

from day_6.day_6 import parse_map


@click.command()
@click.argument('orbit', nargs=-1)
def main(orbit=List[int]):
    """Console script for day_6."""
    click.echo(sum(parse_map(orbit).total_orbits().values()))
    return 0


if __name__ == "__main__":
    sys.exit(main())  # pragma: no cover
