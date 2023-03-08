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
mkdir -p ~/.config/zellij/plugins
```

- Update `~/.config/zellij/layouts/default.yaml`

- **NOTE** HOME needs to be the full path for now

```kdl
// ~/.config/zellij/config.kdl

default_layout "z-tab-bar"

plugins {
  tab-bar { path "tab-bar"; }
  z-tab-bar { path "$HOME/.config/zellij/plugins/z-tab-bar"; }
  status-bar { path "status-bar"; }
  strider { path "strider"; }
  compact-bar { path "compact-bar"; }
}
```

```kdl
// ~/.config/zellij/themes/z-tab-bar.kdl
layout {
  pane
  pane size=1 borderless=true {
    plugin location="zellij:z-tab-bar"
  }
}
session_name "0"
attach_to_session true
```

- Suggested theme

```kdl
// ~/.config/zellij/themes/default.kdl
themes {
  default {
    fg 7
    bg 24
    black 0
    red 1
    green 2
    yellow 3
    blue 4
    magenta 5
    cyan 6
    white 7
    orange 208
  }
}
```

- Copy complied plugin to the plugins directory

```sh
cp target/wasm32-wasi/release/z-tab-bar.wasm ~/.config/zellij/plugins
```

## roadmap

- [ ] config driven options for colors
- [ ] config driven options to customize statusline (hostname, time, etc)
