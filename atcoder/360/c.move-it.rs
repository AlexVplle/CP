use std::{
    collections::HashMap,
    io::{self, BufRead, StdinLock},
    str::FromStr,
};

pub fn get_number<T, R>(reader: &mut R) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    R: BufRead,
{
    let mut buffer: String = String::new();
    let _ = reader.read_line(&mut buffer);
    buffer.trim().parse().unwrap()
}

pub fn get_vec_number<T, R>(reader: &mut R) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    R: BufRead,
{
    let mut buffer: String = String::new();
    let _ = reader.read_line(&mut buffer);
    buffer
        .split_whitespace()
        .map(|string_number: &str| string_number.parse::<T>().unwrap())
        .collect()
}

fn main() {
    let stdin: &mut StdinLock<'static> = &mut io::stdin().lock();
    let n: usize = get_number(stdin);
    let a: Vec<u32> = get_vec_number(stdin);
    let w: Vec<u32> = get_vec_number(stdin);
    let mut result: u32 = 0;
    let mut hashmap: HashMap<u32, u32> = HashMap::with_capacity(n);
    a.iter().zip(w.iter()).for_each(|(&group, &weight)| {
        hashmap.entry(group).and_modify(|old_weight| {
            result += (*old_weight).min(weight);
            *old_weight = (*old_weight).max(weight);
        }).or_insert(weight);
    });
    println!("{}", result);
}
