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
use std::cmp;

struct Game {
    id: i32,
    grabs: Vec<Grab>,
}

struct Grab {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn from_string(string: &str) -> Game {
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
        let split: Vec<&str> = split[1].split(';').collect();
        // Map is pretty cool, huh
        let grabs: Vec<Grab> = split.iter().map(Grab::from_string).collect();
        return Game {
            id: id,
            grabs: grabs,
        };
    }
    // Hack print function for visual debugging
    fn _print(&self) {
        print!("Game {}:", self.id);
        for grab in &self.grabs {
            grab._print();
            print!(";");
        }
        println!("");
    }
}

impl Grab {
    // We need &&str because of the map thing above. I don't quite understand it, tbh
    fn from_string(string: &&str) -> Grab {
        // The red/green/blue components are split by commas
        // '%d {red,green,blue},'
        let split: Vec<&str> = string.split(',').collect();
        let mut ret = Grab {
            red: 0,
            green: 0,
            blue: 0,
        };
        // Walk through to find the number, then parse it and look at the color
        for color in split {
            // .0 is where the number starts, .1 is where it ends
            // .0 is optional because we only want to set it once
            let mut num_range = (None, 0);
            // The parsed number
            let mut num: i32 = -1;
            for (idx, c) in color.char_indices() {
                // If it's a digit, it's part of the number parsing
                if c.is_ascii_digit() {
                    // The end updates every digit
                    num_range.1 = idx;
                    // The start updates only once
                    num_range.0 = if num_range.0.is_none() {
                        Some(idx)
                    } else {
                        num_range.0
                    };
                }
                // is_some() checks to ensure this isn't leading whitespace
                // But if it's not, there's a single space between the number and the color
                if c.is_whitespace() && num_range.0.is_some() {
                    num = color[num_range.0.unwrap()..=num_range.1]
                        .parse::<i32>()
                        .unwrap();
                }
                // We only need the first character to check what color it is
                match c {
                    'r' => ret.red = num,
                    'g' => ret.green = num,
                    'b' => ret.blue = num,
                    // If we're at a whitespace but haven't found the number, etc
                    _ => continue,
                }
                // And once we've found that, move on to the next color
                break;
            }
        }
        return ret;
    }
    // Hacky print for visual debugging
    fn _print(&self) {
        if self.red != 0 {
            print!(" {} red,", self.red);
        }
        if self.green != 0 {
            print!(" {} green,", self.green);
        }
        if self.blue != 0 {
            print!(" {} blue", self.blue);
        }
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
    // Sum of ids that are valid
    let mut sum = 0;

    for datum in data {
        // Read the game in
        let game = Game::from_string(&datum);
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        // Find the max of each color from each grab
        for grab in &game.grabs {
            red = cmp::max(grab.red, red);
            green = cmp::max(grab.green, green);
            blue = cmp::max(grab.blue, blue);
        }
        // Add the power to the sum
        sum += red * green * blue;
    }

    println!("Sum of powers: {}", sum);
}
