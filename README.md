24 Solver
=========

The [24 Game](https://en.wikipedia.org/wiki/24_Game) is a game where a player must use the four given integers construct the number 24. The player can only use multiplication, division, addition, and subtraction. Each number must be used once, but you can use operations more than once. For example, given the integers, 3, 3, 8, and 8, one could construct the equation `8 / (3 - 8/3) = 24`.

This solver solves the game for any goal number (not just 24), any number of initial integers, and any operations. It is written in [Rust](http://www.rust-lang.org/), and the algorithm and source are completely documented if would like to understand how it works.
