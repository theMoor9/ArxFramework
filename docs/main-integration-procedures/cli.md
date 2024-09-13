# Standard Procedure


>Procedura Standard d'Uso per `cli.rs`

---

## Layer 1 Code Base `main.rs`

- Importazione nel main del cli

```Rust
mod cli;
```

- Generazione istanza di config

```Rust
let config = cli::parse_config_cli()
```