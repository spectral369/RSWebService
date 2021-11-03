#![allow(non_snake_case)]
use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub async fn test_handler(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

#[derive(Deserialize)]
struct SendRequest {
    name: String,
    active: bool,
}

pub async fn send_handler(mut ctx: Context) -> Response {
    let body: SendRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.active
        )
        .into(),
    )
}

pub async fn param_handler(ctx: Context) -> String {
    let param = match ctx.params.find("some_param") {
        Some(v) => v,
        None => "empty",
    };

    let days;
    if !check_if_exists(param) {
        days = writefl(param);
    } else {
        days = read_line(param);
    }
    format!("param called, param was: {}", days)
}

fn read_line(src_str: &str) -> i32 {
    let contents = fs::read_to_string("foo.txt").expect("err");
    let mut return_days: i32 = 30;
    for line in contents.lines() {
        if line.contains(src_str) {
            let mut days_remaining: String = line.chars().skip(line.len() - 2).take(2).collect();
            let mut my_int;
            if !days_remaining.contains(">") {
                my_int = days_remaining.parse::<i32>().unwrap();
            } else {
                days_remaining = line.chars().skip(line.len() - 1).take(1).collect();
                my_int = days_remaining.parse::<i32>().unwrap();
            }
            my_int -= 1;
            println!(" {} ", days_remaining);
            return_days = my_int;
        }
    }
    return return_days;
}

fn writefl(str: &str) -> i32 {
    let mut days = 30;
    let data: String = str.to_owned() + "->31\n";
    if Path::new("foo.txt").exists() {
        let mut file1 = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("foo.txt")
            .unwrap();
        if !check_if_exists(&str) {
            file1.write_all(data.as_bytes()).expect("err");
        } else {
            days = seek_and_changefl3(&str, "foo.txt");
        }
    } else {
        let mut file1 = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("foo.txt")
            .unwrap();
        file1.write_all(data.as_bytes()).expect("err");
    }

    return days;
}

fn check_if_exists(search_str: &str) -> bool {
    let contents = fs::read_to_string("foo.txt").expect("Unable to open file");
    for line in contents.lines() {
        if line.contains(search_str) {
            return true;
        }
    }
    false
}
#[warn(dead_code)]
fn readfl() -> String {
    let file = File::open("foo.txt").expect("Unable to open file");
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to open file");
    return contents;
}

fn seek_and_changefl3(searched_item: &str, file_name: &str) -> i32 {
    // std::io::Result<()>{
    let contents = fs::read_to_string(file_name).expect("err");
    let mut new: String = String::new();
    let mut buff: String = String::new();
    let mut return_days: i32 = 30;

    for line in contents.lines() {
        if line.contains(searched_item) {
            let mut days_remaining: String = line.chars().skip(line.len() - 2).take(2).collect();
            let mut my_int;
            let mut substring: String;
            if !days_remaining.contains(">") {
                my_int = days_remaining.parse::<i32>().unwrap();
                substring = line.chars().take(line.len() - 2).collect();
            } else {
                days_remaining = line.chars().skip(line.len() - 1).take(1).collect();
                println!(" {} ", days_remaining);
                my_int = days_remaining.parse::<i32>().unwrap();
                substring = line.chars().take(line.len() - 1).collect();
            }
            my_int -= 1;
            println!(" {} ", days_remaining);
            substring.push_str(&my_int.to_string());

            buff.push_str(substring.as_str());
            buff.push_str("\n");
            return_days = my_int;
        } else {
            buff.push_str(line);
            buff.push_str("\n")
        }
    }
    new.push_str(&buff);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .expect("err");
    file.write(new.as_bytes()).expect("err");
    return return_days;
}
