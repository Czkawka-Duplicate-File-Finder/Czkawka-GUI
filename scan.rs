fn min_size_bytes() -> u64 {
    1024
}

fn main() {
    println!("skip files smaller than {} bytes", min_size_bytes());
}
