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

struct Entry {
    dest: i64,
    source: i64,
    length: i64,
}

impl Entry {
    fn read_from_string(string: &str) -> Entry {
        let mut read = [0, 0, 0];
        let mut idx = 0;
        for num in string.split_whitespace().map(parse_or_panic::<i64>) {
            read[idx] = num;
            idx += 1;
        }
        return Entry {
            dest: read[0],
            source: read[1],
            length: read[2],
        };
    }
    fn print(&self) {
        println!(
            "dest: [{},{}], src: [{},{}]",
            self.dest,
            self.dest + self.length,
            self.source,
            self.source + self.length
        );
    }
}

fn map_to<T>(string: &str, mapping: fn(&str) -> T) -> Vec<T> {
    let mut singles: Vec<&str> = string
        .split(":\n")
        .skip(1)
        .next()
        .expect("")
        .split('\n')
        .collect();
    if singles[singles.len() - 1].len() == 0 {
        singles.pop();
    }
    return singles.into_iter().map(mapping).collect();
}

fn find_from_entry(id: i64, map: &Vec<Entry>) -> i64 {
    for entry in map {
        if !(entry.source..(entry.source + entry.length)).contains(&id) {
            continue;
        }
        return (id - entry.source) + entry.dest;
    }
    return id;
}

fn generate_loc(
        id: i64,
        soil_map: &Vec<Entry>,
        fert_map: &Vec<Entry>,
        water_map: &Vec<Entry>,
        light_map: &Vec<Entry>,
        temp_map: &Vec<Entry>,
        humid_map: &Vec<Entry>,
        loc_map: &Vec<Entry>,
) -> i64 {
        let soil = find_from_entry(id, soil_map);
        let fert = find_from_entry(soil, fert_map);
        let water = find_from_entry(fert, water_map);
        let light = find_from_entry(water, light_map);
        let temp = find_from_entry(light, temp_map);
        let humid = find_from_entry(temp, humid_map);
        let loc = find_from_entry(humid, loc_map);
        return loc;
    }

fn main() {
    // Read our calibration file and split it by line
    let file = fs::read_to_string("data.txt").expect("data.txt not found or busy");
    let sections: Vec<&str> = file.split("\n\n").collect();

    let seed_ranges: Vec<&str> = sections[0]
        .split(":")
        .skip(1)
        .next()
        .expect("")
        .split_whitespace()
        .collect();
    let seed_ranges: Vec<i64> = seed_ranges.into_iter().map(parse_or_panic::<i64>).collect();
    let mut seeds: Vec<i64> = vec![];
    let mut idx = 0;
    println!("{}", seed_ranges.len());
    while idx < seed_ranges.len() {
        println!("{}", idx);
        println!("Add: {}", seed_ranges[idx + 1]);
        for i in seed_ranges[idx]..seed_ranges[idx + 1] {
            seeds.push(i);
        }
        idx += 2;
    }
    println!("Done {}!", seeds.len());

    let seed_to_soil = map_to::<Entry>(sections[1], Entry::read_from_string);
    let soil_to_fert = map_to::<Entry>(sections[2], Entry::read_from_string);
    let fert_to_watr = map_to::<Entry>(sections[3], Entry::read_from_string);
    let watr_to_lght = map_to::<Entry>(sections[4], Entry::read_from_string);
    let lght_to_temp = map_to::<Entry>(sections[5], Entry::read_from_string);
    let temp_to_hmid = map_to::<Entry>(sections[6], Entry::read_from_string);
    let hmid_to_locs = map_to::<Entry>(sections[7], Entry::read_from_string);

    let seeds: Vec<i64> = seeds
        .into_iter()
        .map(|x: i64| {
            generate_loc(
                x,
                &seed_to_soil,
                &soil_to_fert,
                &fert_to_watr,
                &watr_to_lght,
                &lght_to_temp,
                &temp_to_hmid,
                &hmid_to_locs,
            )
        })
        .collect();
    println!(
        "Min is {}",
        seeds.iter().min().unwrap()
    );
}
