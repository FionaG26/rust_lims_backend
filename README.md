# Rust LIMS Backend

A simple backend API for a Laboratory Information Management System (LIMS) built with Rust using the Actix web framework and Diesel ORM for PostgreSQL.

## Features

- Health check endpoint (`/health`).
- User login functionality with bcrypt password verification.
- CRUD operations for samples:
  - Create a new sample (`POST /samples`).
  - Get a sample by ID (`GET /samples/{id}`).
  - Get all samples (`GET /samples`).
  - Delete a sample by ID (`DELETE /samples/{id}`).

## Technologies Used

- **Rust**: The primary language for backend development.
- **Actix Web**: The web framework for building HTTP APIs.
- **Diesel**: ORM for interacting with PostgreSQL.
- **bcrypt**: For password hashing and verification.
- **PostgreSQL**: The database used for storing user and sample data.
- **dotenv**: For managing environment variables.

## Getting Started

Follow these steps to set up and run the project locally.

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (with Cargo)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Diesel CLI](https://diesel.rs/guides/getting-started/)
- [dotenv](https://crates.io/crates/dotenv)

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/rust_lims_backend.git
cd rust_lims_backend
```

### 2. Install Dependencies

Run the following command to install the Rust dependencies:

```bash
cargo build
```

### 3. Set Up Database

Make sure you have PostgreSQL installed and running. Create a new database and set up the schema using Diesel.

1. Configure your `.env` file with your database URL:
   ```ini
   DATABASE_URL=postgres://username:password@localhost/lims_db
   ```

2. Run the migrations (if you haven't already created them):
   ```bash
   diesel setup
   diesel migration run
   ```

### 4. Run the Application

To run the application, use the following command:

```bash
cargo run
```

The API will be available at `http://127.0.0.1:8080`.

### 5. Testing the Endpoints

You can test the API using tools like **Postman** or **cURL**.

#### Health Check Endpoint

- **URL**: `http://127.0.0.1:8080/health`
- **Method**: `GET`
- **Response**: `Server is healthy`

#### Login Endpoint

- **URL**: `http://127.0.0.1:8080/login`
- **Method**: `POST`
- **Body**:
  ```json
  {
    "username": "testuser",
    "password": "password123"
  }
  ```
- **Response**: A JSON response with user data on successful login or `401 Unauthorized` on failure.

#### Samples CRUD Endpoints

1. **Create a Sample**:
   - **URL**: `http://127.0.0.1:8080/samples`
   - **Method**: `POST`
   - **Body**:
     ```json
     {
       "name": "Tumor Sample",
       "sample_type": "Tissue",
       "collected_at": "2025-01-30T12:00:00",
       "status": "Processing"
     }
     ```
   - **Response**: The created sample data with `201 Created` status.

2. **Get All Samples**:
   - **URL**: `http://127.0.0.1:8080/samples`
   - **Method**: `GET`
   - **Response**: A list of all samples.

3. **Get a Sample by ID**:
   - **URL**: `http://127.0.0.1:8080/samples/{id}`
   - **Method**: `GET`
   - **Response**: The sample data for the provided ID, or `404 Not Found` if the sample does not exist.

4. **Delete a Sample by ID**:
   - **URL**: `http://127.0.0.1:8080/samples/{id}`
   - **Method**: `DELETE`
   - **Response**: `Sample deleted` or `404 Not Found` if the sample does not exist.

## Contributing

If you would like to contribute to this project, please fork the repository, create a new branch, make your changes, and then create a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- **Actix Web**: The web framework for building fast APIs in Rust.
- **Diesel**: A safe and extensible ORM and query builder for Rust.
- **bcrypt**: Password hashing library for Rust.
```

### Key Sections in the README:

1. **Technologies Used**: This section lists the tools and frameworks used in the project (Rust, Actix Web, Diesel, bcrypt, etc.).
2. **Getting Started**: It guides the user through setting up the environment, installing dependencies, configuring the database, and running the server.
3. **Testing the Endpoints**: This section provides Postman or cURL examples for testing the health check, login, and sample CRUD operations.
4. **Contributing**: A brief note on how to contribute to the project.
5. **License**: Information about the licensing (MIT in this case).
