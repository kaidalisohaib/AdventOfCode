# Advent of Code Solutions in Rust

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This repository contains my personal solutions for the [Advent of Code](https://adventofcode.com/) programming puzzles, implemented in Rust. It serves as a practical demonstration of my problem-solving abilities, algorithmic thinking, and proficiency in modern, memory-safe systems programming.

## About The Project

Advent of Code (AoC) is an annual set of Christmas-themed computer programming challenges that follow an Advent calendar format. These puzzles require a combination of creative thinking, algorithmic knowledge, and efficient coding to solve.

This repository is my journey through these challenges, showcasing my approach to tackling diverse problems. Each day's solution is a self-contained Rust project, emphasizing clean, readable, and efficient code.

Currently, this repository includes solutions for the following years:
*   [2015](AdventOfCode/2015)
*   [2016](AdventOfCode/2016)
*   [2022](AdventOfCode/2022)

_Solutions for other years are in progress!_

---
<!--
Here's a placeholder for a cool visual. A GIF showing one of the more visual puzzles (like Day 18's Game of Life) would be excellent here.
![Project Demo GIF](path/to/your/demo.gif)
-->

### Tech Stack

This project is built entirely in **Rust**, leveraging its performance, safety features, and rich ecosystem. Key external crates used include:

*   **[Regex](https://crates.io/crates/regex):** For complex string parsing and pattern matching.
*   **[Enum-Iterator](https://crates.io/crates/enum-iterator):** To simplify iteration over enum variants.
*   **[PHF](https://crates.io/crates/phf):** For building compile-time hashmaps, used for efficient lookups.
*   **[Priority-Queue](https://crates.io/crates/priority-queue):** For implementing priority queue data structures, essential for pathfinding algorithms like Dijkstra's.

## Project Structure

The repository is organized by year, with each year containing subdirectories for each day's puzzle.

```
.
├── AdventOfCode
│   ├── 2015
│   │   ├── day01
│   │   │   ├── src/main.rs
│   │   │   ├── Cargo.toml
│   │   │   └── input.txt
│   │   ├── day02
│   │   └── ... (and so on for other days)
│   ├── 2016
│   │   └── ...
│   └── 2022
│       └── ...
└── README.md
```
Each daily challenge is a standalone Cargo project, making it easy to navigate, build, and run individual solutions.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

You will need the Rust programming language and its package manager, Cargo, installed on your system. You can install them via [rustup](https://rustup.rs/).

### Installation & Running

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/kaidalisohaib/AdventOfCode.git
    ```
2.  **Navigate to a specific day's directory:**
    ```sh
    cd AdventOfCode/2015/day01
    ```
3.  **Run the solution:**
    ```sh
    cargo run
    ```
    This will compile and execute the `main.rs` file, printing the solutions for both parts of the day's puzzle to the console.

## Technical Challenges & Lessons Learned

While solving these puzzles, I encountered several interesting challenges that required applying specific computer science concepts. Here are a few examples:

1.  **Combinatorial Optimization (2015, Day 13):** The "Knights of the Dinner Table" puzzle is a variation of the Traveling Salesperson Problem (TSP). My solution implements a brute-force permutation algorithm to explore all possible seating arrangements and find the one with the maximum total happiness. This demonstrates an understanding of combinatorial problems and how to systematically explore a solution space. For larger inputs, this would be optimized with heuristics or dynamic programming.

2.  **State-Space Search (2015, Day 22):** The "Wizard Simulator 20XX" puzzle is a complex state-space search problem with resource management (mana) and status effects. I implemented a search algorithm using a **priority queue** to find the path with the minimum mana cost to win the battle. This required careful state management (player/boss health, mana, active spell timers) and showcases my ability to model complex systems and apply graph traversal algorithms like Dijkstra's to find optimal solutions.

3.  **String Manipulation and Parsing (Multiple Days):** Many puzzles, like **2015, Day 5** and **Day 8**, involve intricate string manipulation and parsing rules. I utilized Rust's powerful string handling and the `regex` crate to efficiently process text, count character occurrences, and validate complex patterns, demonstrating practical data processing skills.

## Future Improvements

- [ ] Complete the remaining days for all years.
- [ ] Refactor older solutions with new Rust features and improved algorithms.
- [ ] Add comprehensive unit tests to ensure correctness for all edge cases.
- [ ] Implement data visualizations for puzzles with a graphical component (e.g., grid-based problems).

- 2015
  - [X] Day 1
  - [X] Day 2
  - [X] Day 3
  - [X] Day 4
  - [X] Day 5
  - [X] Day 6
  - [X] Day 7
  - [X] Day 8
  - [X] Day 9
  - [X] Day 10
  - [X] Day 11
  - [X] Day 12
  - [X] Day 13
  - [X] Day 14
  - [X] Day 15
  - [X] Day 16
  - [X] Day 17
  - [X] Day 18
  - [X] Day 19
  - [X] Day 20
  - [X] Day 21
  - [X] Day 22
  - [X] Day 23
  - [X] Day 24
  - [X] Day 25
- 2016
  - [X] Day 1
  - [X] Day 2
  - [X] Day 3
  - [X] Day 4
  - [X] Day 5
  - [X] Day 6
  - [X] Day 7
  - [X] Day 8
  - [X] Day 9
  - [X] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2017
  - [ ] Day 1
  - [ ] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2018
  - [ ] Day 1
  - [ ] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2019
  - [ ] Day 1
  - [ ] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2020
  - [ ] Day 1
  - [ ] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2021
  - [ ] Day 1
  - [ ] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
- 2022
  - [X] Day 1
  - [X] Day 2
  - [ ] Day 3
  - [ ] Day 4
  - [ ] Day 5
  - [ ] Day 6
  - [ ] Day 7
  - [ ] Day 8
  - [ ] Day 9
  - [ ] Day 10
  - [ ] Day 11
  - [ ] Day 12
  - [ ] Day 13
  - [ ] Day 14
  - [ ] Day 15
  - [ ] Day 16
  - [ ] Day 17
  - [ ] Day 18
  - [ ] Day 19
  - [ ] Day 20
  - [ ] Day 21
  - [ ] Day 22
  - [ ] Day 23
  - [ ] Day 24
  - [ ] Day 25
