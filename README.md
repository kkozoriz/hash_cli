# hash-cli

Hash Finder is a command-line application developed in Rust that iterates through integers starting from 1, computes
their SHA-256 hash, and prints the integer and its corresponding hash if the hash ends with a specified number of
zeroes. 

The number of trailing zeroes (N) and the number of hashes to find (F) are provided by the user as command-line
arguments.

## Usage

You can use it this way:

```shell
git clone https://github.com/kkozoriz/hash_cli.git
```

```shell
cd /hash_cli
```

```shell
cargo build --release
```

```shell
cd /target/release
```

```shell
./hash_cli [OPTIONS]
```

For more options and usage details, you can run:

```shell
./hash_cli --help
```

## Tests

```cargo test```

## Example

```shell
./hash_cli -N 3 -F 6
```

```
4163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"
11848, "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"
12843, "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000"
13467, "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000"
20215, "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000"
28892, "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000"
```