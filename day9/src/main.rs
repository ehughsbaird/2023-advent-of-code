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

fn extrapolate_seq(seq : &Vec<i32>) -> i32 {
    let last = seq[seq.len() - 1];
    let mut little = Vec::<i32>::new();
    let mut all_zero = true;
    for i in 1..seq.len() {
        let val = seq[i] - seq[i - 1];
        little.push(val);
        if val != 0 {
            all_zero = false;
        }
    }
    return last + if all_zero { 0 } else { extrapolate_seq(&little) };
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("data.txt not found");
    let mut data: Vec<&str> = data.split('\n').collect();
    while data.last().unwrap().len() == 0 {
        data.pop();
    }
    let data : Vec<Vec<i32>> = data
        .into_iter()
        .map(|x: &str| -> Vec<i32> { x.split_whitespace().map(parse_or_panic).collect() })
        .collect();
    let mut sum = 0;
    for datum in &data {
        sum += extrapolate_seq(datum);
    }
    println!("Sum is {}", sum);
}
