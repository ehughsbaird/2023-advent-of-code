/**
 * BSD 2-Clause License
 *
 * Copyright (c) 2023, ehughsbaird
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq)]
enum Tile {
    Ground,
    Start,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Tile::NS,
        '-' => Tile::EW,
        'L' => Tile::NE,
        'J' => Tile::NW,
        '7' => Tile::SW,
        'F' => Tile::SE,
        'S' => Tile::Start,
        '.' => Tile::Ground,
        _ => panic!("Bad tile"),
    }
}

fn dir1(pos: (usize, usize), tile: &Tile) -> (usize, usize) {
    match *tile {
        Tile::Start => (pos.0, pos.1),
        Tile::NS => (pos.0 - 1, pos.1),
        Tile::EW => (pos.0, pos.1 + 1),
        Tile::NE => (pos.0 - 1, pos.1),
        Tile::NW => (pos.0 - 1, pos.1),
        Tile::SW => (pos.0 + 1, pos.1),
        Tile::SE => (pos.0 + 1, pos.1),
        _ => pos,
    }
}

fn dir2(pos: (usize, usize), tile: &Tile) -> (usize, usize) {
    match *tile {
        Tile::Start => (pos.0, pos.1),
        Tile::NS => (pos.0 + 1, pos.1),
        Tile::EW => (pos.0, pos.1 - 1),
        Tile::NE => (pos.0, pos.1 + 1),
        Tile::NW => (pos.0, pos.1 - 1),
        Tile::SW => (pos.0, pos.1 - 1),
        Tile::SE => (pos.0, pos.1 + 1),
        _ => pos,
    }
}

// Advance from a position to a non-visited tile
fn advance(
    pos: (usize, usize),
    t: &Tile,
    visited: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let opt1 = dir1(pos, t);
    let opt2 = dir2(pos, t);
    if !visited.contains(&opt1) {
        return Some(opt1);
    }
    if !visited.contains(&opt2) {
        return Some(opt2);
    }
    return None;
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("data.txt not found");
    let mut data: Vec<&str> = data.split('\n').collect();
    while data.last().unwrap().len() == 0 {
        data.pop();
    }
    let data: Vec<Vec<Tile>> = data
        .into_iter()
        .map(|x: &str| -> Vec<Tile> { x.chars().map(char_to_tile).collect() })
        .collect();

    // Find the start tile
    let mut start = (0, 0);
    for i in 0..data.len() {
        let found = data[i].iter().position(|x| *x == Tile::Start);
        if found.is_some() {
            start.0 = i;
            start.1 = found.unwrap();
            break;
        }
    }

    // Find the two pipes leading in to the start tile
    let mut cursors = vec![];
    for (i, j) in vec![
        (start.0 - 1, start.1),
        (start.0 + 1, start.1),
        (start.0, start.1 - 1),
        (start.0, start.1 + 1),
    ] {
        if (i, j) == start {
            continue;
        }
        if dir1((i, j), &data[i][j]) == start || dir2((i, j), &data[i][j]) == start {
            cursors.push((i, j));
        }
    }

    // Walk each cursor along until they meet
    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(start);
    let mut cursorA = cursors[0];
    let mut cursorB = cursors[1];
    let mut steps = 0;
    loop {
        steps += 1;

        visited.insert(cursorA);
        match advance(cursorA, &data[cursorA.0][cursorA.1], &visited) {
            Some(new) => cursorA = new,
            // Cursors met!
            None => break,
        }

        visited.insert(cursorB);
        match advance(cursorB, &data[cursorB.0][cursorB.1], &visited) {
            Some(new) => cursorB = new,
            // Cursors met!
            None => break,
        }
    }
    println!("farthest is {} steps", steps);
}
