#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""Tests for `day_6` package."""

from pathlib import Path

import pytest
from click.testing import CliRunner

from day_6 import cli, day_6


@pytest.fixture
def example_map() -> day_6.Map:
    return day_6.parse_map([
        "COM)B",
        "B)C",
        "C)D",
        "D)E",
        "E)F",
        "B)G",
        "G)H",
        "D)I",
        "E)J",
        "J)K",
        "K)L",
    ])


def test_examples(example_map: day_6.Map):
    orbits = example_map.total_orbits()
    assert orbits['D'] == 3
    assert orbits['L'] == 7
    assert orbits['COM'] == 0


def test_command_line_interface():
    """Test the CLI."""
    runner = CliRunner()
    result = runner.invoke(cli.main)
    assert result.exit_code == 0
    assert '0' in result.output
    help_result = runner.invoke(cli.main, ['--help'])
    assert help_result.exit_code == 0
    assert '--help  Show this message and exit.' in help_result.output

    with (Path(__file__).parent / 'input').open('r') as f:
        puzzle_input = [line.strip() for line in f.readlines()]
    puzzle_result = runner.invoke(cli.main, puzzle_input)
    assert puzzle_result.exit_code == 0
    assert int(puzzle_result.output) == 204521  # Solution for Part 1
