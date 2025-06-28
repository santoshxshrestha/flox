# flox - World Chat

Flox is a fast, sleek, and reliable real-time chat application built with Rust, Actix-Web, SQLx, and PostgreSQL. Designed for learning and fun, it lets users send and delete messages in a simple, modern web UI.

## 🚀 Features

- 🌍 Real-time world chat—send and delete messages instantly
- ✨ Minimalist, responsive web interface (see `static/css/home.css`)
- 🔒 PostgreSQL-backed, using SQLx for async database access
- 🚦 Built with Actix-Web for high performance
- 🐳 Ready to run in Docker or with Docker Compose

## 🖥️ Quick Start

### 1. Clone the Repository

```sh
git clone https://github.com/santoshxshrestha/flox.git
cd flox
```

### 2. Run with Docker Compose

This will spin up both the backend and the PostgreSQL database:

```sh
docker-compose up --build
```

The chat app will be available at [http://localhost:8080](http://localhost:8080).

### 3. Manual (Local) Development

- Ensure you have Rust (edition 2024) and PostgreSQL installed.
- Create a `.env` file with your DB connection settings (see `dotenv` usage).
- Run migrations in the `migrations` directory.
- Start the server:

```sh
cargo run
```

## Tech Stack

- **Rust** (edition 2024)
- **Actix-Web** for HTTP and WebSocket server
- **SQLx** for async PostgreSQL access
- **Askama** for HTML templating
- **Docker** & **Docker Compose** for easy deployment

## File Structure

- `src/` — Rust backend source code
- `templates/` — Askama HTML templates (see the chat UI in your example)
- `static/` — Static files (CSS, JS)
- `migrations/` — SQL migration scripts
- `docker-compose.yaml` & `Dockerfile` — Deployment configs

##  Example Usage

Just open the app, enter your name, type a message, and chat with the world!  
You can also delete your own messages for privacy.

## Contributions

PRs and suggestions welcome! This project started as a way to learn Rust, SQLx, and web basics together in a fun, practical way.

## 📄 License

MIT

---

Happy chatting!  
Made with ❤️ using Rust, Actix-Web, SQLx, and Askama.
