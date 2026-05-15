# Zero To Production

This project accompanies the book *Zero To Production* to gain practical experience with Rust for backend development. 

## Code Adaptations

The following library swaps are made in favor of asynchronous operations:
- `actix` is replaced with `actum` 
- `log` is replaced with `tracing` + `tracing_subscriber` 
- `actix-middleware` is replaced with `tower-http`

## Repository Structure

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


  ## Learning Log

  This is a log of what I learned by chapter that may not be fully seen by the git commits messages

- Chapters 1-2 were mostly setup and the merits of building an Email Newsletter as a small, targeted project
- Chapter 3
   - Lots to look up regarding `actix` vs `actum`
   - understanding the structure of the code took some time
   - adapting the book code to `actum` equivalent was time consuming but helped by LLM usage
   - my LLM usage patterns include 
     - immediate usage:
       - interactive documentation of a library, including bird's eye view 
       - explaining small snippets of code
       - providing small examples of general code usage  
       - best practices for organization of code
     - last resort
       - assistance in debugging compiler errors
       - providing specific code solution      
- Chapter 4
  - looked up significance of `tower-http`, `tracing` and `tracing_subscriber`
  - added log levels to configuration setup
  - `tracinging_subscriber` controls log output specifics