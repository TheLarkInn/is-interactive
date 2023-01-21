# is-interactive
Rust adaptation of [sindresorhus/is-interactive](https://github.com/sindresorhus/is-interactive) from NodeJS
> Check if stdout or stderr is [interactive](https://unix.stackexchange.com/a/43389/7678)

It checks that stedout or stderr is [TTY](https://jameshfisher.com/2017/12/09/what-is-a-tty/), not a dumb terminal, and not running in a CI.

This can be useful to decide whether to present interactive UI or animations in the terminal.

## Install

```
$ cargo add is-interactive
```

## Usage

```rust
use is_interactive::is_interactive;

fn main() {
  if is_interactive() {
    // show beautiful animations 
  } else {
    // skip boring animations
  };
}

```
