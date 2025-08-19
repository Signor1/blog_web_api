# Blog API

A simple blog REST API built with Rust using Actix Web, PostgreSQL, SeaORM, and JWT authentication.

## Features

- User authentication with JWT tokens
- CRUD operations for blog posts
- Image upload with posts
- Database migrations

## Tech Stack

- **Actix Web** - Web framework
- **PostgreSQL** - Database
- **SeaORM** - ORM with migrations
- **JWT** - Authentication

## Quick Start

1. Install dependencies:
```bash
cargo install sea-orm-cli
```

2. Setup database:
```bash
sea-orm-cli migrate up -u postgres://user:password@localhost:5432/blogdb
sea-orm-cli generate entity -o src/entity
```

3. Run the server:
```bash
cargo run
```

## API Endpoints

**Auth:**
- `POST /auth/register` - Register user
- `POST /auth/login` - Login user

**Posts:**
- `POST /posts/create` - Create post with image (auth required)
- `GET /posts/my-posts` - Get user's posts (auth required)
- `GET /posts/all-posts` - Get all posts
- `GET /posts/post/{uuid}` - Get single post

## Example Usage

```bash
# Create a post
curl -X POST http://localhost:8080/posts/create \
  -H "Authorization: Bearer <jwt-token>" \
  -F "title=My Post" \
  -F "text=Post content" \
  -F "file=@image.jpg"
```

## Environment Variables

```env
DATABASE_URL=postgres://user:password@localhost:port/database-name
JWT_SECRET=your-secret-key
```
