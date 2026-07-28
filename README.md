
# numeral-system-rs

**_Converter for Various Numeral Systems_**

converting numbers between different bases, such as binary, octal, decimal, hexadecimal, and custom bases, with a simple and efficient API.

supports from base-2(binary) upto base-32.

### CLI Usage

#### Simple

```bash
cli @ host $ numeral-system-rs 8 77
status => PASS
input  => value = "8"   base = "77"
output => value = "10"  base = "63"
cli @ host $  
```

#### With Output Base

```bash
cli @ host $ numeral-system-rs 8 77 -b 12
status => PASS
input  => value = "8"   base = "77"
output => value = "12"  base = "53"
cli @ host $   
```

#### With Output Format

```bash
cli @ host $ numeral-system-rs 8 77 -b 24 -t clean
2F
cli @ host $  
```

#### Help

```bash
cli @ host $ numeral-system-rs --help              
Usage: numeral-system-rs [OPTIONS] <INPUT_BASE> <INPUT_VALUE>

Arguments:
  <INPUT_BASE>   base of the number
  <INPUT_VALUE>  number value

Options:
  -b <OUTPUT_BASE>  convert to base [default: 10]
  -t <OUTPUT_TYPE>  type of the output [default: verbose] [possible values: json, verbose, clean]
  -h, --help        Print help
  -V, --version     Print version
cli @ host $  
```

### Inspiration

- Rust Coding
- Rust Testing
- Rust Code Coverage
- Rust Docs
- Rust Library
- Python Library with Rust

### TODOs

- Update Docs
- Add Coverage
- Create Rust Lib Release
- Create Python Lib Release (using rust)
- Add Rest Based Usage (Dockerized)
