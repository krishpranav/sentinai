# Sentinai API Reference

## Base URL

`http://localhost:8080` (Development)  

---

# AUTH DOCUMENTATION

Sentinai secures endpoints using **JSON Web Tokens (JWT)**.
To access protected routes, you must include the JWT in the `Authorization` header of your HTTP request.

**Authorization header format:**
```http
Authorization: Bearer <your_jwt_token_here>
```

### Example Login Request
```bash
curl -X POST http://localhost:8080/auth/github \
  -H "Content-Type: application/json" \
  -d '{"code": "mock-token"}'
```

### Example JWT Response
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "github_id": 123456,
    "username": "mock_user",
    "email": "mock@example.com"
  }
}
```

### Example Protected Endpoint Request
```bash
curl -X GET http://localhost:8080/projects \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

# ERROR FORMAT

All API errors return a standard JSON structure to help you programmatically handle failures.

```json
{
  "error": "A human-readable error message",
  "code": "ERROR_CODE",
  "request_id": "123e4567-e89b-12d3-a456-426614174000"
}
```

### Common Error Codes & Examples

**400 Bad Request**
```json
{
  "error": "Invalid request payload",
  "code": "BAD_REQUEST",
  "request_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479"
}
```

**401 Unauthorized**
```json
{
  "error": "Missing or invalid token",
  "code": "UNAUTHORIZED",
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**404 Not Found**
```json
{
  "error": "Project not found",
  "code": "NOT_FOUND",
  "request_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
}
```

**500 Internal Server Error**
```json
{
  "error": "An unexpected infrastructure error occurred",
  "code": "INTERNAL_ERROR",
  "request_id": "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
}
```

---

## Health

### GET /health

**Authentication:** No  
**Description:** Check the health status of the backend, database connections, and memory state.

#### Response (200)

```json
{
  "status": "ok"
}
```

#### curl

```bash
curl http://localhost:8080/health
```

---

## Auth

### POST /auth/login

**Authentication:** No  
**Description:** Traditional email/password login (simulated placeholder). Used primarily for legacy client support.

#### Request Body
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

#### Response (200)
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5c...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "github_id": 0,
    "username": "demo_user",
    "email": "user@example.com"
  }
}
```

#### curl
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com", "password":"securepassword123"}'
```

---

### POST /auth/github

**Authentication:** No  
**Description:** Authenticate using a GitHub OAuth 2.0 authorization code. Returns a Sentinai session JWT.

#### Request Body
```json
{
  "code": "gho_xxxxxx_mock_token_xxxxxx"
}
```

#### Response (200)
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5c...",
  "user": {
    "id": "dd11aa33-e89b-12d3-a456-426614174000",
    "github_id": 12345678,
    "username": "github_user",
    "email": "user@github.com"
  }
}
```

#### curl
```bash
curl -X POST http://localhost:8080/auth/github \
  -H "Content-Type: application/json" \
  -d '{"code":"mock-token"}'
```

---

### GET /auth/me

**Authentication:** Yes  
**Description:** Retrieve the profile of the currently authenticated user payload stored within the provided JWT.

#### Response (200)
```json
{
  "id": "dd11aa33-e89b-12d3-a456-426614174000",
  "github_id": 12345678,
  "username": "github_user",
  "email": "user@github.com",
  "created_at": "2026-02-22T10:00:00Z"
}
```

#### curl
```bash
curl -X GET http://localhost:8080/auth/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

## Projects

### POST /projects

**Authentication:** Yes  
**Description:** Create a new Sentinai project linked to a remote Git repository.

#### Request Body
```json
{
  "name": "sentinai-core",
  "repository_url": "https://github.com/org/sentinai-core"
}
```

#### Response (200)
```json
{
  "id": "a1b2c3d4-e89b-12d3-a456-426614174000",
  "user_id": "dd11aa33-e89b-12d3-a456-426614174000",
  "name": "sentinai-core",
  "repository_url": "https://github.com/org/sentinai-core",
  "created_at": "2026-02-22T10:05:00Z"
}
```

#### curl
```bash
curl -X POST http://localhost:8080/projects \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..." \
  -H "Content-Type: application/json" \
  -d '{"name": "sentinai-core", "repository_url": "https://github.com/org/sentinai-core"}'
```

---

### GET /projects

**Authentication:** Yes  
**Description:** Retrieve a list of all projects owned by the authenticated user.

#### Response (200)
```json
[
  {
    "id": "a1b2c3d4-e89b-12d3-a456-426614174000",
    "user_id": "dd11aa33-e89b-12d3-a456-426614174000",
    "name": "sentinai-core",
    "repository_url": "https://github.com/org/sentinai-core",
    "created_at": "2026-02-22T10:05:00Z"
  }
]
```

#### curl
```bash
curl -X GET http://localhost:8080/projects \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

### GET /projects/:id

**Authentication:** Yes  
**Description:** Retrieve detailed information regarding a specific project.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
{
  "id": "a1b2c3d4-e89b-12d3-a456-426614174000",
  "user_id": "dd11aa33-e89b-12d3-a456-426614174000",
  "name": "sentinai-core",
  "repository_url": "https://github.com/org/sentinai-core",
  "created_at": "2026-02-22T10:05:00Z"
}
```

#### curl
```bash
curl -X GET http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

### DELETE /projects/:id

**Authentication:** Yes  
**Description:** Delete a project and cascade all related pipelines, builds, and findings.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
{}
```

#### curl
```bash
curl -X DELETE http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

## Pipelines

### POST /projects/:id/generate-ci

**Authentication:** Yes  
**Description:** Analyzes the linked repository, generates an optimized CI/CD pipeline YAML configuration, persists it to the database, and emits a real-time SSE event.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
{
  "id": "bb22cc44-e89b-12d3-a456-426614174000",
  "project_id": "a1b2c3d4-e89b-12d3-a456-426614174000",
  "yaml_config": "name: Rust Builders\n\non:\n  push:\n    branches: [ main ]\n\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v2\n      - run: cargo build --release\n",
  "created_at": "2026-02-22T10:15:00Z"
}
```

#### curl
```bash
curl -X POST http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000/generate-ci \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

### GET /projects/:id/pipelines

**Authentication:** Yes  
**Description:** Retrieve all generated CI pipelines associated with a project.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
[
  {
    "id": "bb22cc44-e89b-12d3-a456-426614174000",
    "project_id": "a1b2c3d4-e89b-12d3-a456-426614174000",
    "yaml_config": "name: Rust Builders...",
    "created_at": "2026-02-22T10:15:00Z"
  }
]
```

#### curl
```bash
curl -X GET http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000/pipelines \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

## Security

### POST /projects/:id/security/scan

**Authentication:** Yes  
**Description:** Execute a deterministic security scan against the remote code framework. Creates new findings iteratively and emits real-time SSE events for each finding discovered.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
[
  {
    "id": "cc33dd55-e89b-12d3-a456-426614174000",
    "project_id": "a1b2c3d4-e89b-12d3-a456-426614174000",
    "severity": "high",
    "description": "Detected outdated OpenSSL dependency in Cargo.lock",
    "resolved": false,
    "created_at": "2026-02-22T10:20:00Z"
  },
  {
    "id": "dd44ee66-e89b-12d3-a456-426614174000",
    "project_id": "a1b2c3d4-e89b-12d3-a456-426614174000",
    "severity": "medium",
    "description": "Missing security headers in API response",
    "resolved": false,
    "created_at": "2026-02-22T10:20:01Z"
  }
]
```

#### curl
```bash
curl -X POST http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000/security/scan \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

### GET /projects/:id/security

**Authentication:** Yes  
**Description:** Retrieve all historical security findings associated with a project.
**Path Parameters:**
- `id` (UUID): The unique identifier of the project.

#### Response (200)
```json
[
  {
    "id": "cc33dd55-e89b-12d3-a456-426614174000",
    "project_id": "a1b2c3d4-e89b-12d3-a456-426614174000",
    "severity": "high",
    "description": "Detected outdated OpenSSL dependency in Cargo.lock",
    "resolved": false,
    "created_at": "2026-02-22T10:20:00Z"
  }
]
```

#### curl
```bash
curl -X GET http://localhost:8080/projects/a1b2c3d4-e89b-12d3-a456-426614174000/security \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5c..."
```

---

## Webhooks

### POST /webhooks/github

**Authentication:** No (Uses cryptographic signature payload validation `x-hub-signature-256`)  
**Description:** Receives push and PR events directly from GitHub to track remote code modifications and trigger downstream Sentinai CI builds.

**Headers Required:**
- `x-hub-signature-256`: sha256=...
- `x-github-event`: push

#### Request Body
```json
{
  "ref": "refs/heads/main",
  "repository": {
    "name": "sentinai-core",
    "full_name": "org/sentinai-core",
    "id": 12345678
  },
  "commits": [
    {
      "id": "1a2b3c4d5e",
      "message": "Update memory bounds",
      "author": {
        "name": "Octocat"
      }
    }
  ]
}
```

#### Response (200)
```json
{
  "status": "Webhook acknowledged and processed"
}
```

#### curl
```bash
curl -X POST http://localhost:8080/webhooks/github \
  -H "Content-Type: application/json" \
  -H "x-hub-signature-256: sha256=fb511f621877607a783..." \
  -H "x-github-event: push" \
  -d '{ "ref": "refs/heads/main", "repository": { "name": "sentinai-core" } }'
```

---

# TESTING GUIDE

Use this testing guide to quickly iterate through standard API verification loops locally.

### Start Backend Locally
```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/sentinai
export JWT_SECRET=super_secret_local_key
export PORT=8080

cargo run
```

### Create a mock user & get token
```bash
# Obtain your JWT token via mock route
curl -X POST http://localhost:8080/auth/github \
  -H "Content-Type: application/json" \
  -d '{"code": "mock-token"}'
```

Extract the `token` string and export it in your shell environment:
```bash
export TOKEN="eyJhbGciOiJIUzI1NiIsInR..."
```

### Validate Authorization Header
```bash
curl http://localhost:8080/auth/me \
  -H "Authorization: Bearer $TOKEN"
```

### Example: Invalid JWT Test (401 Authorization Error)
```bash
curl -i http://localhost:8080/projects \
  -H "Authorization: Bearer random.invalid.garbagestring"

# Expected Output:
# HTTP/1.1 401 Unauthorized
# {"error": "Invalid or missing JWT token", "code": "UNAUTHORIZED", "request_id": "..."}
```

### Example: Invalid UUID format on Path Parameter (400 Bad Request)
```bash
curl -i http://localhost:8080/projects/not-a-uuid \
  -H "Authorization: Bearer $TOKEN"

# Expected Output:
# HTTP/1.1 400 Bad Request
# {"error": "Invalid URL parameter format", "code": "BAD_REQUEST", "request_id": "..."}
```

### Example: Webhook Signature Test
Simulate a GitHub webhook with an empty or simulated secret logic:
```bash
curl -i -X POST http://localhost:8080/webhooks/github \
  -H "Content-Type: application/json" \
  -H "x-hub-signature-256: sha256=invalidhash" \
  -H "x-github-event: push" \
  -d '{"ref":"refs/heads/main"}'

# Expected Output:
# HTTP/1.1 401 Unauthorized
# {"error": "Invalid webhook signature validation", "code": "UNAUTHORIZED", "request_id": "..."}
```
