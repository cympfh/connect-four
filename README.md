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

The height and width can be any size if `>= 4`.
The following is also ok, and this is the minimal size.

```
....
....
....
.o..
```

### Commandline

This AI works with `cargo`.

```bash
cargo run --release -- --next x < ./sample.input
```

`--next` specifies which (`o` or `x`) is the next player.

### Output

Best move for the `--next` player will be outputed.

### Graphical User Interface

`server.py` is written in Python3 and launch a server for UI.

```bash
pip3 install fastapi uvicorn
uvicorn server:app --port 8080
$(open-browser) http://localhost:8080
```

