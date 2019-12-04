# frozen_string_literal: true

RSpec.describe Day3 do
  it 'has a version number' do
    expect(Day3::VERSION).not_to be nil
  end

  it 'calculates Manhattan length correctly' do
    expect(Day3::Point.new(0, 0).manhattan_length).to eq(0)
    expect(Day3::Point.new(2, 0).manhattan_length).to eq(2)
    expect(Day3::Point.new(0, 2).manhattan_length).to eq(2)
    expect(Day3::Point.new(3, 4).manhattan_length).to eq(7)
    expect(Day3::Point.new(-3, -4).manhattan_length).to eq(7)
  end

  it 'solves the part 1 examples' do
    expect(Day3.solve1('R75,D30,R83,U83,L12,D49,R71,U7,L72',
                       'U62,R66,U55,R34,D71,R55,D58,R83')).to eq(159)
    expect(Day3.solve1('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
                       'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7')).to eq(135)
  end

  it 'solves part 1' do
    puzzle_input = File.open(File.join(File.dirname(__FILE__), 'input')).collect.to_a
    expect(Day3.solve1(puzzle_input[0], puzzle_input[1])).to eq(721)
  end

  it 'solves the part 2 examples' do
    expect(Day3.solve2('R75,D30,R83,U83,L12,D49,R71,U7,L72',
                       'U62,R66,U55,R34,D71,R55,D58,R83')).to eq(610)
    expect(Day3.solve2('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
                       'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7')).to eq(410)
  end

  it 'solves part 2' do
    puzzle_input = File.open(File.join(File.dirname(__FILE__), 'input')).collect.to_a
    expect(Day3.solve2(puzzle_input[0], puzzle_input[1])).to eq(7388)
  end
end
