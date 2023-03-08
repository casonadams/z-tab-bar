use crate::LinePart;
use ansi_term::{ANSIString, ANSIStrings};
use unicode_width::UnicodeWidthStr;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

fn cursors(focused_clients: &[ClientId], palette: Palette) -> (Vec<ANSIString>, usize) {
  // cursor section, text length
  let mut len = 0;
  let mut cursors = vec![];
  for client_id in focused_clients.iter() {
    if let Some(color) = client_id_to_colors(*client_id, palette) {
      cursors.push(style!(color.1, color.0).paint(" "));
      len += 1;
    }
  }
  (cursors, len)
}

pub fn render_tab(text: String, tab: &TabInfo, _is_alternate_tab: bool, palette: Palette, separator: &str) -> LinePart {
  let focused_clients = tab.other_focused_clients.as_slice();
  let separator_width = separator.width();
  let background_color = palette.green;
  let foreground_color = match palette.theme_hue {
    ThemeHue::Dark => palette.black,
    ThemeHue::Light => palette.black,
  };
  let mut tab_text_len = text.width() + (separator_width * 2) + 2; // +2 for padding
  let tab_styled_text = style!(foreground_color, background_color)
    .bold()
    .paint(format!(" {} ", text));

  let tab_styled_text = if !focused_clients.is_empty() {
    let (cursor_section, extra_length) = cursors(focused_clients, palette);
    tab_text_len += extra_length;
    let mut s = String::new();
    let cursor_beginning = style!(foreground_color, background_color).bold().paint("[").to_string();
    let cursor_section = ANSIStrings(&cursor_section).to_string();
    let cursor_end = style!(foreground_color, background_color).bold().paint("]").to_string();
    s.push_str(&tab_styled_text);
    s.push_str(&cursor_beginning);
    s.push_str(&cursor_section);
    s.push_str(&cursor_end);
    s
  } else {
    ANSIStrings(&[tab_styled_text]).to_string()
  };

  LinePart {
    part: tab_styled_text,
    len: tab_text_len,
    tab_index: Some(tab.position),
  }
}

pub fn tab_style(
  mut tabname: String,
  tab: &TabInfo,
  mut is_alternate_tab: bool,
  palette: Palette,
  capabilities: PluginCapabilities,
) -> LinePart {
  let separator = "";

  if tabname.contains("Tab #") {
    tabname = format!("{}:tab", tab.position);
  } else {
    tabname = format!("{}:{}", tab.position, tab.name.to_owned().replace(' ', "_"));
  }

  if tab.active {
    tabname.push('*');
  }
  if tab.is_fullscreen_active {
    tabname.push('Z');
  }
  if tab.is_sync_panes_active {
    tabname.push('S');
  }
  // we only color alternate tabs differently if we can't use the arrow fonts to separate them
  if !capabilities.arrow_fonts {
    is_alternate_tab = false;
  }

  render_tab(tabname, tab, is_alternate_tab, palette, separator)
}
