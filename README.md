[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/shuffle/blob/master/LICENSE-MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
# 🔑 Shuffle

A simple password generator made with Rust.

## Usage examples

 Shuffle always use a minimum of one char from each groups. This is why the generated password can not have a length lower than the number of selected groups.
 With no argument Shuffle generate a password with 20 chars. It use uppercase letters, lowercase letters and digits:
```
# similar to ./shuffle -uld -L 20
./shuffle
F6EXLSY9CLADiy5bslbZ
```

Generate a password with 30 chars with upper,lower,digits and symbols:
```
./shuffle -ulds -L 30
0@y0RM9_O2@_GldmTng5j#.B6Tl9RK
```

Generate a password with 30 chars with only digits and exclude 0,1,2,3 and 4:
```
./shuffle -d -L 30 --exclude 012345 
699796999789668988897686796789
```

Display full help with -h flag:

```
./shuffle -h

🔑 Random password generator

USAGE:
    shuffle [OPTIONS]

OPTIONS:
    -u, --uppercase           Use UPPERCASE letters [A-Z]
    -l, --lowercase           Use lowercase letters [a-z]
    -d, --digits              Use digits [0-9]
    -s, --symbols             Use special symbols [*&^%$#@!~]
    -L, --length <NUMBER>     Sets the required length [default: 20]
        --output <OUTPUT>     Output in a txt file
        --exclude <EXCLUDE>   Exclude char
    -h, --help                Print help information
    -V, --version             Print version information

If you do not specify any of the [--uppercase, --lowercase, --digits, --symbols] flags,
then uppercase, lowercase letters and digits will be used.
```
## Build Shuffle
Clone this repo, go in shuffle folder and build it with cargo :
```
git clone https://github.com/Antidote1911/shuffle
cd shuffle
cargo build --release

```
