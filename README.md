# A Wandering Mind's Journal

Welcome to the monolithic, over-engineered, uncompromising Rust backend powering my personal corner of the internet.

## Why Does This Exist?

I used to host this blog on Next.js. I had a `node_modules` grave weighing 280MB just to parse markdown, and the browser spent 300ms spinning up a React hydration cycle just so someone could read static serif text. I hated it. 

I briefly nuked it and rewrote it in Go/Templ. That was fast. But then I realized I could squeeze out a few more milliseconds of compile-time performance and eliminate garbage collection entirely if I rewrote the whole thing in Rust. So I did. Because nothing says "aesthetic procrastination" quite like rewriting a simple text blog in an aggressive systems programming language.

## Architecture: The Doomsday Monolith

Sane people use Hugo or Astro for static sites. But I didn't want a static site generator; I wanted to own the engine, and I didn't want my binary talking to the outside world.

This project is a single, self-contained digital printing press. 

- **Routing & Backend**: Axum 0.8 on Tokio.
- **Templating**: Askama. It compiles HTML templates down to raw, statically typed Rust formatting strings. No VDOM, no diffing, no runtime overhead. It just spits out strings and dies.
- **The "Database"**: Doesn't exist. The `contents/` directory is full of `.md` files. At compile time, the `rust-embed` macro sucks every single markdown post directly into the binary. The `AppState` parses them at startup, wraps the massive HTML payloads in atomic pointers (`Arc<String>`), and holds them in memory.
- **Static Assets**: Also embedded. All 36 files of EB Garamond, Playfair Display, and Crimson Text, alongside the minified Tailwind CSS and HTMX, are baked into the executable. No DNS lookups to Google Fonts, no external CDNs.
- **Deployment**: Vercel Serverless Functions. Since it's a monolithic binary, Vercel doesn't have to deal with reading a filesystem or spinning up a Node.js build pipeline. It just executes the binary.

## Running It

If you want to spin up the press yourself:

1. Throw some `.md` files into `./contents`. Frontmatter requires `title`, `date` (YYYY-MM-DD), `author`, `summary`, and `published`.
2. Run `make css` if you changed the Askama HTML templates and need the Tailwind JIT compiler to sweep through and generate new utility classes.
3. Run `cargo run --bin rust-blog`.

The server will wake up on `localhost:3789`. 

## The "CMS"

There is no dashboard. I am an overworked dev; I am not writing a custom auth handler or a rich text editor at 2 AM. If I want to publish an article, my pipeline is primitive:

1. Write a `.md` file.
2. Drop it in `./contents`.
3. `git push`.

Vercel catches the push, the Rust compiler bakes the markdown straight into the binary, and it goes live. High-stakes journalism.

## Performance

By stripping out the "modern web" tax, wiring up `tower_http::compression::CompressionLayer`, forcing `Cache-Control: immutable` headers, and swapping massive heap allocations for cheap `Arc` pointers, this press operates at the theoretical maximum performance floor. 

It is functional garbage that hits a 99 Performance score, and it is beautiful.
