use ratatui::{
    layout::Alignment,
    layout::Constraint,
    layout::Rect,
    prelude::Layout,
    prelude::Direction,
    style::{Color, Style, Modifier},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentlyEditing, CurrentScreen};


/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.size());
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!("AGAMA Terminal UI.\nPress `Ctrl-C` to stop running.",
        )).block(
            Block::default()
                .title("")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        // frame.size(),
        chunks[0],
    );

    let key_notes_user = Paragraph::new(format!("{}",app.username_input)).block(Block::default().borders(Borders::ALL).title(" Login Username "));
    let key_notes_pass = Paragraph::new(format!("{}",app.password_input)).block(Block::default().borders(Borders::ALL).title(" Login Password "));
    
    let style_user = {
        match app.currently_editing {
            Some(CurrentlyEditing::Username) => Style::default().fg(Color::LightYellow),
            _ => Style::default()
        }
    };

    let style_pass = {
        match app.currently_editing {
            Some(CurrentlyEditing::Password) => Style::default().fg(Color::LightYellow),
            _ => Style::default()
        }
    };
    
    if let CurrentScreen::Editing = app.current_screen {
        // frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
        frame.render_widget(key_notes_user.style(style_user), chunks[1]);
        frame.render_widget(key_notes_pass.style(style_pass), chunks[2]);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        // frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
        frame.render_widget(Clear, chunks[1]);
        frame.render_widget(Clear, chunks[2]);
        let popup_block = Block::default()
            .title("")
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Green));

        let exit_text = Text::styled(
            "Would you like to exit? (y/n)",
            Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let area = centered_rect(30, 10, frame.size());
        frame.render_widget(exit_paragraph, area);
    }

}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
