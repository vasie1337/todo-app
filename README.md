# Todo App API

A simple REST API for managing todo tasks built with Rust and Actix Web.

## Features

- Create new tasks
- List all tasks
- Get individual tasks
- Mark tasks as completed
- Delete tasks
- SQLite database storage

## Prerequisites

- Rust (latest stable version)
- Cargo

## Running the Application

1. Clone the repository
2. Run the application:
   ```bash
   cargo run
   ```
3. The server will start on `http://localhost:8080`

## API Endpoints

### Health Check
```http
GET /health
```
Returns server status.

### Create Task
```http
POST /tasks
Content-Type: application/json

{
    "text": "Buy groceries"
}
```
Creates a new task and returns the task ID.

### List All Tasks
```http
GET /tasks
```
Returns all tasks ordered by creation date (newest first).

### Get Single Task
```http
GET /tasks/{id}
```
Returns a specific task by ID.

### Mark Task as Completed
```http
PATCH /tasks/{id}/complete
```
Marks a task as completed and updates the timestamp.

### Delete Task
```http
DELETE /tasks/{id}
```
Deletes a task from the database.

## Response Format

### Task Object
```json
{
    "id": 1,
    "text": "Buy groceries",
    "completed": false,
    "created_at": 1703097600,
    "updated_at": 1703097600
}
```

### Success Response
```json
{
    "message": "Task added successfully",
    "id": 1,
    "text": "Buy groceries"
}
```

### Error Response
```json
{
    "error": "Task not found",
    "id": 1
}
```

## Database

The application uses SQLite for data persistence. The database file will be created automatically when you first run the application. 