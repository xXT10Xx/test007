# Rust Advanced API

A modern, async REST API built with Rust featuring user authentication, database integration, and comprehensive error handling.

## Features

- **Async Web Server**: Built with Axum for high-performance HTTP handling
- **Database Integration**: PostgreSQL with SQLx for type-safe database operations
- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Validation**: Request validation using the validator crate
- **Error Handling**: Comprehensive error handling with custom error types
- **Logging**: Structured logging with tracing
- **CORS Support**: Cross-origin resource sharing enabled
- **Database Migrations**: Automated database schema management

## API Endpoints

### Health Check
- `GET /health` - Service health check

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login

### Users (Protected)
- `GET /api/users` - List users (with pagination)
- `GET /api/users/:id` - Get user by ID
- `POST /api/users` - Create new user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user

## Setup

1. **Prerequisites**
   - Rust 1.70+
   - PostgreSQL 12+

2. **Environment Setup**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Database Setup**
   ```bash
   # Create database
   createdb rust_advanced_api
   
   # Install sqlx-cli for migrations
   cargo install sqlx-cli
   
   # Run migrations
   sqlx migrate run
   ```

4. **Build and Run**
   ```bash
   cargo build
   cargo run
   ```

## Testing

```bash
# Run unit tests
cargo test

# Run with coverage
cargo test --coverage
```

## Project Structure

```
src/
├── config/          # Configuration management
├── database/        # Database connection and setup
├── handlers/        # HTTP request handlers
├── middleware/      # Custom middleware (auth, etc.)
├── models/          # Data models and DTOs
├── services/        # Business logic layer
├── utils/           # Utility functions and error handling
└── main.rs          # Application entry point
```

## Architecture

This project follows a layered architecture:

- **Handlers**: Handle HTTP requests/responses
- **Services**: Contain business logic
- **Models**: Define data structures
- **Database**: Handle data persistence
- **Middleware**: Cross-cutting concerns (auth, logging)

## Security Features

- Password hashing with bcrypt
- JWT token authentication
- Input validation
- SQL injection prevention with SQLx
- CORS protection

## Development

### Adding New Endpoints

1. Define models in `src/models/`
2. Create service logic in `src/services/`
3. Implement handlers in `src/handlers/`
4. Add routes in `src/main.rs`

### Database Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run
```

## License

MIT