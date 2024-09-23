use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use rig::completion::Chat;
use rig::embeddings::EmbeddingsBuilder;
use rig::providers::openai;
use rig::vector_store::{in_memory_store::InMemoryVectorStore, VectorStore};

const RUST_DOCS: &[(&str, &str)] = &[
    ("compilation error", "Rust compilation errors occur when the code doesn't meet the language's rules. Common causes include syntax errors, type mismatches, and borrowing rule violations."),
    ("borrow checker", "Rust's borrow checker ensures memory safety by enforcing rules about data ownership, borrowing, and lifetimes."),
    ("lifetime", "Lifetimes in Rust are a compile-time feature that helps prevent dangling references and ensures references are valid for a specific scope."),
    ("ownership", "Ownership is a key concept in Rust that governs how memory is managed. Each value has an owner, and there can only be one owner at a time."),
    ("mut", "The 'mut' keyword in Rust indicates that a variable binding is mutable, allowing its value to be changed."),
    ("Result", "Result is an enum used for returning and propagating errors. It has two variants: Ok(T) for success and Err(E) for error."),
    ("Option", "Option is an enum that represents an optional value. It has two variants: Some(T) containing a value, or None representing no value."),
    ("unwrap", "The 'unwrap' method extracts the value from an Option or Result, but panics if it's None or Err."),
    ("expect", "Similar to 'unwrap', but allows specifying an error message if the operation fails."),
    ("Vec", "Vec<T> is a growable array type in Rust, providing a contiguous, heap-allocated list of elements."),
    ("String", "String is the standard string type in Rust, representing a growable, mutable, owned UTF-8 encoded string."),
    ("str", "&str is a string slice, representing a view into a string. It's often used for string literals or borrowed string data."),
    ("match", "The 'match' expression in Rust allows pattern matching against a value, often used for control flow."),
    ("if let", "The 'if let' syntax is a concise way to handle a single pattern matching case, often used with Option or Result types."),
    ("trait", "Traits in Rust define shared behavior for types, similar to interfaces in other languages."),
    ("impl", "The 'impl' keyword is used to implement methods or traits for a type."),
    ("generic", "Generics in Rust allow writing code that works with multiple types, promoting code reuse and type safety."),
    ("macro", "Macros in Rust are a way of writing code that writes other code, often used for metaprogramming and reducing boilerplate."),
    ("async/await", "Async/await in Rust provides a way to write asynchronous code that looks and behaves like synchronous code."),
    ("cargo", "Cargo is Rust's package manager and build system, used for managing dependencies and building projects."),
];

struct App {
    input: String,
    output: String,
    chat_history: Vec<String>,
    input_mode: InputMode,
    rag_agent: rig::rag::RagAgent<openai::CompletionModel, InMemoryVectorStore, InMemoryVectorStore>,
}

enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new(rag_agent: rig::rag::RagAgent<openai::CompletionModel, InMemoryVectorStore, InMemoryVectorStore>) -> App {
        App {
            input: String::new(),
            output: String::new(),
            chat_history: Vec::new(),
            input_mode: InputMode::Normal,
            rag_agent,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();

    // Create embedding model and vector store
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");
    let mut vector_store = InMemoryVectorStore::default();

    // Populate vector store with Rust documentation
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(RUST_DOCS.iter().map(|(k, v)| (k.to_string(), v.to_string(), vec![v.to_string()])).collect())
        .build()
        .await?;
    vector_store.add_documents(embeddings).await?;

    // Create RAG agent
    let rag_agent = openai_client.context_rag_agent("gpt-4")
        .preamble("You are RustBuddy, an AI assistant specialized in explaining Rust compilation errors and suggesting fixes. Provide clear, concise, and accurate explanations. Format your response in Markdown.")
        .dynamic_context(3, vector_store.index(embedding_model))
        .build();

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(rag_agent);

    // Run the main loop
    run_app(&mut terminal, &mut app).await?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let input = app.input.drain(..).collect();
                        app.chat_history.push(format!("You: {}", input));
                        let response = app.rag_agent.chat(&input, vec![]).await.unwrap();
                        app.chat_history.push(format!("RustBuddy: {}", response));
                        app.output = response;
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor at the end of the input text
                chunks[2].x + app.input.len() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }

    let messages: Vec<Spans> = app
        .chat_history
        .iter()
        .map(|m| Spans::from(Span::styled(m, Style::default().add_modifier(Modifier::BOLD))))
        .collect();
    let messages =
        Paragraph::new(messages)
            .block(Block::default().borders(Borders::ALL).title("Messages"))
            .wrap(Wrap { trim: true });
    f.render_widget(messages, chunks[1]);
}