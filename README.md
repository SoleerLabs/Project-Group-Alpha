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
