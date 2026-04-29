# Zero To Production

This project accompanies the book *Zero To Production* to gain practical experience with Rust for backend development. 

Before, core logic was only in the lib.rs. As the codebase grew, the following structure was used along with the purpose of each component:

src/
  main.rs              # reads config, binds TcpListener, calls startup::run/app
  lib.rs               # exposes modules only
  configuration.rs     # Settings, DatabaseSettings, get_configuration

  startup.rs           # builds Router, wires routes + shared state

  routes/
    mod.rs             # re-exports route handlers
    health_check.rs    # health_check handlers
    subscriptions.rs   # subscribe handler + request structs

  telemetry.rs         # later: tracing/logging setup
  domain/              # later: business types and validation
    mod.rs
    subscriber_email.rs
    subscriber_name.rs

  email_client.rs      # later: external service integration