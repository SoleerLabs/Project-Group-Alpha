# 📋 Collaborative Task Manager API

A full-featured task management REST API built with Rust, enabling users to organize work through projects and tasks with comprehensive authentication and authorization.

## 🚀 Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)** - Modern async web framework for Rust
- **[SQLx](https://github.com/launchbadge/sqlx)** - Async PostgreSQL driver with compile-time query validation
- **[PostgreSQL](https://www.postgresql.org/)** - Robust relational database for data persistence
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)** - JWT implementation for secure authentication
- **[Tower](https://github.com/tower-rs/tower)** - Middleware ecosystem for request/response processing
- **[Argon2](https://github.com/RustCrypto/password-hashes)** - Secure password hashing algorithm
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime for Rust

## 📋 Features

### 👥 User Management
- Secure user registration with Argon2 password hashing
- JWT-based authentication system
- Protected routes with middleware validation

### 📁 Project Organization
- Create and manage personal projects
- Project-level authorization (users can only access their own projects)
- Full CRUD operations with cascade deletion
- Pagination support for project listings

### ✅ Task Management
- Create tasks within projects with due dates
- Task status tracking (pending, in_progress, completed)
- Comprehensive filtering and search capabilities
- Complete task lifecycle management

### 🔒 Security & Performance
- Argon2 password hashing for maximum security
- JWT token-based stateless authentication
- Project-level access control
- Efficient pagination for large datasets
- Async/await for optimal performance

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Git](https://git-scm.com/downloads)
- `sqlx-cli` for database migrations: `cargo install sqlx-cli`

## Getting Started

### 1. Fork and Clone the Repository

First, fork the repository on GitHub to create your own copy.

Then, clone your forked repository to your local machine, replacing `<your-github-username>` with your actual GitHub username:

```bash
git clone https://github.com/<your-github-username>/Project-Group-Alpha.git
cd Project-Group-Alpha
```

### 2. Set Up the Database

This project requires a PostgreSQL database. You have two main options: running it locally with Docker or using a managed cloud provider.

**Option A: Local Setup with Docker**

This is the quickest way to get a database running locally.

1.  Ensure [Docker](https://www.docker.com/get-started) is installed and running.
2.  Run the following command to start a PostgreSQL container:
    ```bash
    docker run --name postgres-dev -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres
    ```
3.  Your database URL will be: `postgres://postgres:mysecretpassword@localhost:5432/postgres`

**Option B: Cloud Database (Neon, Supabase, etc.)**

1.  Create a new project and database with your preferred cloud provider (e.g., [Neon](https://neon.tech/), [Supabase](https://supabase.com/)).
2.  Locate the database connection string (often called a "Database URL" or "Connection URI"). It will look something like this: `postgres://<user>:<password>@<host>:<port>/<database>`
3.  Ensure the connection string is for a direct PostgreSQL connection, not a connection pooler, for migrations to work correctly.

### 3. Configure Environment Variables

Create a `.env` file in the root of the project. Copy the appropriate database URL from the step above into it:

```
# Example for local Docker setup
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres

# Example for a cloud database
# DATABASE_URL=postgres://user:password@host:port/database
```

### 4. Run Database Migrations

This command sets up the necessary tables and relationships in your database.

```bash
# This may not be necessary if your cloud provider already created the database
sqlx database create

# Run all migration files from the /migrations directory
sqlx migrate run
```

### 5. Run the Application

You can now start the API server.

```bash
cargo run
```

The application will be running at `http://127.0.0.1:8080`.

## API Endpoints

All endpoints are prefixed with `/api`.

### Authentication

- `POST /register`: Register a new user.
- `POST /login`: Log in and receive a JWT token.
- `GET /me`: Get the current user's information (requires authentication).

### User Management

- `GET /users`: List all users.
- `GET /users/{id}`: Get a specific user by ID.
- `PUT /users/{id}`: Update a user's username.
- `DELETE /users/{id}`: Delete a user.

### Projects

- `POST /projects`: Create a new project.
- `GET /projects`: List all projects for the current user.
- `GET /projects/{id}`: Get a specific project by ID.
- `PUT /projects/{id}`: Update a project.
- `DELETE /projects/{id}`: Delete a project.

### Tasks

- `POST /tasks`: Create a new task.
- `GET /tasks`: List all tasks for the current user.
- `GET /tasks/{id}`: Get a specific task by ID.
- `PUT /tasks/{id}`: Update a task.
- `DELETE /tasks/{id}`: Delete a task.

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```
