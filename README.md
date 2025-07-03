# flox - World Chat

FLOX is a lightweight, passwordless messaging web app that lets users post short messages and delete their own entries using cookie-based authentication — no account or password required. 

This project was built while learning Rust, PostgreSQL, and web development. FLOX isn’t meant to compete with real-time chat applications — instead, it serves as a practical, hands-on project to explore programming concepts and database interactions in a fun and meaningful way.

## Project Overview

- Simple world chat: anyone can send messages with a name, and delete them.
- Built with Rust (using Actix-Web), SQLx, and Askama for templating.
- Stores messages in PostgreSQL.
- Uses Docker for easy setup.
- Frontend is plain HTML/CSS with minimal dependencies.

## File Structure

- `src/` - Rust backend code.
- `templates/` - HTML templates for Askama.
- `static/` - Static files like CSS.
- `migrations/` - SQL migration scripts.
- `Dockerfile` - For container setup.

## Why This Project Exists

Why "FLOX"?

The name FLOX comes from the idea of a flowing, open message board — no accounts, no gates — just a stream of thoughts.

This project was created for self-learning, to experiment with Rust web development, database integration, and containerization in a practical way. It is intentionally simple.

## Contributions

Suggestions and improvements are welcome. This is primarily a learning project, so feel free to fork or adapt as you like.

---

Made for fun and learning.
