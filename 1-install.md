# Install the World

> Get a blank app running with fast rebuilds

Open a terminal.

> This is a summary of the variants at
> https://bevyengine.org/learn/book/getting-started/setup/.
> If this doesn't work for you, consult the linked guide.

Run the OS-appropriate one of:
- MacOS:
```sh
xcode-select --install
curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | bash
brew install vscode llvm rustup-init
rustup-init
```
- Ubuntu:
```sh
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 lld
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo snap install --classic code
```
- Windows:
```sh
# Some steps can't be scripted; see
# https://bevyengine.org/learn/book/getting-started/setup/#windows
# for manual steps.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Then, run:
```sh
mkdir -p ~/impatient-bevy
cd ~/impatient-bevy
cargo init
cargo add bevy --features dynamic_linking
cargo install toml-cli --git https://github.com/devnev/toml-cli --branch main
toml set -w Cargo.toml profile.dev.opt-level 1
toml set -w Cargo.toml profile.dev.package.'"*"'.opt-level 3
touch rust-toolchain.toml && toml set -w rust-toolchain.toml toolchain.channel nightly
mkdir -p .cargo
curl --proto '=https' --tlsv1.2 -fsSL https://raw.githubusercontent.com/bevyengine/bevy/main/.cargo/config_fast_builds > .cargo/config.toml
cargo build
```

Note that the last command, `cargo build`, may take a few minutes to complete.

Replace `src/main.rs` with:

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

And run:
```sh
cargo run
```

If this doesn't open an app window, the detailed setup guide is available at
https://bevyengine.org/learn/book/getting-started/setup/

## Navigation

[Code](./1-install) / [Prev](0-in-brief.md) / [Next](2-on-screen.md)

## Detailed Walk-Through

---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
