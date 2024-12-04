#![allow(dead_code)]
use std::{any::type_name, str::FromStr};

pub fn v_grid_by_whitespace<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
{
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| match n.parse::<T>() {
                    Ok(v) => v,
                    Err(_) => panic!("failed to parse \"{n}\" as {}", type_name::<T>()),
                })
                .collect()
        })
        .collect()
}

pub fn char_grid(input: &str) -> Vec<Vec<char>> {
    v_grid_no_whitespace(input)
}

pub fn v_grid_no_whitespace<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
{
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| match n.to_string().parse::<T>() {
                    Ok(v) => v,
                    Err(_) => panic!("failed to parse \"{n}\" as {}", type_name::<T>()),
                })
                .collect()
        })
        .collect()
}
