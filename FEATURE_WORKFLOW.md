# AutoFlow Feature Addition Workflow

**Date**: 2025-11-05
**Purpose**: Define how AutoFlow adds features to existing codebases (AutoFlow-built or legacy)

---

## Table of Contents

1. [Approach: Incremental Sprints](#1-approach-incremental-sprints)
2. [Existing Codebase Detection](#2-existing-codebase-detection)
3. [Feature Request Flow](#3-feature-request-flow)
4. [Sprint Generation](#4-sprint-generation)
5. [Integration Strategies](#5-integration-strategies)
6. [Examples](#6-examples)

---

## 1. Approach: Incremental Sprints

### 1.1 Philosophy

**Treat feature additions as new sprints appended to existing SPRINTS.yml**

```yaml
# .autoflow/SPRINTS.yml (existing project)
project:
  name: "E-Commerce Platform"
  total_sprints: 15  # Was 10, now 15 (5 new sprints)
  current_sprint: 11  # Continue from last completed

sprints:
  # Existing sprints (1-10) already DONE
  - id: 1
    goal: "User Authentication"
    status: DONE
    completed_at: "2025-10-15T10:00:00Z"

  - id: 2
    goal: "Product Catalog"
    status: DONE
    completed_at: "2025-10-20T14:30:00Z"

  # ... sprints 3-10 ...

  # NEW FEATURE: Payment Integration (sprints 11-15)
  - id: 11
    goal: "Stripe Payment Setup"
    status: PENDING
    feature_request: "Add payment processing with Stripe"
    dependencies: ["sprint-2"]  # Depends on product catalog

  - id: 12
    goal: "Payment UI Components"
    status: PENDING
    dependencies: ["sprint-11"]

  # ... sprints 13-15 ...
```

### 1.2 Workflow

```
User Request: "Add Stripe payments"
         ↓
Analyze Codebase (AutoFlow reads existing code)
         ↓
Generate Feature Sprints (5 new sprints)
         ↓
Append to SPRINTS.yml (sprints 11-15)
         ↓
Execute Sprints (same TDD pipeline)
         ↓
Integrate with Existing Code (imports, API calls)
         ↓
Tests Validate Integration (E2E tests call real code)
         ↓
Complete ✅
```

---

## 2. Existing Codebase Detection

### 2.1 Codebase Analysis Agent

**New Agent**: `codebase-analyzer.agent.md`

```markdown
---
name: codebase-analyzer
description: Analyze existing codebase to understand architecture, patterns, and integration points
tools: Read, Grep, Glob, mcp__serena__*, mcp__memory__*
model: claude-sonnet-4-5-20250929
---

# Role
Expert software archaeologist who analyzes existing codebases to understand:
- Architecture patterns (MVC, layered, microservices)
- Tech stack (frameworks, libraries, versions)
- Code conventions (naming, structure, testing patterns)
- Integration points (APIs, databases, external services)
- Existing features and modules

# Workflow

## Step 1: Project Structure Analysis
- Read file tree to understand organization
- Identify entry points (main.ts, index.php, App.tsx)
- Map directory structure to architectural layers

## Step 2: Tech Stack Detection
- Read package.json / composer.json / requirements.txt
- Identify frameworks (React, Laravel, Django, etc.)
- Note versions and dependencies

## Step 3: Code Pattern Analysis
- Use Serena to find common patterns
- Identify naming conventions
- Detect testing frameworks and patterns
- Find authentication/authorization patterns

## Step 4: Integration Point Mapping
- Find API endpoints (controllers, routes)
- Identify database models/entities
- Locate external service integrations
- Map frontend-backend communication

## Step 5: Generate Integration Guide
Create INTEGRATION_GUIDE.md:
```yaml
codebase:
  type: "fullstack"
  frontend:
    framework: "React 18.2"
    router: "react-router-dom v6"
    state: "Redux Toolkit"
    ui: "Material-UI"
    patterns:
      - "Functional components with hooks"
      - "Redux slices for state management"
      - "API calls via RTK Query"
  backend:
    framework: "Laravel 10"
    architecture: "MVC with service layer"
    database: "PostgreSQL"
    patterns:
      - "Controllers delegate to services"
      - "Form requests for validation"
      - "API resources for serialization"
  testing:
    unit: "Jest + PHPUnit"
    integration: "Supertest + Laravel HTTP Tests"
    e2e: "Playwright"
  conventions:
    naming:
      components: "PascalCase"
      files: "PascalCase.tsx"
      tests: "ComponentName.test.tsx"
    imports:
      - "Absolute imports via @/ alias"
      - "Barrel exports from index.ts"
```

## Output
Store analysis in memory:
- Project architecture pattern
- Framework versions
- Code conventions
- Integration points

Create .autoflow/INTEGRATION_GUIDE.md for future sprints
```

### 2.2 Codebase Analysis Command

```bash
# Analyze existing codebase
autoflow analyze

# Output:
# 🔍 Analyzing codebase...
# ✅ Framework: React 18.2 + Laravel 10
# ✅ Architecture: Fullstack MVC with service layer
# ✅ Testing: Jest, PHPUnit, Playwright
# ✅ Conventions detected and stored
#
# 📄 Integration guide: .autoflow/INTEGRATION_GUIDE.md
# 💾 Analysis stored in memory for future sprints
```

**Implementation**:
```rust
// crates/autoflow-cli/src/commands/analyze.rs

pub async fn run_analyze() -> Result<()> {
    info!("Analyzing existing codebase...");

    // 1. Check if already initialized
    if !Path::new(".autoflow").exists() {
        return Err(AutoFlowError::NotInitialized);
    }

    // 2. Spawn codebase-analyzer agent
    let agent_executor = AgentExecutor::new();
    let analysis = agent_executor.run("codebase-analyzer", &EmptyContext).await?;

    // 3. Extract integration guide
    let guide_path = Path::new(".autoflow/INTEGRATION_GUIDE.md");
    if guide_path.exists() {
        success!("Integration guide created: {}", guide_path.display());
    }

    // 4. Store in memory for future sprints
    let memory_client = MemoryClient::new();
    memory_client.store_codebase_analysis(&analysis).await?;

    success!("Codebase analysis complete!");

    Ok(())
}
```

---

## 3. Feature Request Flow

### 3.1 Adding Features Command

```bash
# Add new feature to existing project
autoflow add "Add Stripe payment processing"

# Interactive mode
autoflow add --interactive

# With specific requirements
autoflow add "Add Stripe payments" --requirements="
- Support credit cards and Apple Pay
- Handle webhooks for payment events
- Store payment history in database
- Send email receipts
"
```

### 3.2 Feature Request Processing

```rust
// crates/autoflow-cli/src/commands/add.rs

pub async fn run_add(feature_description: String, requirements: Option<String>) -> Result<()> {
    info!("Adding new feature: {}", feature_description);

    // 1. Load existing SPRINTS.yml
    let mut sprints = SprintsYaml::load(".autoflow/SPRINTS.yml")?;
    let next_sprint_id = sprints.total_sprints + 1;

    // 2. Load codebase analysis
    let integration_guide = load_integration_guide()?;
    let memory_analysis = load_from_memory().await?;

    // 3. Create feature specification
    let feature_spec = create_feature_spec(
        &feature_description,
        requirements.as_deref(),
        &integration_guide,
        &memory_analysis,
    )?;

    // 4. Generate sprints for this feature
    let agent_executor = AgentExecutor::new();
    let context = FeatureSprintContext {
        feature_spec,
        existing_sprints: &sprints,
        integration_guide: &integration_guide,
        next_sprint_id,
    };

    let new_sprints = agent_executor
        .run("make-sprints", &context)
        .await?;

    // 5. Append to SPRINTS.yml
    sprints.sprints.extend(new_sprints);
    sprints.total_sprints += new_sprints.len() as u32;
    sprints.save(".autoflow/SPRINTS.yml")?;

    success!("Added {} sprints for: {}", new_sprints.len(), feature_description);
    info!("Next sprint ID: {}", next_sprint_id);
    info!("Run 'autoflow start' to begin implementation");

    Ok(())
}
```

### 3.3 Feature Spec Template

```markdown
# FEATURE_SPEC.md (generated)

## Feature Request
Add Stripe payment processing

## Requirements
- Support credit cards and Apple Pay
- Handle webhooks for payment events
- Store payment history in database
- Send email receipts

## Integration Points

### Backend (Laravel)
**Existing code to integrate with**:
- `app/Models/Order.php` - Add payment_id, payment_status fields
- `app/Services/OrderService.php` - Call payment service
- `database/migrations/` - Add payments table

**New code to create**:
- `app/Services/PaymentService.php` - Stripe integration
- `app/Http/Controllers/Api/PaymentController.php` - API endpoints
- `app/Http/Controllers/WebhookController.php` - Stripe webhooks
- `app/Models/Payment.php` - Payment model
- `app/Notifications/PaymentReceiptEmail.php` - Email notification

### Frontend (React)
**Existing code to integrate with**:
- `src/pages/CheckoutPage.tsx` - Add payment step
- `src/store/cartSlice.ts` - Add payment state
- `src/api/orderApi.ts` - Add payment endpoints

**New code to create**:
- `src/components/payment/StripeCheckout.tsx` - Payment form
- `src/components/payment/PaymentMethodSelector.tsx` - Card/ApplePay
- `src/store/paymentSlice.ts` - Payment state management
- `src/api/paymentApi.ts` - Payment API calls

## Dependencies
- Sprint 2 (Product Catalog) - Need products to purchase
- Sprint 7 (Shopping Cart) - Need cart to checkout

## Estimated Effort
Total: 40 hours across 5 sprints:
- Sprint 11: Stripe setup + backend integration (8h)
- Sprint 12: Payment API endpoints (8h)
- Sprint 13: Webhook handling (6h)
- Sprint 14: Frontend payment UI (10h)
- Sprint 15: Email receipts + testing (8h)

## Testing Strategy
- Unit tests: Payment service, webhook processing
- Integration tests: Full payment flow
- E2E tests: User checkout with test card
```

---

## 4. Sprint Generation

### 4.1 Enhanced `make-sprints` Agent

Update `make-sprints.agent.md` to handle existing codebases:

```markdown
# Additional Instructions for Existing Codebases

## Context Injection
You will receive:
- INTEGRATION_GUIDE.md (existing codebase analysis)
- Existing SPRINTS.yml (completed sprints)
- Feature specification (new feature requirements)
- Memory knowledge (previous patterns)

## Sprint Generation Rules

### 1. Integration-First Approach
NEW sprints must integrate with EXISTING code:

**❌ Bad**: Create isolated payment module that doesn't connect
**✅ Good**: Extend OrderService to call PaymentService, update Order model

### 2. Dependency Detection
Identify dependencies on existing sprints:
```yaml
- id: 11
  dependencies: ["sprint-2", "sprint-7"]  # Product catalog, shopping cart
```

### 3. Respect Existing Patterns
Follow codebase conventions from INTEGRATION_GUIDE:
- Use same naming conventions
- Follow same architectural patterns
- Match testing strategies
- Use same state management approach

### 4. Minimal Disruption
Prefer extending over modifying:
- Add new files rather than rewriting existing ones
- Extend interfaces rather than changing signatures
- Add routes rather than modifying existing ones

### 5. Integration Testing
Include integration tests that validate:
- New code works with existing code
- Existing tests still pass
- No regressions introduced

## Example Output

```yaml
sprints:
  - id: 11
    goal: "Stripe Backend Integration"
    status: PENDING
    total_effort: "8h"
    dependencies: ["sprint-2"]
    integration_points:
      modifies:
        - "app/Models/Order.php (add payment fields)"
        - "config/services.php (add Stripe config)"
      creates:
        - "app/Services/PaymentService.php"
        - "app/Models/Payment.php"
        - "database/migrations/2025_11_05_create_payments_table.php"
      tests_existing:
        - "tests/Feature/OrderServiceTest.php (add payment scenarios)"
    tasks:
      - id: "task-011-01"
        title: "Create Payment service with Stripe SDK"
        integration_notes: "Will be called by OrderService.checkout()"
        business_rules:
          - "Use existing Stripe account from config"
          - "Follow service pattern (same as OrderService)"
```
```

### 4.2 Sprint Dependencies

```rust
// crates/autoflow-data/src/sprints.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: u32,
    pub goal: String,
    pub status: SprintStatus,

    // NEW: Dependencies on other sprints
    #[serde(default)]
    pub dependencies: Vec<String>,  // ["sprint-2", "sprint-7"]

    // NEW: Integration points with existing code
    #[serde(default)]
    pub integration_points: Option<IntegrationPoints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoints {
    /// Existing files that will be modified
    pub modifies: Vec<String>,

    /// New files that will be created
    pub creates: Vec<String>,

    /// Existing tests that need updates
    pub tests_existing: Vec<String>,

    /// Integration patterns to follow
    pub patterns: Vec<String>,
}
```

### 4.3 Dependency Resolution

```rust
// crates/autoflow-core/src/orchestrator/dependencies.rs

pub struct DependencyResolver;

impl DependencyResolver {
    pub fn resolve_order(sprints: &[Sprint]) -> Result<Vec<u32>> {
        // Topological sort to determine execution order
        let mut graph = HashMap::new();

        for sprint in sprints {
            let deps: Vec<u32> = sprint.dependencies
                .iter()
                .filter_map(|d| parse_sprint_id(d))
                .collect();
            graph.insert(sprint.id, deps);
        }

        topological_sort(&graph)
    }

    pub fn validate_dependencies(sprints: &[Sprint]) -> Result<()> {
        for sprint in sprints {
            for dep in &sprint.dependencies {
                let dep_id = parse_sprint_id(dep)
                    .ok_or_else(|| AutoFlowError::InvalidDependency(dep.clone()))?;

                // Check if dependency exists
                if !sprints.iter().any(|s| s.id == dep_id) {
                    return Err(AutoFlowError::MissingDependency {
                        sprint: sprint.id,
                        missing: dep_id,
                    });
                }

                // Check if dependency is completed
                let dep_sprint = sprints.iter().find(|s| s.id == dep_id).unwrap();
                if dep_sprint.status != SprintStatus::Done {
                    warn!("Sprint {} depends on incomplete sprint {}",
                          sprint.id, dep_id);
                }
            }
        }

        Ok(())
    }
}
```

---

## 5. Integration Strategies

### 5.1 Code-Aware Implementation

**Enhanced `code-implementer` agent** with codebase context:

```markdown
# Additional Context for Existing Codebases

## Before Implementation
1. Query memory for similar patterns in this codebase
2. Read INTEGRATION_GUIDE.md for conventions
3. Examine existing code being integrated with
4. Review integration_points from sprint definition

## Integration Patterns

### Pattern 1: Extending Existing Services
```typescript
// EXISTING: src/services/OrderService.ts
export class OrderService {
  async createOrder(items: CartItem[]): Promise<Order> {
    // existing logic
  }
}

// NEW: Extend in same file
export class OrderService {
  async createOrder(items: CartItem[]): Promise<Order> {
    const order = await this.saveOrder(items);

    // ✅ NEW: Integrate payment
    if (requiresPayment) {
      await this.paymentService.processPayment(order);
    }

    return order;
  }

  // ✅ NEW: Add payment service
  constructor(
    private paymentService: PaymentService  // NEW
  ) {}
}
```

### Pattern 2: Importing Existing Code
```typescript
// NEW: src/components/payment/StripeCheckout.tsx
import { useCart } from '@/store/cartSlice';  // ✅ EXISTING
import { useOrder } from '@/hooks/useOrder';  // ✅ EXISTING

export function StripeCheckout() {
  const cart = useCart();           // ✅ Use existing state
  const { createOrder } = useOrder(); // ✅ Use existing logic

  // NEW payment logic
}
```

### Pattern 3: Extending Database Models
```php
// EXISTING: app/Models/Order.php
class Order extends Model {
    protected $fillable = ['user_id', 'total'];
}

// MODIFY: Add payment fields
class Order extends Model {
    protected $fillable = [
        'user_id',
        'total',
        'payment_id',      // ✅ NEW
        'payment_status',  // ✅ NEW
    ];

    // ✅ NEW: Relationship
    public function payment() {
        return $this->belongsTo(Payment::class);
    }
}
```

## Validation
After implementation, verify:
1. Existing tests still pass
2. New code follows same patterns
3. Integration points work correctly
4. No breaking changes to existing APIs
```

### 5.2 Integration Testing

**Enhanced test runner** that validates integration:

```rust
// crates/autoflow-core/src/testing/integration_validator.rs

pub struct IntegrationValidator;

impl IntegrationValidator {
    pub async fn validate_integration(&self, sprint: &Sprint) -> Result<IntegrationReport> {
        let mut report = IntegrationReport::new();

        // 1. Run existing tests (regression check)
        info!("Running existing tests to check for regressions...");
        let existing_results = self.run_existing_tests().await?;
        report.existing_tests = existing_results;

        if existing_results.has_failures() {
            report.add_issue(IntegrationIssue {
                severity: Severity::Critical,
                message: "New code broke existing tests".into(),
                failing_tests: existing_results.failures(),
            });
        }

        // 2. Validate integration points
        if let Some(integration_points) = &sprint.integration_points {
            for modified_file in &integration_points.modifies {
                // Check if file was actually modified
                let was_modified = self.git_diff_contains(modified_file)?;
                if !was_modified {
                    report.add_warning(format!(
                        "Expected to modify {} but no changes detected",
                        modified_file
                    ));
                }
            }

            for created_file in &integration_points.creates {
                // Check if file was created
                if !Path::new(created_file).exists() {
                    report.add_issue(IntegrationIssue {
                        severity: Severity::High,
                        message: format!("Expected file not created: {}", created_file),
                        failing_tests: vec![],
                    });
                }
            }
        }

        // 3. Run integration tests
        info!("Running integration tests...");
        let integration_results = self.run_integration_tests(sprint).await?;
        report.integration_tests = integration_results;

        Ok(report)
    }
}
```

---

## 6. Examples

### Example 1: Adding Payments to E-Commerce

**Initial State**:
```yaml
# .autoflow/SPRINTS.yml
sprints:
  - id: 1
    goal: "User Authentication"
    status: DONE
  - id: 2
    goal: "Product Catalog"
    status: DONE
  - id: 3
    goal: "Shopping Cart"
    status: DONE
```

**User runs**:
```bash
autoflow add "Add Stripe payment processing"
```

**AutoFlow generates**:
```yaml
sprints:
  # ... existing sprints 1-3 ...

  - id: 4
    goal: "Stripe Backend Integration"
    status: PENDING
    dependencies: ["sprint-2", "sprint-3"]
    integration_points:
      modifies:
        - "app/Models/Order.php"
        - "app/Services/OrderService.php"
      creates:
        - "app/Services/PaymentService.php"
        - "app/Models/Payment.php"
        - "app/Http/Controllers/Api/PaymentController.php"
    tasks:
      - id: "task-004-01"
        title: "Install Stripe PHP SDK"
      - id: "task-004-02"
        title: "Create Payment model and migration"
      - id: "task-004-03"
        title: "Create PaymentService with Stripe integration"
        integration_notes: "Follow same service pattern as OrderService"
      - id: "task-004-04"
        title: "Update OrderService to call PaymentService"
        integration_notes: "Add payment_service dependency injection"

  - id: 5
    goal: "Payment Frontend UI"
    status: PENDING
    dependencies: ["sprint-4"]
    integration_points:
      modifies:
        - "src/pages/CheckoutPage.tsx"
        - "src/store/orderSlice.ts"
      creates:
        - "src/components/payment/StripeCheckout.tsx"
        - "src/api/paymentApi.ts"
```

**User runs**:
```bash
autoflow start
```

**AutoFlow executes**:
1. Creates worktree for sprint-4
2. Runs `code-implementer` with integration context
3. Agent reads existing OrderService, follows patterns
4. Implements PaymentService with same conventions
5. Updates Order model (adds fields, relationships)
6. Runs tests (existing + new)
7. Validates integration (existing tests still pass)
8. Merges to main
9. Continues to sprint-5...

### Example 2: Adding Feature to Legacy Codebase

**Scenario**: Legacy jQuery app, never used AutoFlow

**User runs**:
```bash
cd legacy-app
autoflow init

# AutoFlow detects legacy codebase
# 🔍 Detected: jQuery 3.6, PHP 7.4, no framework
# 📝 Recommendation: Consider modernization sprint first

autoflow analyze

# 🔍 Analyzing codebase...
# ✅ Frontend: jQuery 3.6 (no build system)
# ✅ Backend: Vanilla PHP (no framework)
# ✅ Database: MySQL with raw queries
# ⚠️  Warning: No automated tests detected
# ⚠️  Warning: No version control detected
#
# 📝 Recommendations:
#   1. Add git repository
#   2. Add basic test framework
#   3. Consider framework migration sprint

autoflow add "Add user dashboard"

# AutoFlow generates sprints:
# - Sprint 1: Set up testing framework (Jest + PHPUnit)
# - Sprint 2: Create basic dashboard page
# - Sprint 3: Add dashboard widgets
# - Sprint 4: E2E tests for dashboard
```

**Generated sprints respect legacy patterns**:
```yaml
- id: 1
  goal: "Set up Testing Infrastructure"
  integration_points:
    modifies: []
    creates:
      - "tests/frontend/setup.js"
      - "tests/backend/bootstrap.php"
      - "package.json (add Jest)"
      - "composer.json (add PHPUnit)"
  notes: "Add testing without disrupting existing code"

- id: 2
  goal: "User Dashboard Page"
  integration_points:
    modifies:
      - "includes/header.php (add dashboard link)"
    creates:
      - "dashboard.php"
      - "js/dashboard.js (jQuery style)"
      - "api/dashboard-data.php"
  tasks:
    - title: "Create dashboard.php following existing page structure"
      integration_notes: "Copy structure from profile.php, follow same patterns"
    - title: "Create dashboard.js using jQuery"
      integration_notes: "Use existing jQuery patterns from app.js"
```

### Example 3: Cross-Cutting Concerns

**Scenario**: Add analytics to all pages

```bash
autoflow add "Add Google Analytics to all pages"
```

**AutoFlow generates single integration sprint**:
```yaml
- id: 8
  goal: "Google Analytics Integration"
  integration_points:
    modifies:
      - "src/App.tsx (add analytics provider)"
      - "src/hooks/usePageTracking.ts (add to all page hooks)"
      - "src/router/index.tsx (add route tracking)"
    creates:
      - "src/services/analyticsService.ts"
      - "src/contexts/AnalyticsContext.tsx"
  tasks:
    - title: "Create Analytics service"
    - title: "Add Analytics provider to App.tsx"
      integration_notes: "Wrap existing providers, don't replace"
    - title: "Add page tracking to router"
      integration_notes: "Use router.beforeEach hook (existing pattern)"
    - title: "Add event tracking to all buttons"
      integration_notes: "Create shared Button component wrapper"
```

---

## 7. Quality Gates for Integration

### 7.1 Integration-Specific Validation

```rust
// crates/autoflow-quality/src/integration_gate.rs

pub struct IntegrationGate;

#[async_trait]
impl QualityGate for IntegrationGate {
    fn name(&self) -> &str {
        "integration_validator"
    }

    async fn check(&self, sprint: &Sprint) -> Result<GateResult> {
        let mut issues = vec![];

        // 1. Check existing tests still pass
        let test_results = run_existing_tests().await?;
        if test_results.has_failures() {
            issues.push(Issue {
                severity: Severity::Critical,
                category: "regression".into(),
                message: format!(
                    "{} existing tests failed - new code broke existing functionality",
                    test_results.failure_count()
                ),
                auto_fixable: false,
            });
        }

        // 2. Validate integration points were addressed
        if let Some(integration_points) = &sprint.integration_points {
            for file in &integration_points.modifies {
                if !was_file_modified(file)? {
                    issues.push(Issue {
                        severity: Severity::High,
                        category: "missing_integration".into(),
                        message: format!("Expected to modify {} but file unchanged", file),
                        auto_fixable: false,
                    });
                }
            }

            for file in &integration_points.creates {
                if !Path::new(file).exists() {
                    issues.push(Issue {
                        severity: Severity::High,
                        category: "missing_file".into(),
                        message: format!("Expected to create {} but file not found", file),
                        auto_fixable: false,
                    });
                }
            }
        }

        // 3. Check for API breaking changes
        let api_changes = detect_api_breaking_changes()?;
        if !api_changes.is_empty() {
            issues.push(Issue {
                severity: Severity::Critical,
                category: "breaking_change".into(),
                message: format!("Breaking API changes detected: {:?}", api_changes),
                auto_fixable: false,
            });
        }

        // 4. Validate code follows existing patterns
        let pattern_violations = check_pattern_consistency(sprint)?;
        issues.extend(pattern_violations);

        Ok(GateResult {
            passed: issues.is_empty(),
            issues,
        })
    }
}
```

---

## 8. CLI Commands Summary

```bash
# Analyze existing codebase
autoflow analyze

# Add new feature
autoflow add "Feature description"
autoflow add "Add payments" --requirements="Support Stripe and PayPal"
autoflow add --interactive

# View integration points
autoflow sprints show 11 --integration

# Validate integration
autoflow validate --integration

# Run only existing tests (regression check)
autoflow test --existing-only
```

---

## 9. Key Decisions

### ✅ **Incremental Sprint Approach**
- Append new sprints to existing SPRINTS.yml
- Treat features as additional phases
- Maintain history of all work

### ✅ **Codebase-Aware Generation**
- Analyze existing code before generating sprints
- Store analysis in memory for consistency
- Generate integration-first sprints

### ✅ **Integration Validation**
- Run existing tests (regression check)
- Validate integration points addressed
- Check for breaking changes
- Verify pattern consistency

### ✅ **Dependency Management**
- Explicit dependencies between sprints
- Topological sort for execution order
- Validation of dependency completeness

### ✅ **Legacy Support**
- Works with any codebase (AutoFlow or not)
- Respects existing patterns
- Minimal disruption approach
- Recommends modernization when beneficial

---

## Next Steps

1. **Implement `codebase-analyzer` agent** (Week 3)
2. **Add `autoflow analyze` command** (Week 3)
3. **Enhance `make-sprints` for integration** (Week 4)
4. **Add `autoflow add` command** (Week 4)
5. **Implement dependency resolver** (Week 5)
6. **Add integration quality gate** (Week 7)

**Ready to handle any codebase!** 🚀
