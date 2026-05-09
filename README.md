# Rust + Axum + Askama + HTMX + Vercel

A premium bootstrap for modern Rust web development.

## Features
- **Axum 0.8**: High-performance web framework.
- **Askama**: Type-safe templates compiled at build time.
- **HTMX**: Reactive frontend without complex JavaScript.
- **Vercel Ready**: Configured for serverless deployment.
- **Glassmorphism UI**: A sleek, dark-themed design system.

## Local Development
Run the development server:
```bash
cargo run
```
Then visit `http://localhost:3000`.

## Deployment
This project is configured for Vercel. To deploy, use the Vercel CLI:
```bash
vercel deploy
```

## Structure
- `src/lib.rs`: Shared application logic and handlers.
- `src/main.rs`: Local dev entry point.
- `api/index.rs`: Vercel serverless function entry point.
- `templates/`: Askama HTML templates.
- `vercel.json`: Vercel routing configuration.
