#!/usr/bin/env python
"""Tests for `day_1` package."""

from pathlib import Path

from click.testing import CliRunner

from day_1 import cli, day_1


def test_fuel_requirement():
    """Examples from the puzzle description."""
    assert day_1.fuel_requirement(12) == 2
    assert day_1.fuel_requirement(14) == 2
    assert day_1.fuel_requirement(1969) == 654
    assert day_1.fuel_requirement(100756) == 33583


def test_additional_fuel_requirements():
    """Examples from the puzzle description."""
    assert list(day_1.additional_fuel_requirements(2)) == []
    assert list(day_1.additional_fuel_requirements(654)) == [216, 70, 21, 5]
    assert list(day_1.additional_fuel_requirements(33584)) == [
        11192, 3728, 1240, 411, 135, 43, 12, 2
    ]


def test_full_fuel_requirement():
    """Examples from the puzzle description."""
    assert day_1.full_fuel_requirement(14) == 2
    assert day_1.full_fuel_requirement(1969) == 966
    assert day_1.full_fuel_requirement(100756) == 50346


def test_part_1():
    """Solution for part 1."""
    with (Path(__file__).parent / 'input').open('r') as f:
        puzzle_input = [int(line) for line in f.readlines()]
    assert sum(day_1.fuel_requirement(mass)
               for mass in puzzle_input) == 3372695


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
        puzzle_input = f.readlines()
    puzzle_result = runner.invoke(cli.main, puzzle_input)
    assert puzzle_result.exit_code == 0
    assert int(puzzle_result.output) == 5056172  # Solution for Part 2
