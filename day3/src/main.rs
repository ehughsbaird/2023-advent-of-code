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

// Read a number that has a digit at provided point
// point.0 is index in data[point.1]
// Dirty is points that have already been visited
fn parse_point(
    point: (i32, i32),
    data: &Vec<&[u8]>,
    dirty: &mut HashSet<(i32, i32)>,
) -> Option<i32> {
    if dirty.contains(&point) {
        return None;
    }
    let mut start: i32 = point.0;
    let mut end: i32 = point.0;
    let line = data[point.1 as usize];

    while start >= 0 {
        if !char::from(line[start as usize]).is_ascii_digit() {
            break;
        }
        dirty.insert((start, point.1));
        start -= 1;
    }
    // We go to the next one, but we'll always have failed to read, so walk back to the last valid
    // digit
    start += 1;

    while (end as usize) < line.len() {
        if !char::from(line[end as usize]).is_ascii_digit() {
            break;
        }
        dirty.insert((end, point.1));
        end += 1;
    }
    // We go to the next one, but we'll always have failed to read, so walk back to the last valid
    // digit
    end -= 1;

    let num_str = std::str::from_utf8(&line[(start as usize)..=(end as usize)]).unwrap();
    return Some(num_str.parse::<i32>().unwrap());
}

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read("data.txt").expect("data.txt not found or busy");
    let mut data: Vec<&[u8]> = file.split(|c| -> bool { *c == b'\n' }).collect();
    // Get rid of empty string at the end
    while data.last().unwrap().len() == 0 {
        data.pop();
    }

    // Points which we will try to parse, organized by gears they are around
    let mut parseable: Vec<HashSet<(i32, i32)>> = vec![];

    // Points adjacent to the symbol point
    let mut tuples: Vec<(i32, i32)> = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            tuples.push((x, y));
        }
    }

    // Look through the whole set for gears
    for (y, line) in data.iter().enumerate() {
        for (x, sym) in line.iter().enumerate() {
            if *sym != b'*' {
                continue;
            }
            let mut candidates = HashSet::<(i32, i32)>::new();
            // When we find a digit adjacent to a gear, mark to attempt to parse the number there
            for relative in &tuples {
                let x = (x as i32 + relative.0) as usize;
                let y = (y as i32 + relative.1) as usize;
                if char::from(data[y][x]).is_ascii_digit() {
                    candidates.insert((x as i32, y as i32));
                }
            }
            // Add the numbers around this gear to the list
            parseable.push(candidates);
        }
    }

    // Sum of all values
    let mut acc = 0;

    // Points we've already read, and thus shouldn't read again
    let mut dirty = HashSet::<(i32, i32)>::new();
    for points in parseable {
        let mut around = Vec::<i32>::new();
        for point in points {
            // Points will fail to parse if they have already been read
            match parse_point(point, &data, &mut dirty) {
                Some(num) => around.push(num),
                None => continue,
            }
        }
        // Not actually a gear
        if around.len() != 2 {
            continue;
        }
        acc += around[0] * around[1];
    }
    println!("Total is {}", acc);
}
