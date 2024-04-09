# `weather`
weather tool.

## Build
```
cargo build
```

## Usage

```
Usage: ./target/debug/weather [options] [city name[,country code]]
Eg: ./target/debug/weather -f Hanoi[VN]

Options:
    -p, --location-provider 0 to 3
                        Location provider
    -f, --full-info     Full weather information
    -s, --silent        Silent mode
    -v, --version       Print program version
    -h, --help          Print this help menu
```
