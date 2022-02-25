# z-tab-bar

zellij tab-bar plugin that is similar to tmux default status line

## building

```sh
cargo build --release
```

## usage

### setup

Create a common place for plugins

```sh
mkdir -p ~/.zellij/plugins
```

Update `~/.config/zellij/layouts/default.yaml`
**NOTE** USER_HOME needs to be the full path for now

```yaml
---
template:
  direction: Horizontal
  parts:
    - direction: Vertical
      body: true
    - direction: Vertical
      borderless: true
      split_size:
        Fixed: 1
      run:
        plugin:
          location: "file:USER_HOME/.zellij/plugins/tab-bar.wasm"
session:
  name: "0"
  attach: true
```

Copy complied plugin to the plugins directory

```sh
cp target/wasm32-wasi/release/tab-bar.wasm ~/.zellij/plugins
```
