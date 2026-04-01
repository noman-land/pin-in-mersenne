# PIN In Mersenne

In [No Such Thing As A Fish](https://www.nosuchthingasafish.com/) episode 98 from January 2016, James Harkin says that the 49th Mersenne Prime, `2^74207281 - 1`, a `22,338,618` digit number, contains his 6 digit PIN 21 times.

This project aims to discover all the pins in the 49th Mersenne Prime that could possibly be James's PIN.

The [final list](pins.txt) is 83,080 out of a possible 1,000,000 6 digit PINs. One of these is his PIN.

You can find the prime txt here: https://www.mersenne.org/primes/

Direct link: https://www.mersenne.org/primes/digits/M74207281.zip

## To run

```bash
cargo run
```

## To build release binary and run that

5x speed improvement

```bash
cargo build --release
./target/release/pin-in-mersenne
```
