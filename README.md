# npmcheater
Artificially bump the downloads of an npm package.

## Installing

### Install from Crates.io
Have Cargo Installed.
```bash
cargo install npmcheater
```

### Build it yourself
Have Cargo Installed.
```bash
git clone https://github.com/DaBigBlob/npmcheater.git
cd npmcheater
cargo build --release
cp target/release/npmcheater .
./npmcheater
```

## Usage
```txt
Usage: npmcheater [OPTIONS]

Options:
  -s, --show-logs <SHOW_LOGS>            [possible values: true, false]
  -p, --packages <PACKAGES>              
  -m, --max-sleep-mili <MAX_SLEEP_MILI>  
  -u, --user-agent <USER_AGENT>          
  -h, --help                             Print help
  -V, --version                          Print version
```

## Example
`user@host ~# npmcheater -p libsql -p base64-js -p discord.js -m 5000 -s true`
![demo output](https://raw.githubusercontent.com/DaBigBlob/npmcheater/main/demo.png)
