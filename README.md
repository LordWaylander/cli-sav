# Cli-sav
## A tool to save a disk, a folder or a file

### WARNNG
Only tested with Linux (Debian Bookworm), will probably not work with Windows & MacOS (for the moment)

### installation
```
$ git clone https://github.com/LordWaylander/cli-sav.git
$ cd cli-sav/
$ cargo build --release
```

# Commands
```
help-> show help
scan -> scan all disk mounted on the system
save <option> -> save a disk, folder or file
restore <option> -> restore a disk, folder or file
```