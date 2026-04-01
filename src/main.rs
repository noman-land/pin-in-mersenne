use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::str;
use std::time::Instant;

// The path of the file we are searching through (contains the Mersenne prime)
static PATH: &str = "data/M74207281.txt";
// The length of the PIN we are looking for (6 digits)
static PIN_LENGTH: usize = 6;
// Number of times the PIN must appear in the file to be a match
static PIN_COUNT: u8 = 21;
// Calculate number of possibilities based on PIN length: 10^6 = 1,000,000
static NUM_PINS: usize = 10usize.pow(PIN_LENGTH as u32);

fn main() {
    // Start the timer to track how long the program takes
    let start = Instant::now();

    // Open the output file where we'll write matching PINs
    let mut file = OpenOptions::new()
        // Create the file if it doesn't exist
        .create(true)
        // Open for writing (not reading)
        .write(true)
        // Truncate existing file to empty before writing
        .truncate(true)
        // Open or create our results file in the current directory
        .open("pins.txt")
        // Panic if file can't be opened
        .expect("Failed to open pins.txt for writing");

    // Main processing chain: read file → count all PINs → filter matches → write to file → count results
    let count = fs::read_to_string(PATH)
        // Panic if the file can't be read
        .expect("\nCouldn't read file\n\n")
        // Convert the entire file contents into an iterator over bytes
        .bytes()
        // Keep only digit characters (0-9), discard newlines and other characters
        .filter(|b| b.is_ascii_digit())
        // Collect all the digit bytes into a Vec<u8> so we can use .windows()
        .collect::<Vec<u8>>()
        // Create a sliding window of 6 digits at a time (each window overlaps by 5 digits)
        .windows(PIN_LENGTH)
        // Convert each 6-digit window into a usize number (the PIN value/index)
        .map(|window| {
            // Convert the byte slice to a UTF-8 string (should always succeed for digits)
            str::from_utf8(window)
                // Panic if conversion fails (should never happen for digit-only data)
                .expect("\nCouldn't convert to string\n\n")
                // Parse the string as a base-10 integer to use as array index
                .parse::<usize>()
                // Panic if parsing fails (should never happen for valid digit strings)
                .expect("\nCouldn't convert to usize\n\n")
        })
        // Fold: start with a vector of 1,000,000 zeros, increment position for each PIN seen
        .fold(vec![0u8; NUM_PINS], |mut acc, pin| {
            // Increment the count at position [pin] since we've found this PIN once more
            acc[pin] += 1;
            // Return the updated accumulator for the next iteration
            acc
        })
        // Now we have a Vec<u8> where each index (0-999999) holds the count of that PIN
        // Iterate over each element in the counts vector
        .iter()
        // Enumerate gives us both the index (which represents the PIN number) and its count
        .enumerate()
        // Keep only PINs where the count equals our target (21)
        .filter(|(_index, count)| **count == PIN_COUNT)
        // Fold: for each matching PIN, write it to the file and count how many we found
        .fold(0, |acc, (index, _)| {
            // Format the PIN index as a zero-padded 6-digit string (e.g., 42 → "000042")
            let pin = format!("{:0width$}", index, width = PIN_LENGTH);
            // Write the formatted PIN to the output file followed by a newline
            writeln!(file, "{}", pin).ok();
            // Return the count incremented by 1 for this match
            acc + 1
        });

    // Print the final result: how many matching PINs found and total time elapsed
    println!(
        // Format: "Count: [number] (completed in [time])"
        "Count: {} (completed in {:.2?})",
        // The number of PINs that appeared exactly 21 times
        count,
        // The elapsed time since we started the timer
        start.elapsed()
    )
}
