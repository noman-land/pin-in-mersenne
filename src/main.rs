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

    // Fill a vector with zeros for counting PIN occurrences
    // The indices represent PINs (0-999999) and values are their counts
    let mut pins = vec![0u8; NUM_PINS];

    // Open the file that contains the prime number
    fs::read_to_string(PATH)
        // Error message if file can't be opened
        .expect("\nCouldn't read file\n\n")
        // Convert the string to a collection of bytes for processing
        .bytes()
        // Filter out the non-numbers (like newlines) to keep only digits
        .filter(|b| b.is_ascii_digit())
        // Collect the filtered bytes into a vector
        .collect::<Vec<u8>>()
        // Split up into chunks the length of the pin (6 digits each)
        .windows(PIN_LENGTH)
        // For each 6-digit chunk, increment its count in the pins vector
        .for_each(|window| {
            // Convert the 6-digit chunk into a string
            let pin = str::from_utf8(window)
                // Error message if UTF-8 conversion fails
                .expect("\nCouldn't convert to string\n\n")
                // Convert the string representation to a usize index
                .parse::<usize>()
                // Error message if number parsing fails
                .expect("\nCouldn't convert to usize\n\n");
            // Increment the count for this PIN since we've seen it once more
            pins[pin] += 1;
        });

    let matching_pin_indices: Vec<usize> = pins
        // Enumerate to get both index (PIN value) and count
        .iter()
        .enumerate()
        // Filter to keep only those with count == PIN_COUNT, map to just the index
        .filter_map(|(index, count)| {
            if *count == PIN_COUNT {
                Some(index) // Keep the PIN index
            } else {
                None // Skip this PIN
            }
        })
        // Collect the matching indices into a vector
        .collect();

    // Format each matching PIN as a zero-padded 6-digit string
    let formatted_pins: Vec<String> = matching_pin_indices
        // Convert each PIN index to a zero-padded string
        .iter()
        .map(|pin| format!("{:0width$}", pin, width = PIN_LENGTH))
        // Collect the formatted strings into a vector
        .collect();

    // Write the formatted PINs to a file in the current working directory
    let mut file = OpenOptions::new()
        // Create the file if it doesn't exist
        .create(true)
        // Open for writing
        .write(true)
        // Truncate the file to zero length if it exists
        .truncate(true)
        // Open "pins.txt" in the current directory
        .open("pins.txt")
        // Error message if file opening fails
        .expect("Failed to open pins.txt for writing");

    // Write each formatted PIN to the file followed by a newline
    for pin in &formatted_pins {
        writeln!(file, "{}", pin).expect("Failed to write pin to file");
    }

    // Print summary statistics
    println!(
        // Format: Count: [number] (completed in [time])
        "Count: {} (completed in {:.2?})",
        // Number of matching PINs found
        formatted_pins.len(),
        // Elapsed time since start
        start.elapsed()
    )
}
