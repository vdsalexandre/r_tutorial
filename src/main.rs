#![allow(unused)]

use std::{io, u128, usize};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    print_sizes();

    using_data_types();

    say_hello();

    math();

    random_values();
}

fn say_hello() {
    println!("What is your name ?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello {} ! {}", name.trim_end(), greeting);
}

fn using_data_types() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "34";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn print_sizes() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn math() {
    let num_1: f32 = 1.11111111111111111111111111;
    println!("f32: {}", num_1 + 0.11111111111111111111111111);
    let num_2: f64 = 1.11111111111111111111111111;
    println!("f64: {}", num_2 + 0.11111111111111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 - 4 = {}", num_3 - num_4);
}

fn random_values() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("touch my random: {}", random_num);
}