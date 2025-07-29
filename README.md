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
- [PostgreSQL](https://www.postgresql.org/download/)
- [Docker](https://www.docker.com/get-started) (optional, for running a local PostgreSQL instance)
- `sqlx-cli` for database migrations: `cargo install sqlx-cli`

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/your-repo-name.git
cd your-repo-name
```

### 2. Set Up the Database

You can use a local PostgreSQL instance or a Docker container.

**Using Docker:**

```bash
docker run --name postgres-dev -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres
```

**Create a `.env` file** in the root of the project and add your database URL:

```
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres
```

### 3. Run Database Migrations

```bash
sqlx database create
sqlx migrate run
```

### 4. Run the Application

```bash
cargo run
```

The application will be running at `http://127.0.0.1:8080`.

## API Endpoints

### Authentication

- `POST /api/register`: Register a new user.
- `POST /api/login`: Log in and receive a JWT token.
- `GET /api/me`: Get the current user's information (requires authentication).

### Projects

- `POST /api/projects`: Create a new project.
- `GET /api/projects`: List all projects for the current user.
- `GET /api/projects/{id}`: Get a specific project by ID.
- `PUT /api/projects/{id}`: Update a project.
- `DELETE /api/projects/{id}`: Delete a project.

### Tasks

- `POST /api/tasks`: Create a new task.
- `GET /api/tasks`: List all tasks for the current user.
- `GET /api/tasks/{id}`: Get a specific task by ID.
- `PUT /api/tasks/{id}`: Update a task.
- `DELETE /api/tasks/{id}`: Delete a task.

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```