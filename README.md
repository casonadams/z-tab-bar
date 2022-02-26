# z-tab-bar

zellij tab-bar plugin that is similar to tmux default status line

<img width="718" alt="image" src="https://user-images.githubusercontent.com/17597548/155839636-1a14d830-320f-4dc4-85eb-20a4ea7e2431.png">

## building

```sh
cargo build --release
```

## usage

### setup

- Create a common place for plugins

```sh
mkdir -p ~/.zellij/plugins
```

- Update `~/.config/zellij/layouts/default.yaml`

- **NOTE** USER_HOME needs to be the full path for now

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
          location: "file:USER_HOME/.zellij/plugins/z-tab-bar.wasm"
session:
  name: "0"
  attach: true
```

- Suggested theme

```yaml
themes:
  default:
    fg: 7
    bg: 24
    black: 0 # tab-bar foreground color
    red: 1
    green: 2 # tab-bar background color
    yellow: 3
    blue: 4
    magenta: 5
    cyan: 6
    white: 7
    orange: 208
    gray: 247
```

- Copy complied plugin to the plugins directory

```sh
cp target/wasm32-wasi/release/z-tab-bar.wasm ~/.zellij/plugins
```

## roadmap

- [ ] config driven options for colors
- [ ] config driven options to customize statusline (hostname, time, etc)
