# basers

Convert rational numbers to another base with repeating fractional parts.

`basers` reads a numerator/denominator and optional base from stdin, then writes
an exact representation of the value in that base. Repeating fractional digits
are wrapped in parentheses.

## Input format

```
p[/q][@b]
```

- `p` is the numerator.
- `q` is the denominator (defaults to `1`).
- `b` is the base (defaults to `10`).

Examples:

- `5` means `5/1` in base 10.
- `1/3` means one third in base 10.
- `10@2` means `10/1` in base 2.
- `1/8@2` means one eighth in base 2.

## Output

- Integer and fractional parts are separated by `.`.
- Repeating fractional digits are wrapped in `(...)`.
- Digits are `0-9A-Z` (bases up to 36 are supported).

## Examples

```
$ echo "1/3" | cargo run --quiet
0.(3)

$ echo "1/8@2" | cargo run --quiet
0.001

$ echo "10@2" | cargo run --quiet
1010

$ echo "1/6@10" | cargo run --quiet
0.1(6)
```

## Build

```
cargo build
```

## Notes

- Input is read from stdin and output is written to stdout.
- Bases outside 2-36 are not validated yet; invalid digits will panic.
