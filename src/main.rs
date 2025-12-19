use std::fs;
use std::str::from_utf8;

const PIN_LENGTH: usize = 6;
const PIN_COUNT: u8 = 21;

fn main() {
    let mut pins = vec![0u8; 1_000_000];

    fs::read_to_string("data/prime.txt")
        .expect("Can't read file")
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .collect::<Vec<u8>>()
        .windows(PIN_LENGTH)
        .enumerate()
        .for_each(|(_, window)| {
            let pin = from_utf8(window)
                .expect("Couldn't convert to string")
                .parse::<usize>()
                .expect("Couldn't convert to usize");
            pins[pin] += 1;
        });

    println!(
        "Count: {}",
        pins.iter().filter(|count| **count == PIN_COUNT).count()
    )
}
