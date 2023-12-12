use std::cmp;
use std::collections::HashMap;
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

// Thanks, https://users.rust-lang.org/t/how-to-sort-enum-variants/52291/2
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Hand {
    HighCard(i32),
    OnePair(i32),
    TwoPair(i32),
    ThreeOfAKind(i32),
    FullHouse(i32),
    FourOfAKind(i32),
    FiveOfAKind(i32),
}

fn promote(hand: Hand, jokers: i32) -> Hand {
    if jokers == 0 {
        return hand;
    }
    match hand {
        // Lucky!
        Hand::FiveOfAKind(_) => hand,
        // Can't be zero, must be either 1 or 4
        Hand::FourOfAKind(val) => Hand::FiveOfAKind(val),
        // Can't be zero, must be 2 or 3
        Hand::FullHouse(val) => Hand::FiveOfAKind(val),
        Hand::ThreeOfAKind(val) => Hand::FourOfAKind(val),
        Hand::TwoPair(val) => match jokers {
            1 => Hand::FullHouse(val),
            2 => Hand::FourOfAKind(val),
            _ => panic!("How do you have {} jokers", jokers),
        },
        Hand::OnePair(val) => Hand::ThreeOfAKind(val),
        Hand::HighCard(val) => Hand::OnePair(val),
    }
}

fn parse_to_hand(string: &str) -> Hand {
    let mut value: i32 = 0;
    let mut card_map = HashMap::<char, i32>::new();
    for c in string.chars() {
        value *= 14;
        value += match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!(),
        };
        if card_map.contains_key(&c) {
            card_map.insert(c, card_map.get(&c).unwrap() + 1);
        } else {
            card_map.insert(c, 1);
        }
    }
    if card_map.len() == 1 {
        return promote(Hand::FiveOfAKind(value), *card_map.get(&'J').unwrap_or(&0));
    }
    if card_map.len() == 2 {
        for (k, v) in &card_map {
            if *v == 4 || *v == 1 {
                return promote(Hand::FourOfAKind(value), *card_map.get(&'J').unwrap_or(&0));
            }
            if *v == 3 || *v == 2 {
                return promote(Hand::FullHouse(value), *card_map.get(&'J').unwrap_or(&0));
            }
        }
    }
    let mut pairs = 0;
    for (k, v) in &card_map {
        if *v == 3 {
            return promote(Hand::ThreeOfAKind(value), *card_map.get(&'J').unwrap_or(&0));
        }
        if *v == 2 {
            pairs += 1;
        }
    }
    if pairs == 2 {
        return promote(Hand::TwoPair(value), *card_map.get(&'J').unwrap_or(&0));
    }
    if pairs == 1 {
        return promote(Hand::OnePair(value), *card_map.get(&'J').unwrap_or(&0));
    }
    return promote(Hand::HighCard(value), *card_map.get(&'J').unwrap_or(&0));
}

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read_to_string("data.txt").expect("data.txt not found or busy");
    let mut data: Vec<&str> = file.split('\n').collect();
    // Get rid of empty string at the end
    while data.last().unwrap_or(&"a").len() == 0 {
        data.pop();
    }
    let mut hands = Vec::<(Hand, i32)>::new();
    for datum in &data {
        let bits: Vec<&str> = datum.split_whitespace().collect();
        hands.push((parse_to_hand(bits[0]), bits[1].parse::<i32>().unwrap()));
    }
    hands.sort();
    let mut sum = 0;
    for i in 0..hands.len() {
        println!("{:?}: {}", hands[i].0, (i + 1) as i32 * hands[i].1);
        sum += (i + 1) as i32 * hands[i].1;
    }
    println!("{}", sum);
}
