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

pub fn render_tab(tab_info: &mut TabInfo, palette: Palette, focused_clients: &[ClientId]) -> LinePart {
  let background_color = palette.green;
  let mut tab_text = tab_info.name.clone();
  let mut tab_text_len = tab_info.name.width() + 2; // 2 for left and right separators, 2 for the text padding
  if tab_info.active {
    tab_text = format!("{}*", tab_text);
  }
  if tab_info.is_fullscreen_active {
    tab_text = format!("{}Z", tab_text);
  }
  if tab_info.is_sync_panes_active {
    tab_text = format!("{}S", tab_text);
  }

  let tab_styled = style!(palette.black, background_color).paint(&tab_text);

  let tab_styled_text = if !focused_clients.is_empty() {
    let (cursor_section, extra_length) = cursors(focused_clients, palette);
    tab_text_len += extra_length;
    let cursor_beginning = style!(palette.black, background_color).paint("|");
    let cursor_end = style!(palette.black, background_color).paint("| ");
    let mut tmp = vec![tab_styled, cursor_beginning];
    for section in cursor_section {
      tmp.push(section);
    }
    tmp.push(cursor_end);

    ANSIStrings(&tmp).to_string()
  } else {
    let end = style!(palette.black, background_color).paint(" ");
    ANSIStrings(&[tab_styled, end]).to_string()
  };

  LinePart {
    part: tab_styled_text,
    len: tab_text_len,
  }
}

pub fn tab_style(tab_info: &mut TabInfo, palette: Palette, focused_clients: &[ClientId]) -> LinePart {
  render_tab(tab_info, palette, focused_clients)
}
