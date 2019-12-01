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
    assert int(puzzle_result.output) == 3372695
