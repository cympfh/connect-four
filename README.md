# Connect-Four Game AI

## References

1. [wikipedia/Connect_Four](https://en.wikipedia.org/wiki/Connect_Four)
1. [nintendo/Club House Games](https://www.nintendo.com/games/detail/clubhouse-games-51-worldwide-classics-switch/#all-games)

## Usage

### Input Text Format

Each Input is like

```
.......
.......
.......
...o...
...ox..
```

Character `o` and `x` are the colored discs for players.
And `.` is for empty.

The height and width can be any size.
The following is also ok.

```
...
.o.
```

### Commandline

This AI works with `cargo`.

```bash
cargo run -- --next o < ./sample.input
```

`--next` specifies which (`o` or `x`) is the next player.

### Output

Best move for the `--next` player will be outputed.

