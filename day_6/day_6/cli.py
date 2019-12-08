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
    if orbit:
        click.echo(len(parse_map(orbit).shortest_path('YOU', 'SAN')) - 1)
    else:
        click.echo('At least one orbit required.')
    return 0


if __name__ == "__main__":
    sys.exit(main())  # pragma: no cover
