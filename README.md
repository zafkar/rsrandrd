# rsrandrd

A Rust reimplementation of [`srandrd`](https://github.com/jceb/srandrd), a simple X11 RandR event daemon. `rsrandrd` listens for RandR output change events (e.g., monitor connect/disconnect) and executes a user-defined command in response.

## Features

- Listens for X11 RandR output events using `x11rb`
- Emits a user-defined command on output connect/disconnect
- Passes context via environment variables:
  - `SRANDRD_OUTPUT`: Output ID
  - `SRANDRD_EVENT`: `connected` or `disconnected`
  - `SRANDRD_EDID`: Monitor EDID (hex string)
  - `SRANDRD_SCREENID`: Screen index via Xinerama

## Requirements

- X11 with RandR and Xinerama extensions
- Rust toolchain
- `x11rb` crate

## Build

```sh
cargo build --release
```

## Install

```sh
cargo install --git https://github.com/zafkar/rsrandrd.git
```

## Usage

```sh
rsrandrd /path/to/your/script.sh
```

Your script will be called with the environment variables described above.

## Example

```bash
#!/bin/bash
echo "Event: $SRANDRD_EVENT"
echo "Output: $SRANDRD_OUTPUT"
echo "EDID: $SRANDRD_EDID"
echo "Screen ID: $SRANDRD_SCREENID"
```

Assuming the script can be found at the path ~/test.sh
You can for example then add the following to your ~/.xinitrc

```
exec rsrandr ~/test.sh
```

