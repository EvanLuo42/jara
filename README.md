# Jara

## Installation
Install cargo and run:
```bash
cargo install jara
```

## Usage

```
Usage: jara <COMMAND>

Commands:
  install   Install a JDK
  set       Set current JDK
  import    Import JDK
  versions  List all imported & installed versions
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Set Current JDK
```
Set current JDK

Usage: jara set <BUILD> <ARCH> <VERSION>

Arguments:
  <BUILD>
  <ARCH>
  <VERSION>

Options:
  -h, --help  Print help
```

### Import JDK
```
Import JDK

Usage: jara import <PATH>

Arguments:
  <PATH>

Options:
  -h, --help  Print help
```

### List All Imported & Installed Versions
```
List all imported & installed versions

Usage: jara versions

Options:
  -h, --help  Print help
```
