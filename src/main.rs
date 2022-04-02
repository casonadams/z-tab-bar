mod line;
mod tab;

use std::cmp::{max, min};
use std::convert::TryInto;

use zellij_tile::prelude::*;

use crate::line::tab_line;
use crate::tab::tab_style;

#[derive(Debug, Default)]
pub struct LinePart {
  part: String,
  len: usize,
}

#[derive(Default)]
struct State {
  tabs: Vec<TabInfo>,
  active_tab_idx: usize,
  mode_info: ModeInfo,
  mouse_click_pos: usize,
  should_render: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
  fn load(&mut self) {
    set_selectable(false);
    subscribe(&[EventType::TabUpdate, EventType::ModeUpdate, EventType::Mouse]);
  }

  fn update(&mut self, event: Event) {
    match event {
      Event::ModeUpdate(mode_info) => self.mode_info = mode_info,
      Event::TabUpdate(tabs) => {
        self.active_tab_idx = tabs.iter().position(|t| t.active).unwrap() + 1;
        let mut new_tabs: Vec<TabInfo> = vec![];
        for (i, tab) in tabs.iter().enumerate() {
          let tab_name;
          if tab.name.contains("Tab #") {
            tab_name = format!("{}:tab", i);
          } else {
            tab_name = format!("{}:{}", i, tab.name.to_owned().replace(" ", "_"));
          }
          let temp = TabInfo {
            position: tab.position,
            name: tab_name,
            active: tab.active,
            panes_to_hide: tab.panes_to_hide,
            is_fullscreen_active: tab.is_fullscreen_active,
            is_sync_panes_active: tab.is_sync_panes_active,
            are_floating_panes_visible: tab.are_floating_panes_visible,
            other_focused_clients: tab.clone().other_focused_clients,
          };
          new_tabs.push(temp);
        }
        self.tabs = new_tabs;
      }
      Event::Mouse(me) => match me {
        Mouse::LeftClick(_, col) => {
          self.mouse_click_pos = col;
          self.should_render = true;
        }
        Mouse::ScrollUp(_) => {
          switch_tab_to(min(self.active_tab_idx + 1, self.tabs.len()) as u32);
        }
        Mouse::ScrollDown(_) => {
          switch_tab_to(max(self.active_tab_idx.saturating_sub(1), 1) as u32);
        }
        _ => {}
      },
      _ => unimplemented!(), // FIXME: This should be unreachable, but this could be cleaner
    }
  }

  fn render(&mut self, _rows: usize, cols: usize) {
    if self.tabs.is_empty() {
      return;
    }
    let mut all_tabs: Vec<LinePart> = vec![];
    let mut active_tab_index = 0;
    for t in &mut self.tabs {
      active_tab_index = t.position;
      let tab = tab_style(
        &mut t.clone(),
        self.mode_info.style.colors,
        t.other_focused_clients.as_slice(),
      );
      all_tabs.push(tab);
    }
    let tab_line = tab_line(
      self.mode_info.session_name.as_deref(),
      all_tabs,
      active_tab_index,
      cols.saturating_sub(1),
      self.mode_info.style.colors,
    );
    let mut s = String::new();
    let mut len_cnt = 0;
    for (idx, bar_part) in tab_line.iter().enumerate() {
      s = format!("{}{}", s, &bar_part.part);

      if self.should_render
        && self.mouse_click_pos > len_cnt
        && self.mouse_click_pos <= len_cnt + bar_part.len
        && idx > 2
      {
        // First three elements of tab_line are "Zellij", session name and empty thing, hence the idx > 2 condition.
        // Tabs are indexed starting from 1, therefore we need subtract 2 below.
        switch_tab_to(TryInto::<u32>::try_into(idx).unwrap() - 2);
      }
      len_cnt += bar_part.len;
    }
    match self.mode_info.style.colors.green {
      PaletteColor::Rgb((r, g, b)) => {
        println!("{}\u{1b}[48;2;{};{};{}m\u{1b}[0K", s, r, g, b);
      }
      PaletteColor::EightBit(color) => {
        println!("{}\u{1b}[48;5;{}m\u{1b}[0K", s, color);
      }
    }
    self.should_render = false;
  }
}
