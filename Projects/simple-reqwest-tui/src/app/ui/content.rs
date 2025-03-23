use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style,
    },
    text::Line,
    widgets::{HighlightSpacing, List, ListItem, StatefulWidget},
};

use crate::app::{App, DownloadStatus};

impl App {
    pub fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        //let items = ["Item1", "Item2", "Item3"];
        //let list = List::new(items);

        let items: Vec<ListItem> = self
            .download_hashmap
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let (url, downloaditem) = item;
                let display_text = match &downloaditem.status {
                    DownloadStatus::Loading => {
                        format!(
                            "{} Loading: {}",
                            //self.loading_frames[item.loading_frame], item.url
                            self.loading_frames[downloaditem.loading_frame],
                            url
                        )
                    }
                    DownloadStatus::Complete(status) => status.clone(),
                    DownloadStatus::Error(status) => status.clone(),
                };

                ListItem::new(Line::from(display_text)).style(
                    Style::default()
                        .bg(alternate_colors(i))
                        .fg(match &downloaditem.status {
                            DownloadStatus::Loading => BLUE.c400,
                            DownloadStatus::Complete(_) => Color::Green,
                            DownloadStatus::Error(_) => Color::Red,
                        }),
                )
            })
            .collect();

        //let items: Vec<ListItem> = self.download_hashmap.iter().enumerate().map(|(i, item))

        let list = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(BLUE.c500))
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">> ");

        list.render(area, buf, &mut self.list_state);
    }
}

fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        SLATE.c950
    } else {
        SLATE.c900
    }
}
