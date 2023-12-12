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

// Used for the trie. Binary strings of the numbers we can parse, and the digit representing their
// value as the terminator
const NUM_STRINGS: [&[u8]; 10] = [
    b"zero0", b"one1", b"two2", b"three3", b"four4", b"five5", b"six6", b"seven7", b"eight8",
    b"nine9",
];

// The trie!
// Key is the value of this node, next is the children of this node
struct Trie {
    key: u8,
    next: Vec<Trie>,
}

impl Trie {
    // Build the trie from the ascii strings representing numbers it can parse
    fn build(strings: &[&[u8]]) -> Trie {
        // The root node is always skipped, because not all strings will start with the same value
        let mut root = Trie {
            key: b'\0',
            next: vec![],
        };
        for string in strings {
            root.insert(&string);
        }
        return root;
    }
    // Insert a string into the trie
    fn insert(&mut self, string: &[u8]) {
        // When it's empty, we're done
        // As this function inserts into children, we're done when we have one value remaining in
        // the string.
        if string.len() == 1 {
            return;
        }
        // If we skip this node, consume no input
        let consumed = if self.key == b'\0' { 0 } else { 1 };
        // See if there already exist children to insert into
        for option in &mut self.next {
            if option.key == string[consumed] {
                option.insert(&string[consumed..]);
                return;
            }
        }
        // Otherwise, we'll have to add a child
        let new = self.next.len();
        self.next.push(Trie {
            key: string[consumed],
            next: vec![],
        });
        self.next[new].insert(&string[consumed..]);
    }
    fn parse(&self, checked: &[u8]) -> Option<i32> {
        // If we found a digit, then it parse to that digit value
        if char::from(self.key).is_ascii_digit() {
            return Some((self.key - b'0') as i32);
        }
        // Went through the whole string, didn't find our digit
        if checked.len() == 0 {
            return None;
        }
        // If we ended up in the wrong place, we failed to parse
        if self.key != b'\0' && self.key != checked[0] {
            return None;
        }
        // Find the child it will parse from
        for option in &self.next {
            let consume = if self.key == b'\0' { 0 } else { 1 };
            let ret = option.parse(&checked[consume..]);
            if ret.is_some() {
                return ret;
            }
        }
        // And if it parses into none of the children, we fail
        return None;
    }
    fn _print(&self, gen: i32) {
        let mut tabs = String::new();
        for _ in 0..gen {
            tabs.push('\t');
        }
        println!("{}'{}': [{}]", tabs, self.key as char, self.key as i32);
        for option in &self.next {
            option._print(gen + 1);
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
    // Our calibration sum
    let mut sum: i32 = 0;
    // The trie parser for our digits
    let num_parser = Trie::build(&NUM_STRINGS);
    // Parse every string
    for string in data {
        // Convert to bytes, because all our data is ASCII and utf8 is a PITA
        let seq = string.as_bytes();
        // The first digit in the string
        let mut first: Option<i32> = None;
        // The last digit in the string
        let mut last: Option<i32> = None;

        // walk forwards through the string and try to find the digits, either an ascii digit or the start of string we can parse to a digit
        for idx in 0..seq.len() {
            // Try and parse something at the current index
            let parsed = if char::from(seq[idx]).is_ascii_digit() {
                // Easy, it was digit
                Some((seq[idx] - b'0') as i32)
            } else {
                // If we parse successfully, we found it, otherwise this is None and we go forwards
                num_parser.parse(&seq[idx..])
            };
            // If it parses to anything, that's the last digit so far
            last = match parsed {
                Some(num) => Some(num),
                None => last,
            };
            // And if we haven't found first, it's also the first digit
            if first.is_none() {
                first = last;
            }
        }

        // Visual verification
        /*
        println!(
            "{}: {}, {}",
            string,
            first.unwrap_or(-1),
            last.unwrap_or(-1)
        );
        */
        sum += (first.unwrap() * 10) + last.unwrap();
    }
    println!("{} total", sum);
}
