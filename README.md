# wcount

wcount is a CLI word counter that counts words from given files and outputs the results as CSV.

## Building

wcount is written in [Rust](https://www.rust-lang.org/) and you will need a Rust installation to build it.
```
$ git clone https://github.com/Brogolem35/wcount
$ cd wcount
$ cargo build --release
$ ./target/release/wcount --version
```

## License

wcount is free software licensed under GPL-2.0-or-later license.

## Contributing

Feel free to open issues and pull requests. If you want to help with what I am currently working on, take a look at the [Stuff left to do](#stuff-left-to-do) section.

## Stuff left to do

- Recursive directory travelsal (with `--recursive` flag)
- Better error handling
- Better performance
- More and better tests
- Better code documentation