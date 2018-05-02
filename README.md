# Digado
a plugin-based chat client

# Setup instructions!
1. If you don't have postgres (or are too lazy to remember how to start it up):
[postgres]
2. Oh and rockert: [rocket]
3. `cargo install diesel_cli --no-default-features --features postgres`
4. `diesel setup`
5. `diesel migration run`
6. `cargo build`

And that should do it... If not, ping me?!

[postgres]: https://www.postgresql.org/docs/10/static/install-short.html
[rocket]: https://rocket.rs/guide/quickstart/#running-examples
