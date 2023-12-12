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
use std::fs;

#[derive(Copy)]
enum Tile {
    Galaxy,
    Space,
    DoubleSpace
}

fn char_to_tile(c: char) -> Tile
{
    match c {
        '#' => Tile::Galaxy,
        '.' => Tile::Space,
        _ => panic!("Nope")
    }
}

fn tile_to_char(t: Tile) -> char
{
    match t {
        Tile::Galaxy => '#',
        Tile::Space => '.',
        Tile::DoubleSpace => '_'
    }
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
}
