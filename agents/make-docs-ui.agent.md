---
model: claude-sonnet-4-5-20250929
tools: Read, Write, Grep, Glob
description: Generate UI_SPEC and TESTING_STRATEGY (frontend and quality docs)
---

# UI & Testing Documentation Generator (AI-Optimized)

Generate frontend specification and testing strategy that AI agents need to implement UI and tests.

## Documentation Suite Context

Related docs (ALREADY EXIST):
- **BUILD_SPEC.md** - Tech stack, requirements
- **ARCHITECTURE.md** - System design, component relationships
- **API_SPEC.md** - Backend endpoints to call

Read these first to understand the system.

## Your Task

Generate 2 files in `.autoflow/docs/`:

### 1. UI_SPEC.md

**What to include:**

**1. User Flows**
- ASCII diagrams for each major flow
- Multi-tenant flows (if applicable)
- Error states and recovery
- Success states

**2. Pages/Views**
For EACH page:
- Route (including tenant slug if multi-tenant)
- Purpose
- ASCII wireframe
- Components used
- Props/state needed
- API calls (reference API_SPEC.md)
- Authentication requirements

**3. Component Hierarchy**
- Component tree diagram
- Reusable vs page-specific components
- Props interfaces

**4. Design System**
- Colors (hex codes for primary, secondary, semantic)
- Typography (fonts, sizes, weights)
- Spacing scale
- **Theming system** (if applicable):
  - Theme structure
  - How themes applied (CSS vars, styled-components)
  - Theme switching mechanism
  - Per-user/per-tenant preferences

**5. Responsive Design**
- Breakpoints (mobile, tablet, desktop)
- Mobile-first approach
- Touch targets (44x44px minimum)

**6. Accessibility (WCAG 2.1 AA)**
- Semantic HTML requirements
- ARIA labels for all interactive elements
- Keyboard navigation (tab order, shortcuts)
- Screen reader support
- Color contrast ratios
- Focus management

**7. State Management**
- Global vs local state decisions
- Library (Redux/Zustand/Context)
- **Data fetching** (React Query/SWR patterns)
- **Cache strategies** (stale-while-revalidate)
- **Form handling** (React Hook Form/Formik)
- URL state sync
- State persistence (localStorage)

### 2. TESTING_STRATEGY.md

**What to include:**

**1. Testing Pyramid**
```
     E2E (10%)
    ───────────
   Integration (20%)
  ─────────────────
 Unit Tests (70%)
```

**2. Framework Choices**
- Unit: Vitest/Jest with React Testing Library
- E2E: Playwright/Cypress
- Architecture tests: Pest (if Laravel)
- Rationale for each

**3. Coverage Requirements**
- Overall: 80% minimum
- Critical paths: 100% (auth, payments, data mutations)
- Business logic: 95%
- UI components: 80%

**4. What to Test**
- **Unit**: Business logic, utilities, components, hooks
- **Integration**: API endpoints, database ops, auth flows
- **E2E**: Critical user flows (registration → login → main workflow)
- **Architecture** (if Pest): Layer boundaries, naming conventions

**5. Multi-Tenant Testing** (if applicable)
- Tenant isolation tests
- Cross-tenant data access prevention

**6. Testing Patterns**
- Mock strategies (when to mock external APIs)
- Test data factories
- Setup/teardown patterns
- CI/CD integration

## Quality Standards

- **Concise**: Only what AI needs to write code/tests
- **Actionable**: Clear specs, not vague descriptions
- **Cross-reference**: Link to API_SPEC for endpoints
- **No deployment**: No CI/CD infrastructure details

## Output Files

Create in `.autoflow/docs/`:
- `UI_SPEC.md` (1500-2000 lines - includes state management)
- `TESTING_STRATEGY.md` (800-1200 lines)

Start now - read existing docs, then generate UI and testing specifications.
