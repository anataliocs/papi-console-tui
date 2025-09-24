use ratatui::style::{Styled, Stylize};
use ratatui::text::ToText;
use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;

pub(crate) mod layout {
    use std::borrow::{Borrow, BorrowMut};

    use ratatui::buffer::Buffer;
    use ratatui::Frame;
    use ratatui::layout::{Alignment, Constraint, Layout, Rect};
    use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
    use ratatui::symbols::scrollbar;
    use ratatui::text::Text;
    use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Tabs, Wrap};
    use strum::IntoEnumIterator;

    use crate::app;
    use crate::app::{App, SelectedTab};
    use crate::app::SelectedTab::{Tab1, Tab2, Tab3, Tab4};
    use crate::event::{UiUpdateContent, UiUpdatePayload, UiWidget};

    /// Renders the user interface widgets.
    pub fn render<'a>(app: &'a mut App, frame: &mut Frame, event1: UiUpdateContent) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
        // - https://github.com/ratatui/ratatui/tree/master/examples

        let [tab_area, top_area, bot_area] =
            Layout::vertical([Constraint::Max(3), Constraint::Max(4), Constraint::Fill(2)])
                .areas(frame.area());

        let [top_left, top_right] =
            Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(tab_area);

        let [bot_left, bot_right] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(4)]).areas(bot_area);

        let [bot_right_console, bot_right_scroll] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Max(2)]).areas(bot_right);

        let titles = app::SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), app.selected_tab.palette().c950);
        let selected_tab_index = app.selected_tab as usize;

        match event1.ui_widget() {
            UiWidget::NoUpdate => {}
            UiWidget::Tabs => {}
            UiWidget::Network => {}
            UiWidget::ListSelect => {}
            UiWidget::CmdOutput => {}
            UiWidget::Scrollbar => {}
        }

        render_network_widget(frame, &event1, top_right);


        frame.render_widget(
            Tabs::new(titles).style(Style::default().add_modifier(Modifier::BOLD))
                             .highlight_style(highlight_style)
                             .select(selected_tab_index)
                             .padding(" ", " ")
                             .divider(" ")
                             .block(Block::bordered().border_type(BorderType::Rounded)
                                                     .bg(Color::DarkGray)
                                                     .padding(Padding::horizontal(2))
                             ),
            top_left,
        );

        frame.render_widget(
            Paragraph::new(
                "Press `Esc`, `Ctrl-C` or `q` to quit.\n\
                Press left and right to move between tabs.\n",
            )
                .block(
                    Block::bordered()
                        .title("Stellar Contract Explorer")
                        .title_alignment(Alignment::Center)
                        .title_style(Style::default().add_modifier(Modifier::BOLD))
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .centered(),
            top_area,
        );

        frame.render_stateful_widget(
            match app.selected_tab {
                Tab1 => list_factory(
                    vec![
                        ListItem::new("Extend Instance TTL"),
                        ListItem::new("Extend Persistence TTL"),
                        ListItem::new("Extend Temporary TTL"),
                        ListItem::new("Generate Data Key"),
                    ],
                    "Extend TTL Scripts",
                ),
                Tab2 => list_factory(
                    vec![
                        ListItem::new("Restore Persistent Storage"),
                        ListItem::new("Restore Contract Instance"),
                        ListItem::new("Restore Contract Code Hash"),
                        ListItem::new("Restore Instance Storage"),
                    ],
                    "Restore Archived Data Scripts",
                ),
                Tab3 => list_factory(
                    vec![
                        ListItem::new("Set Persistent Data"),
                        ListItem::new("Set Instance Data"),
                        ListItem::new("Extend Persistent TTL"),
                        ListItem::new("Extend Instance TTL"),
                    ],
                    "Contract Invocation Scripts",
                ),
                Tab4 => list_factory(
                    vec![
                        ListItem::new("Show Contract Data"),
                        ListItem::new("Show Invocations"),
                        ListItem::new("Show Storage TTLs"),
                        ListItem::new("Show Misc data"),
                    ],
                    "Display Contract Info Scripts",
                ),
            },
            bot_left,
            match selected_tab_index {
                0 => {
                    &mut app.list_states.list_state
                }
                1 => {
                    &mut app.list_states.list_state2
                }
                2 => {
                    &mut app.list_states.list_state3
                }
                3 => {
                    &mut app.list_states.list_state4
                }
                _ => {
                    &mut app.list_states.list_state
                }
            },
        );

        render_cmd_output_window(frame, event1, bot_right_console);

        CmdOutputScrollbar::default()
            .render(bot_right_scroll, frame.buffer_mut(), &mut app.cmd_output_state.cmd_output_scrollbar);
    }

    fn render_cmd_output_window(frame: &mut Frame, event1: UiUpdateContent, bot_right_console: Rect) {
        frame.render_widget(
            Paragraph::new(Text::raw(event1.ui_update_content()))
                .left_aligned()
                .scroll((0, 0))
                .wrap(Wrap::default())
                .block(
                    Block::bordered()
                        .title("Command Output")
                        .title_alignment(Alignment::Center)
                        .title_style(Style::default().add_modifier(Modifier::BOLD))
                        .border_type(BorderType::Rounded).padding(Padding::symmetric(1, 1)),
                )
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .left_aligned(),
            bot_right_console,
        );
    }

    fn render_network_widget(frame: &mut Frame, event1: &UiUpdateContent, top_right: Rect) {
        frame.render_widget(
            Paragraph::new(Text::raw(event1.ui_update_content().to_string()))
                .right_aligned()
                .style(Style::default().add_modifier(Modifier::BOLD)
                                       .bg(Color::DarkGray).fg(Color::Yellow))
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(Padding::horizontal(2)
                    )),
            top_right,
        );
    }

    fn list_factory<'a>(list_items: Vec<ListItem<'a>>, title: &'a str) -> List<'a> {
        List::new(list_items)
            .bg(Color::Black)
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .highlight_symbol(" ❯❯ ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded).border_style(Style::default().fg(Color::Yellow))
                    .title(title)
                    .title_style(Style::default().add_modifier(Modifier::BOLD)),
            )
    }

    #[derive(Debug)]
    pub(crate) struct CmdOutputScrollbar {
        scrollbar_state: ScrollbarState,
    }

    impl CmdOutputScrollbar {
        pub fn scroll(mut self) {
            &mut self.borrow_mut().scrollbar_state.next();
        }

        pub fn set_scrollbar_state(&mut self, scrollbar_state: ScrollbarState) {
            self.scrollbar_state = scrollbar_state;
        }

        pub fn scrollbar_state(&self) -> ScrollbarState {
            self.scrollbar_state
        }
    }

    impl Default for CmdOutputScrollbar {
        fn default() -> Self {
            Self { /* fields */ scrollbar_state: Default::default() }
        }
    }

    impl Styled for CmdOutputScrollbar {
        type Item = ();

        fn style(&self) -> Style {
            todo!()
        }

        fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
            todo!()
        }
    }

    impl StatefulWidget for CmdOutputScrollbar {
        type State = (ScrollbarState);

        fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
            self.borrow_mut().set_scrollbar_state(state.to_owned());

            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Option::from("v"))
                .end_symbol(Option::from("^"))
                .symbols(scrollbar::VERTICAL)
                .begin_symbol(None)
                .track_symbol(None)
                .end_symbol(None)
                .render(area, buf, state);
        }
    }
}