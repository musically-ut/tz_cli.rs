# tz_cli.rs

Times from different zones on your CLI.

## Installation 

```bash
cargo install --git https://github.com/musically-ut/tz_cli.rs.git
```

This will install a binary called `tz`.

## Usage

Create the configuration file `~/.tz.rc` and put time-zones in it one per line:

```
Asia/Kolkata
America/New_York
```

Then run `tz` to see the local time in all time-zones:

```
$ tz
Local time      	= 2017-03-12 17:03 +01:00
Asia/Kolkata    	= 2017-03-12 21:33 IST
America/New_York	= 2017-03-12 12:03 EDT
```
