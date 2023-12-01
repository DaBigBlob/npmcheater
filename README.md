# npmcheater
Artificially bump the downloads of an npm package.

## Installing

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
