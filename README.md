<a id="top"></a>

<p align="center">
  <h1 align="center">revelation-user</h1>
  <p align="center">
    <strong>Professional user domain for Rust web applications</strong>
  </p>
  <p align="center">
    Authentication, authorization, and user management — batteries included
  </p>
</p>

<p align="center">
  <a href="https://crates.io/crates/revelation-user">
    <img src="https://img.shields.io/crates/v/revelation-user.svg?style=for-the-badge" alt="Crates.io"/>
  </a>
  <a href="https://docs.rs/revelation-user">
    <img src="https://img.shields.io/docsrs/revelation-user?style=for-the-badge" alt="Documentation"/>
  </a>
</p>

<p align="center">
  <a href="https://github.com/Revelation-Path/revelation-user/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/Revelation-Path/revelation-user/ci.yml?style=for-the-badge" alt="CI Status"/>
  </a>
  <a href="https://codecov.io/gh/Revelation-Path/revelation-user">
    <img src="https://img.shields.io/codecov/c/github/Revelation-Path/revelation-user?style=for-the-badge" alt="Coverage"/>
  </a>
</p>

<p align="center">
  <a href="https://github.com/Revelation-Path/revelation-user/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License: MIT"/>
  </a>
</p>

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Types](#core-types)
- [Framework Integration](#framework-integration)
- [Extending Users](#extending-users)
- [RBAC Permissions](#rbac-permissions)

---

## Features

- **Multiple Auth Methods** — Telegram, Email, Phone authentication out of the box
- **JWT Claims** — Type-safe claims with expiration and role checking
- **RBAC Permissions** — Fine-grained permission system with bitflags
- **Framework Extractors** — Zero-boilerplate auth for Axum and Actix-web
- **Projections** — Type-safe views that prevent accidental data leaks
- **Extensible** — `extend_user!` macro for application-specific fields
- **Entity-Derive Integration** — Auto-generated DTOs via [entity-derive](https://crates.io/crates/entity-derive)

<div align="right"><a href="#top">⬆ back to top</a></div>

## Installation

```toml
[dependencies]
revelation-user = { version = "0.1", features = ["axum"] }
```

### Available Features

| Feature | Description |
|---------|-------------|
| `postgres` | PostgreSQL support via sqlx |
| `api` | OpenAPI schema generation via utoipa |
| `validate` | Validation derives via validator |
| `axum` | Axum framework extractors |
| `actix` | Actix-web framework extractors |

> **Note**: `axum` and `actix` features are mutually exclusive.

<div align="right"><a href="#top">⬆ back to top</a></div>

## Quick Start

```rust
use revelation_user::{RUser, Claims, RUserRole};

// Create user from authentication
let user = RUser::from_telegram(123456789);
let user = RUser::from_email("user@example.com");
let user = RUser::from_phone("+14155551234");

// Create JWT claims
let exp = (chrono::Utc::now().timestamp() + 3600) as usize;
let claims = Claims::new(user.id, RUserRole::User, exp);

// Check permissions
assert!(!claims.is_expired());
assert!(!claims.is_admin());
```

<div align="right"><a href="#top">⬆ back to top</a></div>

## Core Types

### Entity

| Type | Description |
|------|-------------|
| `RUser` | Core user entity with all fields |
| `Claims` | JWT claims for authentication |

### Projections

| Type | Description |
|------|-------------|
| `RUserPublic` | Safe for API responses (excludes sensitive data) |
| `RUserAuth` | For JWT/session context (includes role) |

### DTOs

| Type | Description |
|------|-------------|
| `CreateUserRequest` | Create new user |
| `UpdateProfileRequest` | Update user profile |
| `BindTelegram` | Bind Telegram account |
| `BindEmail` | Bind email address |
| `BindPhone` | Bind phone number |

<div align="right"><a href="#top">⬆ back to top</a></div>

## Framework Integration

### Axum

```rust
use axum::{Router, Json, routing::get};
use revelation_user::{Claims, RUserPublic};

async fn me(claims: Claims) -> Json<RUserPublic> {
    // Claims extracted from JWT cookie or Authorization header
    todo!()
}

let app = Router::new().route("/me", get(me));
```

### Actix-web

```rust
use actix_web::{web, HttpResponse};
use revelation_user::Claims;

async fn me(claims: Claims) -> HttpResponse {
    HttpResponse::Ok().json(claims.user_id())
}
```

<div align="right"><a href="#top">⬆ back to top</a></div>

## Extending Users

Use the `extend_user!` macro for application-specific fields:

```rust
use revelation_user::{extend_user, RUser};
use uuid::Uuid;

extend_user! {
    /// Corporate user with company-specific fields.
    pub struct CorpUser {
        pub company_id: Uuid,
        pub department: String,
    }
}

let user = CorpUser::from_telegram(123456789)
    .name("John")
    .then()
    .company_id(Uuid::now_v7())
    .department("Engineering")
    .build();

// Access RUser fields via Deref
assert!(user.telegram_id.is_some());
```

<div align="right"><a href="#top">⬆ back to top</a></div>

## RBAC Permissions

Fine-grained permission system:

```rust
use revelation_user::{Permissions, RUserRole, Role};

// Check role permissions
let role = RUserRole::Premium;
assert!(role.can(Permissions::READ));
assert!(role.can(Permissions::WRITE));

// Admin has all permissions
let admin = RUserRole::Admin;
assert!(admin.can(Permissions::all()));

// Custom permission checks in Claims
let claims = Claims::new(uuid::Uuid::now_v7(), RUserRole::User, exp);
assert!(claims.can(Permissions::READ));
```

### Permission Flags

| Permission | Description |
|------------|-------------|
| `READ` | Read access |
| `WRITE` | Write access |
| `DELETE` | Delete access |
| `ADMIN` | Administrative access |

<div align="right"><a href="#top">⬆ back to top</a></div>

## Code Coverage

<p align="center">
  <a href="https://codecov.io/gh/Revelation-Path/revelation-user">
    <img src="https://codecov.io/gh/Revelation-Path/revelation-user/graphs/sunburst.svg?token=YOUR_TOKEN" alt="Coverage Sunburst"/>
  </a>
</p>

<div align="right"><a href="#top">⬆ back to top</a></div>

## License

MIT
