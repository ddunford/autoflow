---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Grep, Glob
description: Generate BUILD_SPEC and ARCHITECTURE (foundation docs for AI coding)
---

# Foundation Documentation Generator (AI-Optimized)

Generate concise, actionable documentation that AI agents need to write code. NO deployment/ops content.

## Your Task

Generate 2 foundational files in `.autoflow/docs/`:

### 1. BUILD_SPEC.md

**What to include:**
- Project goals and success criteria
- Complete tech stack with versions
- All features with acceptance criteria
- Non-functional requirements (performance, security, accessibility targets)
- Architecture constraints

**What to EXCLUDE:**
- Deployment steps (not needed for coding)
- Infrastructure details (not needed for coding)
- CI/CD pipelines (not needed for coding)

### 2. ARCHITECTURE.md

**What to include:**
- Component diagram (ASCII/Mermaid)
- Component responsibilities and relationships
- Data flow diagrams
- Design patterns to use (Repository, Factory, etc.)
- **Error handling strategy** (patterns, how errors propagate)
- **Security architecture** (auth flow, encryption, OWASP mitigations)
- Multi-tenancy architecture (if applicable)
- Integration points between components

**What to EXCLUDE:**
- Docker/Kubernetes configs
- Server provisioning
- Deployment procedures

## Quality Standards

- **Concise**: Only information needed to write code
- **Actionable**: Developers know exactly what to build
- **No duplication**: Don't repeat what's in other docs
- **Cross-reference**: Link to API_SPEC, UI_SPEC when relevant

## Output Files

Create in `.autoflow/docs/`:
- `BUILD_SPEC.md` (800-1200 lines)
- `ARCHITECTURE.md` (600-900 lines, includes error handling and security patterns)

Start now - read IDEA.md context and generate both files.
