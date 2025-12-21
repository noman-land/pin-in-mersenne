# Pin In Mersenne

In No Such Thing As A Fish episode 98 from January 2016, James Harkin says that the 49th Mersenne Prime, `2^74207281 - 1`, the `22,338,618` digit number contains his 6 digit pin 21 times.

I am trying to find out how many 6 digit pins satisfy this criteria.

You can find the prime txt here: https://www.mersenne.org/primes/

Direct link: https://www.mersenne.org/primes/digits/M74207281.zip

## To run

```bash
cargo run
```

## To build release binary and run that

10x speed improvement

```bash
cargo build --release
./target/release/pin-in-mersenne
```
