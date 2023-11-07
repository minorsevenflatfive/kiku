use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  
    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
  
    // This is the state of audio
    let mut status = "Playing";

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Setup sink to handle audio playback
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("audio.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Append source to sink to play
    sink.append(source);
  
    // Main application loop
    loop {
      // Render the UI
      terminal.draw(|f| {
        f.render_widget(Paragraph::new(format!("Song: {status}")), f.size());
      })?;
  
      // Check for user input every 250 milliseconds
      if crossterm::event::poll(std::time::Duration::from_millis(250))? {
        // If a key event occurs, handle it
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
          if key.kind == crossterm::event::KeyEventKind::Press {
            match key.code {
              crossterm::event::KeyCode::Char('j') => {
                sink.play();
                status = "Playing";
            },
              crossterm::event::KeyCode::Char('k') => {
                sink.pause();
                status = "Paused";
            },
              crossterm::event::KeyCode::Char('h') => {
                // TODO: slower playback
            },
              crossterm::event::KeyCode::Char('l') => {
                // TODO: faster playback
            },
              crossterm::event::KeyCode::Char('q') => break,
              _ => {},
            }
          }
        }
      }
    }
  
    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
  
    Ok(())
  }
