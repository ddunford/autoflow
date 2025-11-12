# Claude Code Configuration

## Project Structure

**Code Root**: `./src/`
**Test Root**: `./tests/`
**Config Root**: `./`

## Project Guidelines

### Code Quality
- DRY (Don't Repeat Yourself)
- SOLID principles
- Test coverage ≥80%
- OWASP Top 10 compliance

### Testing Strategy
- TDD workflow (RED → GREEN → REFACTOR)
- Unit tests for business logic
- Integration tests for API endpoints
- E2E tests for user journeys

### Conventions
- Clear, descriptive naming
- Document complex logic
- Handle errors explicitly
- Log important events

## AutoFlow Integration

This project is managed by AutoFlow. Sprints are defined in `.autoflow/SPRINTS.yml`.

Do not manually edit sprint status or task completion. The AutoFlow orchestrator handles all state transitions.
