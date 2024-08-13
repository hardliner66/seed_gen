# seed_gen
A small helper tool that can be used when you need one or multiple seeds in a bash script.

- [Usage](#usage)
- [Subcommands](#subcommands)
  - [Single](#single)
  - [Random](#random)
  - [Range](#range)
  - [Full](#full)

## Example
If you want to call something with 10 random seeds you can do something like this:

```bash
# bash example

for seed in $(seed_gen random 10);
do
  echo "$seed"
done
```

```fish
# fish example

for seed in (seed_gen r 10)
  echo "$seed"
end
```

## Usage

```
Usage: seed_gen <COMMAND>

Commands:
  single  Returns the provided seed. If seed is a string, it is hashed beforehand [aliases: s]
  random  Returns the specied amount of random seeds [aliases: r, rand]
  range   Returns numbers in a range
  full    Generates all values from u64::MIN to u64::MAX
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Subcommands
### Single
```
Returns the provided seed. If seed is a string, it is hashed beforehand

Usage: seed_gen single <SEED>

Arguments:
  <SEED>  The specified seed

Options:
  -h, --help  Print help
```

### Random
```
Returns the specied amount of random seeds

Usage: seed_gen random [COUNT]

Arguments:
  [COUNT]  The amount of seeds to print

Options:
  -h, --help  Print help
```

### Range
```
Returns numbers in a range

Usage: seed_gen range [OPTIONS] <MIN> <MAX>

Arguments:
  <MIN>  The lowest number to print
  <MAX>  The highest number to print

Options:
  -s, --step <STEP>  The step size between two numbers
  -h, --help         Print help
```

### Full
```
Generates all values from u64::MIN to u64::MAX

Usage: seed_gen full

Options:
  -h, --help  Print help
```
