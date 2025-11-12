---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Grep, Glob
description: Generate API_SPEC (includes data model and security implementation)
---

# API Documentation Generator (AI-Optimized)

Generate complete backend/API specification that AI agents need to implement APIs and data layers.

## Documentation Suite Context

Related docs (ALREADY EXIST):
- **BUILD_SPEC.md** - Tech stack, requirements
- **ARCHITECTURE.md** - System design, patterns, error handling strategy

Read these first to understand the system.

## Your Task

Generate 1 comprehensive file in `.autoflow/docs/`:

### API_SPEC.md

**What to include:**

**1. API Overview**
- Base URL and versioning strategy
- Authentication mechanism (JWT, OAuth, API keys)
- Common headers and request patterns

**2. All Endpoints**
For EACH endpoint:
- Method, path, description
- Authentication required (yes/no, what roles)
- Request parameters (query, path, body with JSON schema)
- Response schema (success with status code)
- Error responses (all possible errors with codes)
- Example request/response
- Rate limits

**3. Data Model**
- Database choice and rationale
- ER diagram (ASCII/Mermaid)
- Complete schema for ALL tables/collections:
  - Table name, description
  - All columns (name, type, constraints, defaults)
  - Primary keys, foreign keys, indexes
  - Validation rules
- Multi-tenant data isolation (if applicable)
- Sample data for development

**4. Security Implementation**
- Authentication flow diagram
- Authorization patterns (RBAC/ABAC)
- Password hashing (bcrypt/argon2)
- JWT token generation/validation
- Input validation (library, patterns)
- SQL injection prevention (parameterized queries)
- XSS prevention (sanitization, CSP headers)
- CSRF protection
- Rate limiting implementation
- Secrets management (env vars, never commit)
- Security headers (HSTS, X-Frame-Options)

**5. Common Patterns**
- Pagination (offset/limit or cursor)
- Sorting and filtering
- Batch operations
- WebSocket/real-time (if applicable)

## Quality Standards

- **Complete**: Every endpoint, every table
- **Realistic examples**: Actual data, not placeholders
- **Security-focused**: How to implement safely
- **No deployment content**: No Docker, CI/CD, infrastructure

## Cross-References

- Reference ARCHITECTURE.md for error handling patterns
- Reference BUILD_SPEC.md for tech stack
- Will be referenced by UI_SPEC.md for API calls

## Output File

Create in `.autoflow/docs/`:
- `API_SPEC.md` (2000-3000 lines - comprehensive, includes data model and security)

Start now - read BUILD_SPEC.md and ARCHITECTURE.md, then generate complete API documentation.
