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
// Comment for cargo fmt
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Galaxy,
    Space,
    DoubleSpace,
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '#' => Tile::Galaxy,
        '.' => Tile::Space,
        _ => panic!("Nope"),
    }
}

fn tile_to_char(t: Tile) -> char {
    match t {
        Tile::Galaxy => '▞',
        Tile::Space => '▒',
        Tile::DoubleSpace => '░',
    }
}

fn canvas(data: &Vec<Vec<Tile>>) {
    for arr in data {
        for tile in arr {
            print!("{}", tile_to_char(*tile));
        }
        println!();
    }
}

fn a_star(domain: &Vec<Vec<Tile>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)>
{
    let todo = vec![start];
    let prev = HashSet::<(usize, usize), (usize, usize)>::new();

    let cost = HashSet::<(usize, usize), i32>::new();
    cost.add(start, 0);

    while todo.len() > 0 {
        let current = todo.pop().unwrap();
        if current == end {
            return reconstruct(prev, current);
        }
        for next in neighbors(current) {
        }
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("data.txt not found");
    let mut data: Vec<&str> = data.split('\n').collect();
    while data.last().unwrap().len() == 0 {
        data.pop();
    }
    let mut data: Vec<Vec<Tile>> = data
        .into_iter()
        .map(|x: &str| -> Vec<Tile> { x.chars().map(char_to_tile).collect() })
        .collect();
    // Any clear horizontal lines
    for line in &mut data {
        if !line.iter().any(|x| *x == Tile::Galaxy) {
            for tile in line {
                *tile = Tile::DoubleSpace
            }
        }
    }
    // Any clear vertical lines
    for x in 0..data.len() {
        let mut clean = true;
        for line in &data {
            if line[x] == Tile::Galaxy {
                clean = false;
                break;
            }
        }
        if !clean {
            continue;
        }
        for line in &mut data {
            line[x] = Tile::DoubleSpace;
        }
    }
}
