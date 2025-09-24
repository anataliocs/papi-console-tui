## Terminal User Interface Wrapper for CLI

**Built with:**
- [Ratatui](https://github.com/ratatui/ratatui)
- [Tokio](https://tokio.rs/)
- Rust

**Build Project:**
```shell
cargo build --color=always --message-format=json-diagnostic-rendered-ansi --profile dev
```

**Run TUI:**
```shell
cargo run --color=always --package tui-cli-wrapper --bin tui-cli-wrapper --profile dev
```

> ℹ️ Don't see the TUI?  Try expanding the size of your console and scrolling up or down. 

<img width="1343" height="472" alt="image" src="https://github.com/user-attachments/assets/bd1d66db-1967-4b86-9c2b-8380e2d4cf14" />

**Controls**
- Use arrows ⬆️⬇️⬅️➡️ to navigate Terminal user interface
- Use `Enter` to select option

**Project structure:**

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```

This is identical to the [simple] template but has `async` events out of the box with `tokio` and
`crossterm`'s `EventStream`.

