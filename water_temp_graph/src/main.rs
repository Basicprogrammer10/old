#![allow(warnings)]
use attohttpc;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use std::num::ParseFloatError;
use std::{collections::HashMap, num::ParseIntError};

#[macro_use]
mod color;
mod arg_parse;
use color::Color;

struct DataPoint {
    time: u32,
    temp: f32,
}

struct TempData {
    values: u32,
    min: f32,
    max: f32,
    /// Time, Temp
    data: Vec<DataPoint>,
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let host: &str = arg_parse::get_arg_value(&args, "--host").unwrap_or("water.connorcode.com");
    let per: usize = arg_parse::get_arg_value(&args, "--per")
        .unwrap_or("15")
        .parse()
        .unwrap_or(100);

    color_print!(Color::Green, "[*] Starting :P");

    // Get temperature history from api
    color_print!(Color::Cyan, "[*] Getting Data");
    let res = attohttpc::get(format!("https://{}/data/download.csv", host))
        .send()
        .unwrap();

    if !res.is_success() {
        color_print!(Color::Red, "[-] Error Getting data from api :/");
        panic!("Error Getting data from api")
    }
    let res_data = res.text().unwrap();

    // Parse data to a Hash Map
    color_print!(Color::Cyan, "[*] Parsing Data");
    let raw_data: Vec<&str> = res_data.split('\n').collect();
    let mut data: Vec<DataPoint> = Vec::new();
    let mut last_update: u32 = 0;
    let mut min: f32 = f32::MAX;
    let mut max: f32 = f32::MIN;

    // Example Line: 1629049085,8/15/2021 5:38:05 PM,77.9
    for i in raw_data.iter().skip(1).step_by(per) {
        let line: Vec<&str> = i.split(',').collect();
        if line.len() < 3 {
            continue;
        }

        let time: Result<u32, ParseIntError> = line[0].parse();
        let temp: Result<f32, ParseFloatError> = line[2].parse();
        if time.is_err() || temp.is_err() {
            continue;
        }
        last_update = *time.as_ref().unwrap() as u32;
        min = f32::min(min, *temp.as_ref().unwrap());
        max = f32::max(max, *temp.as_ref().unwrap());
        data.push(DataPoint::new(time.unwrap(), temp.unwrap()));
    }
    let data = TempData::new(0, 0.0, 100.0, data);

    // Graphing...
    color_print!(Color::Cyan, "[*] Graphing!");

    let mut working: Vec<f32> = Vec::new();
    for i in &data.data {
        working.push(i.temp);
    }

    // Draw Graph
    for i in working.iter() {
        println!("{}", draw_h_line(*i));
    }

    print_stats(last_update, raw_data.len(), working.len(), min, max);
}

fn draw_h_line(len: f32) -> String {
    // All Chars: █▉▊▋▌▍▎
    let mut working: String = "".to_string();
    working += &"█".repeat(len as usize);
    working += match len - (len as u32) as f32 {
        0.90..=1.00 => "█",
        0.75..=0.90 => "▉",
        0.60..=0.75 => "▊",
        0.45..=0.60 => "▋",
        0.30..=0.45 => "▌",
        0.15..=0.30 => "▍",
        0.00..=0.15 => "▎",
        _ => "",
    };

    working.to_string()
}

/// Fancy Stats Box Thing :P
fn print_stats(update: u32, total_points: usize, points: usize, min: f32, max: f32) {
    let naive_datetime = NaiveDateTime::from_timestamp(update as i64, 0);
    let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃            Water Temp Graph            ┃");
    println!("┃            ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔            ┃");
    println!("┃ Last Update ─ {}  ┃", datetime_again);
    println!(
        "┃ Data Points ─ {} ({}){}  ┃",
        points,
        total_points,
        " ".repeat(20 - points.to_string().len() - total_points.to_string().len())
    );
    println!(
        "┃         Min ─ {}°F{}  ┃",
        min,
        " ".repeat(21 - min.to_string().len())
    );
    println!(
        "┃         Max ─ {}°F{}  ┃",
        max,
        " ".repeat(21 - max.to_string().len())
    );
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}

impl TempData {
    fn new(values: u32, min: f32, max: f32, data: Vec<DataPoint>) -> TempData {
        TempData {
            values,
            min,
            max,
            data,
        }
    }
}

impl DataPoint {
    fn new(time: u32, temp: f32) -> DataPoint {
        DataPoint { time, temp }
    }
}
