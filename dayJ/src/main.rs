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

fn tile_to_char(t: &Tile) -> char {
    match *t {
        Tile::NS => '║',
        Tile::EW => '═',
        Tile::NE => '╚',
        Tile::NW => '╝',
        Tile::SW => '╗',
        Tile::SE => '╔',
        Tile::Start => 'S',
        Tile::Ground => '.',
    }
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

fn canvas(data: &Vec<Vec<Tile>>, visited: &HashSet<(usize, usize)>, start: (usize, usize)) {
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if start == (i, j) {
                print!("@");
            } else if visited.contains(&(i, j)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if visited.contains(&(i, j)) {
                print!(" ");
            } else {
                print!("{}", tile_to_char(&data[i][j]));
            }
        }
        println!();
    }
}

fn neighbors(pos: (usize, usize), data: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if pos.0 < 0 || pos.0 >= data.len() || pos.1 < 0 || pos.1 >= data[pos.0].len() {
        return ret;
    }
    if pos.0 > 0 {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < data.len() - 1 {
        ret.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < data[pos.0].len() - 1 {
        ret.push((pos.0, pos.1 + 1));
    }
    return ret;
}

fn main() {
    let data = fs::read_to_string("i2.txt").expect("data.txt not found");
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
    for (i, j) in neighbors((start.0, start.1), &data) {
        if (i, j) == start {
            continue;
        }
        if dir1((i, j), &data[i][j]) == start || dir2((i, j), &data[i][j]) == start {
            cursors.push((i, j));
        }
    }
    canvas(&data, &HashSet::<(usize, usize)>::new(), start);

    // Walk each cursor along until they meet
    let mut loop_tiles = HashSet::<(usize, usize)>::new();
    loop_tiles.insert(start);
    let mut cursor_a = cursors[0];
    let mut cursor_b = cursors[1];
    let mut steps = 0;
    loop {
        steps += 1;

        loop_tiles.insert(cursor_a);
        match advance(cursor_a, &data[cursor_a.0][cursor_a.1], &loop_tiles) {
            Some(new) => cursor_a = new,
            // Cursors met!
            None => break,
        }

        loop_tiles.insert(cursor_b);
        match advance(cursor_b, &data[cursor_b.0][cursor_b.1], &loop_tiles) {
            Some(new) => cursor_b = new,
            // Cursors met!
            None => break,
        }
    }
    canvas(&data, &loop_tiles, start);
    println!("farthest is {} steps", steps);

    // Find the enclosed area by flood-filling from each tile. Area inside the loop is counted by
    // the area of the flood-fills that terminate only by hitting loop tiles
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut total_area = 0;
    for i in 0..data.len() {
        for j in 0..data[1].len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            if loop_tiles.contains(&(i, j)) {
                continue;
            }
            // We're at a fresh, unvisited tile
            // We want to keep track of:
            // - how many tiles we visited
            // - What the tiles we've yet to visit are
            // - If we've terminated by hitting a edge of the map
            let mut area = 0;
            let mut queue = Vec::<(usize, usize)>::new();
            let mut seen = HashSet::<(usize, usize)>::new();
            queue.push((i, j));
            let mut enclosed = true;
            while !queue.is_empty() {
                let start = queue[queue.len() - 1];
                queue.pop();
                seen.insert(start);
                area += 1;
                let next = neighbors((start.0, start.1), &data);
                if next.len() != 4 {
                    enclosed = false;
                }
                for (di, dj) in next {
                    if seen.contains(&(di, dj)) {
                        continue;
                    }
                    if loop_tiles.contains(&(di, dj)) {
                        let one = dir1((di, dj), &data[i][j]);
                        let two = dir1((di, dj), &data[i][j]);
                        if !seen.contains(&one) && loop_tiles.contains(&one) && (dir1(one, &data[one.0][one.1]) == one || dir2(one, &data[one.0][one.1]) == one) {
                            queue.push(one);
                        }
                        seen.insert(one);
                        if !seen.contains(&two) && loop_tiles.contains(&two) && (dir1(two, &data[two.0][two.1]) == two || dir2(two, &data[two.0][two.1]) == two) {
                            queue.push(two);
                        }
                        seen.insert(two);
                        continue;
                    }
                    queue.push((di, dj));
                    seen.insert((di, dj));
                }
            }
            println!("============================");
            canvas(&data, &seen, (i, j));
            if enclosed {
                println!("Adding {} tiles", area);
                total_area += area;
            }
            for pos in seen {
                visited.insert(pos);
            }
        }
    }
    println!("Total area enclosed: {}", total_area);
}
