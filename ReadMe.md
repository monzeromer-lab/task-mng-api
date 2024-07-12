# Task Management Application API - Built with Rust (DDD)

Welcome to the Task Management Application API, designed using Rust and Domain-Driven Design (DDD) principles. This project is perfect for front-end developers who need a robust backend demo for building and testing a task management application locally. It's an excellent tool for learning front-end development and adding a project to your portfolio.

If you find this project useful, please give it a star!

## Features

### User Management
- **Login:** Secure user authentication.
- **Signup:** User registration with validation.
- **Profile:** Manage user profile details.

### Task Management
- **Create and Manage Tasks:** Add, update, and delete tasks efficiently.
- **Due Dates:** Set and track task deadlines.
- **Notifications:** Receive alerts for upcoming due dates and important updates.
- **Collaborations:** Share and manage tasks with other users.

## API Documentation

### Authentication

#### Login
```http
POST /api/auth/login
```
**Request Body:**
```json
{
  "username": "yourUsername",
  "password": "yourPassword"
}
```
**Response:**
```json
{
  "token": "yourAuthToken"
}
```

#### Signup
```http
POST /api/auth/signup
```
**Request Body:**
```json
{
  "username": "yourUsername",
  "password": "yourPassword",
  "email": "yourEmail@example.com"
}
```
**Response:**
```json
{
  "message": "User registered successfully"
}
```

### Task Management

#### Create Task
```http
POST /api/tasks
```
**Request Body:**
```json
{
  "title": "Task Title",
  "description": "Task Description",
  "dueDate": "2023-12-31",
  "assignedTo": "userId"
}
```
**Response:**
```json
{
  "message": "Task created successfully"
}
```

#### Get Tasks
```http
GET /api/tasks
```
**Response:**
```json
[
  {
    "id": "taskId",
    "title": "Task Title",
    "description": "Task Description",
    "dueDate": "2023-12-31",
    "assignedTo": "userId",
    "status": "Pending"
  }
]
```

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/monzeromer-lab/task-mng-api.git
   ```
2. Navigate to the project directory:
   ```sh
   cd task-mng-api
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```
4. Run the server:
   ```sh
   cargo run
   ```

## Contact

If you have any questions or suggestions, feel free to reach out:

- **Email:** monzer.a.omer@gmail.com
- **LinkedIn:** [Monzer Omer](https://www.linkedin.com/in/monzeromer)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with any improvements or new features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

Feel free to contribute and suggest new features! This API is an excellent resource for developing and testing front-end applications locally.
