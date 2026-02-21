# 🔐 Chinxeleer

A zero-bullshit password manager built with **Rust**, **Diesel**, and **PostgreSQL**. No runtime surprises, just compile-time guarantees and type safety. Because memory safety isn't optional.

## What You Get

- **Rust** — Borrow checker keeping your secrets secure (and your code valid)
- **Diesel ORM** — Compile-time query checking so SQL injection is literally impossible
- **PostgreSQL** — Battle-tested, ACID-compliant storage for all your passwords

## Quick Start

1. Clone this repo
2. Create a `.env` file: `DATABASE_URL=postgres://username:password@localhost/your_db`
3. Run `chinxeleer account --help` to see what's cooking

That's it. No bloat, no fuss.
