use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::str;
use std::time::Instant;

// The path of the file we are searching through
static PATH: &str = "data/M74207281.txt";
// The length of the pin we are looking for
static PIN_LENGTH: usize = 6;
// Number of times the pin appears in the file
static PIN_COUNT: u8 = 21;
// Calculate number of possibilities based on pin length: 10^6 = 1,000,000
static NUM_PINS: usize = 10usize.pow(PIN_LENGTH as u32);

fn main() {
    // Start measuring run time
    let start = Instant::now();

    // Fill a vector with zeros
    // The indices are the pins and the values are the counts per pin
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

    // Collect matching pin indices
    let matching_pin_indices: Vec<usize> = pins
        .iter()
        .enumerate()
        .filter_map(|(index, count)| {
            if *count == PIN_COUNT {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    // Format pins as zero-padded strings
    let formatted_pins: Vec<String> = matching_pin_indices
        .iter()
        .map(|pin| format!("{:0width$}", pin, width = PIN_LENGTH))
        .collect();

    // Write to file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("pins.txt")
        .expect("Failed to open pins.txt for writing");

    for pin in &formatted_pins {
        writeln!(file, "{}", pin).expect("Failed to write pin to file");
    }

    println!(
        // Format the count and the time to 2 decimal places
        "Count: {} (completed in {:.2?})",
        // Count of matching pins
        formatted_pins.len(),
        // Output elapsed time
        start.elapsed()
    )
}
