# frozen_string_literal: true

require 'ostruct'
require 'day_3/version'

module Day3
  class Error < StandardError; end

  class Point
    def initialize(x, y)
      @x = x
      @y = y
    end

    attr_reader :x
    attr_reader :y

    def to_s
      "(#{x}, #{y})"
    end

    def manhattan_length
      @x.abs + @y.abs
    end

    def ==(other)
      @x == other.x && @y == other.y
    end

    def eql?(other)
      self == other
    end

    def hash
      "#{x}-#{y}".hash
    end
  end

  def self.parse_segment(str)
    segment = OpenStruct.new
    segment.direction = case str[0]
                        when 'R'
                          :right
                        when 'L'
                          :left
                        when 'U'
                          :up
                        when 'D'
                          :down
                        else
                          raise Error, "Unknown character #{dir}"
    end
    segment.length = str[1..-1].to_i
    segment
  end

  def self.parse_wire(str)
    str.split(',').collect { |x| parse_segment(x) }
  end

  def self.to_points(segments)
    start = Point.new(0, 0)
    points = []
    segments.each do |s|
      values = (1..s.length)
      points += case s.direction
                when :right
                  values.collect { |v| Point.new(start.x + v, start.y) }
                when :left
                  values.collect { |v| Point.new(start.x - v, start.y) }
                when :up
                  values.collect { |v| Point.new(start.x, start.y + v) }
                when :down
                  values.collect { |v| Point.new(start.x, start.y - v) }
        end
      start = points.last
    end
    points
  end

  def self.solve1(wire1, wire2)
    (to_points(parse_wire wire1) & to_points(parse_wire wire2))
      .collect(&:manhattan_length)
      .to_a
      .min
  end

  def self.solve2(wire1, wire2)
    points1 = Hash[to_points(parse_wire wire1).zip (1..)]
    points2 = Hash[to_points(parse_wire wire2).zip (1..)]
    (points1.keys & points2.keys)
      .collect { |pt| points1[pt] + points2[pt] }
      .to_a
      .min
  end
end
