package main

import "testing"

func TestPart1(t *testing.T) {
	part1Solution := 0
	part1Answer := part1("test_input.txt")
	if part1Answer != part1Solution {
		t.Fatalf("Part 1 failed, expected %d got: %d", part1Solution, part1Answer)
	}
}

func TestPart2(t *testing.T) {
	part2Solution := 0
	part2Answer := part2("test_input.txt")
	if part2Answer != part2Solution {
		t.Fatalf("Part 2 failed, expected %d got: %d", part2Solution, part2Answer)
	}
}
