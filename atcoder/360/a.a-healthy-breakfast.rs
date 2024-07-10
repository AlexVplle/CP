use std::io;

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    if let (Some(r_pos), Some(m_pos)) = (buffer.find('R'), buffer.find('M')) {
        if r_pos < m_pos {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}
