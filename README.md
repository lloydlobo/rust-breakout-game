# Rust Breakout Game

Easy to play & build Breakout Game binary built in Rust with macroquad game library.

![rust-breakout-game_v0.1.2](https://user-images.githubusercontent.com/76430758/183045843-62da6cf0-31d1-4e41-9ee5-6fd6a052ff4f.png)

<!-- ![rust-breakout-game_v0.1.1](https://user-images.githubusercontent.com/76430758/183031203-0c71cc2c-f513-4415-8fa3-be35c03a6fcc.png) -->

## Development

### Release

- Incremental releases: (v0.1.1 --> v0.1.2)

```sh
cargo release patch --no-publish --execute
```

### Pre-requisites

- Audio library (optional)

```sh
sudo apt-get install libatlas-base-dev
```

## File Structure

```ascii
|- .git
|- .gitignore
|- Cargo.lock
|- Cargo.toml
|- README.md
|- src
|- target
.
```

[Source](https://stackoverflow.com/questions/59938763/usr-bin-ld-cannot-find-lasound)
