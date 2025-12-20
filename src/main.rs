use std::fs;
use std::str;

static PIN_LENGTH: usize = 6;
static PIN_COUNT: u8 = 21;
static PATH: &str = "data/prime.txt";

fn main() {
    let mut pins = vec![0u8; 10_usize.pow(PIN_LENGTH as u32)];

    fs::read_to_string(PATH)
        .expect("\nCouldn't read file\n\n")
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .collect::<Vec<u8>>()
        .windows(PIN_LENGTH)
        .for_each(|window| {
            let pin = str::from_utf8(window)
                .expect("\nCouldn't convert to string\n\n")
                .parse::<usize>()
                .expect("\nCouldn't convert to usize\n\n");
            pins[pin] += 1;
        });

    println!(
        "Count: {}",
        pins.iter().filter(|count| **count == PIN_COUNT).count()
    )
}
