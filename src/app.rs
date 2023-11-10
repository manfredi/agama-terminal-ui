use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Current edit state
#[derive(Debug)]
pub enum CurrentlyEditing {
    Username,
    Password,
}

/// Current screen active
#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    // pub counter: u8,
    /// login username
    pub username_input: String,
    /// login password
    pub password_input: String,
    /// Position of cursor in the editor area.
    pub cursor_position: usize,
    /// The current screen the user is looking at
    pub current_screen: CurrentScreen,
    /// The state of current edit
    pub currently_editing: Option<CurrentlyEditing>,
}


impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            // counter: 0,
            username_input: String::new(),
            password_input: String::new(),
            cursor_position: 0,
            current_screen: CurrentScreen::Main,
            currently_editing: Some(CurrentlyEditing::Username), // None
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // pub fn increment_counter(&mut self) {
    //     if let Some(res) = self.counter.checked_add(1) {
    //         self.counter = res;
    //     }
    // }

    // pub fn decrement_counter(&mut self) {
    //     if let Some(res) = self.counter.checked_sub(1) {
    //         self.counter = res;
    //     }
    // }

    pub fn clear_area(&mut self) {
        self.current_screen = CurrentScreen::Exiting;
    }

    pub fn enter_char(&mut self, new_char: char) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Username => { self.username_input.push(new_char); }
                CurrentlyEditing::Password => { self.password_input.push(new_char); }
            }
        }
        
    }

    pub fn remove_char(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Username => { self.username_input.pop(); }
                CurrentlyEditing::Password => { self.password_input.pop(); }
            }
        }
        
    }

    pub fn next_frame(&mut self) {
        match self.current_screen {
            CurrentScreen::Editing => {
                if let Some(edit_mode) = &self.currently_editing {
                    match edit_mode {
                        CurrentlyEditing::Username => self.currently_editing = Some(CurrentlyEditing::Password),
                        CurrentlyEditing::Password => self.currently_editing = Some(CurrentlyEditing::Username),
                    };
                } else {
                    self.currently_editing = Some(CurrentlyEditing::Username);
                }
            }
            CurrentScreen::Exiting => {

            }
            CurrentScreen::Main => {
                self.current_screen = CurrentScreen::Editing;
            }
        }
        
    }
    

}

