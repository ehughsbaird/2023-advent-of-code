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

fn parse_or_panic<T: std::str::FromStr>(string: &str) -> T {
    match string.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("Couldn't parse '{}'", string),
    }
}

fn ways_to_win_for(time: i64, dist: i64) -> i64 {
    let mut ways = 0;
    for i in 0..time {
        let travels = i * (time - i);
        //println!("{} travels {}", i, travels);
        if travels > dist {
            ways += 1;
        }
    }
    return ways;
}

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read_to_string("data.txt").expect("data.txt not found or busy");
    let data: Vec<&str> = file.split('\n').collect();
    let times: i64 = parse_or_panic(
        &data[0]
            .split(':')
            .skip(1)
            .next()
            .expect("")
            .split_whitespace()
            .collect::<String>(),
    );
    let dist: i64 = parse_or_panic(
        &data[1]
            .split(':')
            .skip(1)
            .next()
            .expect("")
            .split_whitespace()
            .collect::<String>(),
    );
    println!("{}", ways_to_win_for(7, 9));
    let mut product = 1;
    let wins = ways_to_win_for(times, dist);
    println!("Wins are {}", wins);
}
