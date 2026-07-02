# IAM Platform (Rust)

## About

This is a simple Identity and Access Management (IAM) backend built using **Rust**, **Axum**, **PostgreSQL**, and **SQLx**.

The project allows users to register, log in, create organizations, manage roles and permissions, and authorize users using Role-Based Access Control (RBAC).

---

## Features

* User Registration
* User Login
* Password Hashing using Argon2
* JWT Authentication
* Session Management
* Protected Routes using Middleware
* Organization Management
* Role Management
* Permission Management
* Membership Management
* Role-Based Access Control (RBAC)
* Permission Checking using `has_permission()`

---

## Tech Stack

* Rust
* Axum
* PostgreSQL
* SQLx
* JWT
* Argon2

---

## Project Structure

```
src/
│
├── handlers/
├── services/
├── repositories/
├── models/
├── middleware/
├── auth/
├── routes/
├── config/
└── main.rs
```

---

## Authentication Flow

1. User registers.
2. Password is hashed using Argon2.
3. User logs in.
4. A session is created.
5. A JWT token is generated.
6. Protected routes verify the JWT using middleware.
7. Session validity is checked before allowing access.

---

## Authorization Flow

* A user creates an organization.
* An Owner role is automatically created.
* All permissions are assigned to the Owner role.
* The creator is added as the Owner member.
* Before performing protected actions, the backend checks whether the user has the required permission using `has_permission()`.

---

## Database Tables

* users
* sessions
* organizations
* roles
* permissions
* role_permissions
* memberships

---

## How to Run

1. Create a PostgreSQL database.
2. Set the database URL in the environment.
3. Run migrations:

```
sqlx migrate run
```

4. Start the server:

```
cargo run
```

The server runs on:

```
http://127.0.0.1:3000
```

---

## Author

KV Ramana
