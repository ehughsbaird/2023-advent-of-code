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
use std::collections::HashMap;
use std::fs;

fn time_for_cursor(cursor: &str, map: &HashMap<&str, (&str, &str)>, instructions: &str) -> i64 {
    let mut acc = 0;
    let mut current = cursor;
    while current.as_bytes()[2] != b'Z' {
        for c in instructions.chars() {
            acc += 1;
            let node = map.get(current).unwrap();
            if c == 'L' {
                current = node.0;
            }
            if c == 'R' {
                current = node.1;
            }
        }
    }
    return acc;
}

// As outlined in The Art of Computer Programming, Vol. 1, Page 2 :)
fn gcd(m: i64, n: i64) -> i64 {
    let remainder = m % n;
    if remainder == 0 {
        return n;
    }
    return gcd(n, remainder);
}

// https://stackoverflow.com/questions/3154454/what-is-the-most-efficient-way-to-calculate-the-least-common-multiple-of-two-int
fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read_to_string("data.txt").expect("data.txt not found or busy");
    let data: Vec<&str> = file.split("\n\n").collect();
    // Left, Left, Left, Right
    let instructions: &str = data[0];
    // Node = (Left, Right)
    let mut nodes: &str = data[1];
    // Get rid of the empty lines
    let mut idx = nodes.len() - 1;
    while nodes.as_bytes()[idx] == b'\n' {
        nodes = &nodes[0..idx];
        idx -= 1;
    }
    let mut map = HashMap::<&str, (&str, &str)>::new();
    let mut cursors = Vec::<&str>::new();
    for node in nodes.split('\n') {
        let id = &node[0..3];
        if id.as_bytes()[2] == b'A' {
            cursors.push(id);
        }
        let left = &node[7..10];
        let right = &node[12..15];
        map.insert(id, (left, right));
    }
    let mut least = time_for_cursor(cursors[0], &map, instructions);
    for cursor in &cursors {
        let steps = time_for_cursor(cursor, &map, instructions);
        println!("{} takes {} steps", cursor, steps);
        least = lcm(steps, least);
    }
    println!("Least Common Multiple is {} steps", least);
}
