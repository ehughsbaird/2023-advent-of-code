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
use std::fs;
use std::collections::HashSet;

struct Scratchcard {
    id: i32,
    winning: HashSet<i32>,
    values: Vec<i32>
}

impl Scratchcard {
    fn from_string(string: &str) -> Scratchcard {
        // Format is Game %d: Grab*, so split off the game name and the grabs
        let split: Vec<&str> = string.split(':').collect();
        // So we can find the id
        let mut id: i32 = -1;
        // Walk through the id string until we find the first digit, then parse the rest
        for (idx, c) in split[0].char_indices() {
            if c.is_ascii_digit() {
                id = (split[0][idx..]).parse::<i32>().unwrap();
                break;
            }
        }
        // Now, we get the games
        let split: Vec<&str> = split[1].split('|').collect();
        // Map is pretty cool, huh
        let mut winning = HashSet::<i32>::new();
        for parsed in split[0].split_whitespace().map(str::parse::<i32>) {
            match parsed {
                Ok(int) => { winning.insert(int); },
                Err(_) => panic!("Couldn't parse")
            }
        }
        // Duplicate of above code, because I can't figure out how to handle the parse fail
        let mut values = Vec::<i32>::new();
        for parsed in split[1].split_whitespace().map(str::parse::<i32>) {
            match parsed {
                Ok(int) => { values.push(int); },
                Err(_) => panic!("Couldn't parse")
            }
        }
        return Scratchcard { id: id, winning: winning, values: values };
    }
    fn score(&self) -> i32 {
        let mut matched = 0;
        for val in &self.values {
            if self.winning.contains(&val) {
                matched += 1;
            }
        }
        if matched == 0 {
            return 0
        }
        return 1 << (matched - 1);
    }
}

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read_to_string("data.txt").expect("data.txt not found or busy");
    let mut data: Vec<&str> = file.split('\n').collect();
    // Get rid of empty string at the end
    while data.last().unwrap_or(&"a").len() == 0 {
        data.pop();
    }

    let mut sum = 0;
    for datum in data {
        let card = Scratchcard::from_string(datum);
        sum += card.score();
    }
    println!("Sum is {}", sum);
}
