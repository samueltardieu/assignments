## Purpose

This program assigns students to projects or papers according to their preferences using the Kuhn-Munkres algorithm (also known as the Hungarian algorithm) by maximizing the students satisfaction.

There are M students and N possible choices (N >= M) numbered from 1 to N. Each student may rank some (from zero to all) of the choices. The satisfaction for a choice at rank r is ((r-1)*m)^p, where m and p can be chosen. All unranked choices are considered to be ranked in position N+1.

## Compiling the program

Install a Rust development toolchain from https://rustup.rs/ if needed. You can then build the program with

```bash
$ cargo build --release
```

The resulting program is located in `target/release/assignments`.

## Input file format

The input file is a CSV file without a header and with a variable number of fields on each line. Each line contains the name of the student (which must be unique) and a ranked list of choices.

## Command line parameters

```
USAGE:
    assignments [FLAGS] [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -v               Be verbose
    -V, --version    Prints version information

OPTIONS:
    -m, --mult <coeff>       Multiplicative coefficient for rank [default: 1]
    -n, --num-choices <n>    Number of choices [default: the number of students]
    -p, --power <coeff>      Power coefficient for rank [default: 2]

ARGS:
    <INPUT>    CSV file
```

## Testing

Test files can be generated using `utils/mkcsv.py`. This program takes an integer representing the number of students, and generates a file with a line per student and an acceptable random choices set.

## License

This software can be redistributed under the [GNU General Public License](https://www.gnu.org/licenses/gpl-3.0.html) version 3 or a later version, at your convenience.
