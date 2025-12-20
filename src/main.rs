use std::fs;
use std::str;

// The path of the file we are searching through
static PATH: &str = "data/prime.txt";
// The length of the pin we are looking for
static PIN_LENGTH: usize = 6;
// Number of times the pin appears in the file
static PIN_COUNT: u8 = 21;
// Calculate number of possibilities based on pin length: 10^6 = 1,000,000
static NUM_PINS: usize = 10usize.pow(PIN_LENGTH as u32);

fn main() {
    // Fill a vector with zeros
    let mut pins = vec![0u8; NUM_PINS];

    // Open the file that contains the prime number
    fs::read_to_string(PATH)
        // Error message if file can't be opened
        .expect("\nCouldn't read file\n\n")
        // Convert the string to a collection of bytes
        .bytes()
        // Filter out the non-numbers (like newlines)
        .filter(|b| b.is_ascii_digit())
        // Collect into a vector
        .collect::<Vec<u8>>()
        // Split up into chunks the length of the pin
        .windows(PIN_LENGTH)
        // For each chunk value, keep a tally of how many times it appears
        .for_each(|window| {
            // Convert the chunk into a string
            let pin = str::from_utf8(window)
                // Error message if it can't convert
                .expect("\nCouldn't convert to string\n\n")
                // Convert the string into a number
                .parse::<usize>()
                // Error message if it can't convert
                .expect("\nCouldn't convert to usize\n\n");
            // Add another tally indicating we've seen this pin another time
            pins[pin] += 1;
        });

    println!(
        "Count: {}",
        // Filter for all the pins that appear PIN_COUNT times
        pins.iter().filter(|count| **count == PIN_COUNT).count()
    )
}
