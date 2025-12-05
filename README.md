## Terminal User Interface(TUI) for PAPI Console

<div align="center"> 
<img width="100" height="100" alt="148661419-419ad5b3-1b9f-480a-b723-3f292616730c" src="https://github.com/user-attachments/assets/bbe3fe0a-5568-4b8d-b880-8bdfa7b0d030" />
<br>
<h4>Polkadot PAPI Console</h4>
</div>

----

<p align="center">
  <a href="https://polkadot.com/developers/" target="blank"><img src="img/built-with-love-for-polkadot.svg" height="20" alt="Polkadot badge" /></a>
  <a href="https://github.com/use-ink/ink" target="blank"><img src="https://raw.githubusercontent.com/use-ink/ink/master/.images/badge_flat.svg" alt="Polkadot badge" /></a>
</p>

----

**Built with:**

- [Ratatui](https://github.com/ratatui/ratatui)
- [Tokio](https://tokio.rs/)
- Rust
- PAPI - Polkadot API

**Build Project:**

```shell
cargo build --color=always --message-format=json-diagnostic-rendered-ansi --profile dev
```

**Run TUI:**

```shell
cargo run --color=always --package tui-cli-wrapper --bin tui-cli-wrapper --profile dev
```

> ℹ️ Don't see the TUI? Try expanding the size of your console and scrolling up or down.

<img width="1437" height="378" alt="image" src="https://github.com/user-attachments/assets/9de399a9-21d8-4353-bf5f-291d57c536fd" />

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

----

<div align="center"> 
<img width="100" height="100" alt="148661419-419ad5b3-1b9f-480a-b723-3f292616730c" src="https://github.com/user-attachments/assets/bbe3fe0a-5568-4b8d-b880-8bdfa7b0d030" />
<br>
<h4>Polkadot</h4>
</div>

