use std::{io, str::SplitWhitespace};

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    let mut iter_input: SplitWhitespace = buffer.split_whitespace();
    if let (Some(s), Some(t)) = (iter_input.next(), iter_input.next()) {
        let s_length: usize = s.len();
        for c in 1..s_length {
            for w in c..s_length {
                if s.as_bytes()
                    .chunks(w)
                    .filter_map(|chunk: &[u8]| chunk.get(c - 1))
                    .eq(t.as_bytes())
                {
                    println!("Yes");
                    return;
                }
            }
        }
        println!("No");
    }
}
