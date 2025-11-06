# AutoFlow Autonomous Environment Setup

**Date**: 2025-11-05
**Purpose**: Define how AutoFlow autonomously sets up complete development environments from scratch

---

## Table of Contents

1. [Philosophy: Zero to Running](#1-philosophy-zero-to-running)
2. [Environment Setup Flow](#2-environment-setup-flow)
3. [DevOps Agent](#3-devops-agent)
4. [Infrastructure Automation](#4-infrastructure-automation)
5. [Per-Sprint Environment Isolation](#5-per-sprint-environment-isolation)
6. [Examples](#6-examples)

---

## 1. Philosophy: Zero to Running

### 1.1 Fully Autonomous Setup

**Goal**: `autoflow init` creates a **fully functional, ready-to-develop environment** with:
- Docker containers configured and running
- Databases created and migrated
- Services connected and healthy
- Development tools installed
- Testing infrastructure ready
- CI/CD pipelines configured

**User provides**: High-level requirements in `BUILD_SPEC.md`

**AutoFlow handles**: Everything else

### 1.2 From Requirements to Running Code

```
User writes BUILD_SPEC.md
         ↓
autoflow init
         ↓
1. Generate design docs (ARCHITECTURE.md, DATABASE.md, etc.)
         ↓
2. Generate sprints (SPRINTS.yml)
         ↓
3. Set up environment (Sprint 0 - Infrastructure)
   - Create docker-compose.yml
   - Configure databases
   - Set up Redis/queues
   - Configure web server
   - Set up test environment
         ↓
4. Verify environment
   - Start containers
   - Run health checks
   - Verify connectivity
         ↓
5. Execute feature sprints (Sprint 1+)
   - Build features
   - Run tests
   - Deploy
         ↓
✅ Fully functional application
```

---

## 2. Environment Setup Flow

### 2.1 Sprint 0: Infrastructure Sprint

**AutoFlow automatically creates "Sprint 0"** before feature sprints:

```yaml
# .autoflow/SPRINTS.yml (auto-generated)
sprints:
  - id: 0
    goal: "Infrastructure & Environment Setup"
    status: PENDING
    priority: CRITICAL
    must_complete_first: true
    total_effort: "3h"
    deliverables:
      - "Docker containers configured and running"
      - "Database schema created"
      - "Development environment verified"
      - "Testing infrastructure ready"
    tasks:
      - id: "task-000-01"
        title: "Create docker-compose.yml for all services"
        business_rules:
          - "PostgreSQL 15 database"
          - "Redis for caching"
          - "Node.js 20 for frontend"
          - "Nginx as reverse proxy"

      - id: "task-000-02"
        title: "Configure database connection and create schema"
        business_rules:
          - "Use .env for credentials"
          - "Auto-run migrations on startup"

      - id: "task-000-03"
        title: "Set up testing environment"
        business_rules:
          - "Playwright for E2E tests"
          - "Jest for unit tests"
          - "Separate test database"

      - id: "task-000-04"
        title: "Verify environment health"
        business_rules:
          - "All containers healthy"
          - "Database connectable"
          - "Can run tests"

  - id: 1
    goal: "User Authentication"
    status: PENDING
    dependencies: ["sprint-0"]  # Can't start until infra ready
    # ... feature tasks
```

### 2.2 When Sprint 0 is Created

**Automatically** when:
1. `autoflow init` on empty project
2. `autoflow start` detects no infrastructure
3. ARCHITECTURE.md specifies infrastructure needs

**Agent used**: `devops-setup` agent

### 2.3 Command Flow

```bash
cd new-project

# Create BUILD_SPEC.md
cat > BUILD_SPEC.md <<EOF
# E-Commerce Platform

## Requirements
- User authentication
- Product catalog
- Shopping cart
- Payment processing (Stripe)

## Tech Stack
- Frontend: React 18
- Backend: Node.js with Express
- Database: PostgreSQL
- Cache: Redis
- Payments: Stripe API
EOF

# Initialize AutoFlow
autoflow init

# What happens:
# 1. Read BUILD_SPEC.md
# 2. Generate design docs (make-docs agent)
#    → ARCHITECTURE.md: Specifies PostgreSQL, Redis, Docker
# 3. Generate sprints (make-sprints agent)
#    → Sprint 0: Infrastructure setup (auto-added)
#    → Sprint 1-10: Feature sprints
# 4. Run Sprint 0 immediately (devops-setup agent)
#    → Create docker-compose.yml
#    → Create .env.example
#    → Create Dockerfile
#    → Start containers
#    → Run migrations
#    → Verify health
# 5. Ready for feature development!

# Start building features
autoflow start

# AutoFlow executes:
# - Sprint 0: ✅ DONE (infrastructure ready)
# - Sprint 1: User authentication (in progress...)
```

---

## 3. DevOps Agent

### 3.1 Enhanced `devops-setup` Agent

```markdown
---
name: devops-setup
description: Autonomous infrastructure setup - Docker, databases, services, CI/CD
tools: Write, Edit, Bash, mcp__context7__*, mcp__memory__*
model: claude-sonnet-4-5-20250929
---

# Role
Expert DevOps engineer who sets up complete, production-ready development environments.

# Workflow

## Step 1: Read Architecture Requirements
Load `.autoflow/docs/ARCHITECTURE.md`:
- Tech stack (Node.js, PostgreSQL, Redis, etc.)
- Services needed
- Database requirements
- External services (Stripe, S3, etc.)
- Testing requirements

## Step 2: Generate Docker Compose
Create `docker-compose.yml` with all services:

```yaml
version: '3.8'

services:
  # Application
  app:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=development
      - DATABASE_URL=postgresql://user:pass@db:5432/myapp
      - REDIS_URL=redis://redis:6379
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_healthy
    volumes:
      - ./src:/app/src
      - /app/node_modules

  # Database
  db:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: myapp
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user"]
      interval: 5s
      timeout: 5s
      retries: 5

  # Cache
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

  # Test Database (isolated)
  test_db:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: myapp_test
    ports:
      - "5433:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
```

## Step 3: Generate Dockerfile
Create optimized Dockerfile:

```dockerfile
FROM node:20-alpine AS builder

WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm ci --only=production

# Copy source
COPY . .

# Build if needed
RUN npm run build || true

# Production stage
FROM node:20-alpine

WORKDIR /app

COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package*.json ./

EXPOSE 3000

CMD ["npm", "start"]
```

## Step 4: Generate Environment Configuration
Create `.env.example`:

```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost:5432/myapp
TEST_DATABASE_URL=postgresql://user:pass@localhost:5433/myapp_test

# Redis
REDIS_URL=redis://localhost:6379

# Application
NODE_ENV=development
PORT=3000

# External Services
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Email
SMTP_HOST=smtp.mailtrap.io
SMTP_PORT=2525
SMTP_USER=your_user
SMTP_PASS=your_pass

# JWT
JWT_SECRET=your-secret-key-change-in-production
```

## Step 5: Database Initialization
Create initialization scripts:

```sql
-- scripts/init-db.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create initial schema
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
```

## Step 6: Testing Infrastructure
Create test configuration:

```javascript
// playwright.config.ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',

  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'mobile',
      use: { ...devices['iPhone 12'] },
    },
  ],

  webServer: {
    command: 'npm run start:test',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

## Step 7: CI/CD Configuration
Create GitHub Actions workflow:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: pass
          POSTGRES_DB: myapp_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

      redis:
        image: redis:7
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379

    steps:
      - uses: actions/checkout@v3

      - uses: actions/setup-node@v3
        with:
          node-version: '20'
          cache: 'npm'

      - run: npm ci
      - run: npm run build
      - run: npm run test
      - run: npm run test:e2e
```

## Step 8: Start Environment
Run startup sequence:

```bash
#!/bin/bash
# scripts/start.sh

set -e

echo "🚀 Starting development environment..."

# Build images
docker-compose build

# Start services
docker-compose up -d

# Wait for health checks
echo "⏳ Waiting for services to be healthy..."
timeout 60s bash -c 'until docker-compose ps | grep -q "healthy"; do sleep 2; done'

# Run migrations
echo "📊 Running database migrations..."
docker-compose exec app npm run migrate

# Seed database if needed
if [ "$1" == "--seed" ]; then
  echo "🌱 Seeding database..."
  docker-compose exec app npm run seed
fi

echo "✅ Environment ready!"
echo ""
echo "Services:"
echo "  App:      http://localhost:3000"
echo "  Database: postgresql://localhost:5432/myapp"
echo "  Redis:    redis://localhost:6379"
echo ""
echo "Commands:"
echo "  docker-compose logs -f     # View logs"
echo "  docker-compose ps          # Check status"
echo "  docker-compose down        # Stop all"
```

## Step 9: Health Check
Verify everything works:

```bash
# Check services
docker-compose ps | grep healthy

# Test database connection
docker-compose exec db psql -U user -d myapp -c "SELECT 1"

# Test Redis
docker-compose exec redis redis-cli ping

# Test app
curl -f http://localhost:3000/health || exit 1

echo "✅ All systems operational"
```

## Step 10: Document Setup
Create README.md with setup instructions:

```markdown
# E-Commerce Platform

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Node.js 20+ (for local development)

### Setup
\`\`\`bash
# Clone repository
git clone <repo>
cd project

# Copy environment variables
cp .env.example .env

# Start development environment
./scripts/start.sh --seed

# The app is now running at http://localhost:3000
\`\`\`

### Development
\`\`\`bash
# View logs
docker-compose logs -f

# Run tests
npm run test
npm run test:e2e

# Stop environment
docker-compose down
\`\`\`

## Architecture
See [ARCHITECTURE.md](.autoflow/docs/ARCHITECTURE.md)
```

## Output
- ✅ docker-compose.yml created
- ✅ Dockerfile optimized
- ✅ .env.example configured
- ✅ Database initialization scripts
- ✅ Test infrastructure configured
- ✅ CI/CD pipeline ready
- ✅ All containers healthy
- ✅ README.md with instructions
```

---

## 4. Infrastructure Automation

### 4.1 Infrastructure Validator

```rust
// crates/autoflow-quality/src/infrastructure_validator.rs

pub struct InfrastructureValidator;

impl InfrastructureValidator {
    pub async fn validate_environment(&self) -> Result<InfrastructureReport> {
        let mut report = InfrastructureReport::new();

        // 1. Check Docker installed
        if !self.check_docker_installed().await? {
            report.add_critical_issue("Docker not installed");
            return Ok(report);
        }

        // 2. Check docker-compose.yml exists
        if !Path::new("docker-compose.yml").exists() {
            report.add_error("docker-compose.yml not found");
        }

        // 3. Check containers running
        let containers = self.get_container_status().await?;
        for container in containers {
            if container.status != "healthy" {
                report.add_error(format!(
                    "Container {} is {}, expected healthy",
                    container.name, container.status
                ));
            }
        }

        // 4. Check database connectivity
        if let Err(e) = self.test_database_connection().await {
            report.add_error(format!("Database connection failed: {}", e));
        }

        // 5. Check Redis connectivity
        if let Err(e) = self.test_redis_connection().await {
            report.add_error(format!("Redis connection failed: {}", e));
        }

        // 6. Check application health endpoint
        if let Err(e) = self.test_app_health().await {
            report.add_error(format!("App health check failed: {}", e));
        }

        // 7. Check test database isolated
        if !self.verify_test_db_isolation().await? {
            report.add_warning("Test database not properly isolated");
        }

        // 8. Check environment variables
        if !Path::new(".env").exists() {
            report.add_warning(".env file not found (using .env.example?)");
        }

        Ok(report)
    }

    async fn test_database_connection(&self) -> Result<()> {
        let output = Command::new("docker-compose")
            .args(&["exec", "-T", "db", "psql", "-U", "user", "-d", "myapp", "-c", "SELECT 1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(AutoFlowError::DatabaseConnectionFailed);
        }

        Ok(())
    }

    async fn test_redis_connection(&self) -> Result<()> {
        let output = Command::new("docker-compose")
            .args(&["exec", "-T", "redis", "redis-cli", "ping"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(AutoFlowError::RedisConnectionFailed);
        }

        Ok(())
    }

    async fn test_app_health(&self) -> Result<()> {
        let response = reqwest::get("http://localhost:3000/health").await?;
        if !response.status().is_success() {
            return Err(AutoFlowError::AppHealthCheckFailed);
        }

        Ok(())
    }
}
```

### 4.2 Startup Script

```rust
// crates/autoflow-core/src/infrastructure/startup.rs

pub struct InfrastructureManager;

impl InfrastructureManager {
    pub async fn start_environment(&self) -> Result<()> {
        info!("Starting development environment...");

        // 1. Build images
        self.docker_compose_build().await?;

        // 2. Start services
        self.docker_compose_up().await?;

        // 3. Wait for health
        self.wait_for_healthy().await?;

        // 4. Run migrations
        self.run_migrations().await?;

        // 5. Verify health
        let validator = InfrastructureValidator;
        let report = validator.validate_environment().await?;

        if report.has_errors() {
            return Err(AutoFlowError::InfrastructureNotReady {
                issues: report.errors(),
            });
        }

        success!("Environment ready!");
        info!("App:      http://localhost:3000");
        info!("Database: postgresql://localhost:5432/myapp");
        info!("Redis:    redis://localhost:6379");

        Ok(())
    }

    async fn docker_compose_build(&self) -> Result<()> {
        info!("Building Docker images...");

        let status = Command::new("docker-compose")
            .arg("build")
            .status()
            .await?;

        if !status.success() {
            return Err(AutoFlowError::DockerBuildFailed);
        }

        Ok(())
    }

    async fn docker_compose_up(&self) -> Result<()> {
        info!("Starting services...");

        let status = Command::new("docker-compose")
            .args(&["up", "-d"])
            .status()
            .await?;

        if !status.success() {
            return Err(AutoFlowError::DockerStartFailed);
        }

        Ok(())
    }

    async fn wait_for_healthy(&self) -> Result<()> {
        info!("Waiting for services to be healthy...");

        let max_wait = Duration::from_secs(60);
        let start = Instant::now();

        loop {
            if start.elapsed() > max_wait {
                return Err(AutoFlowError::ServiceStartTimeout);
            }

            let output = Command::new("docker-compose")
                .args(&["ps", "--format", "json"])
                .output()
                .await?;

            let containers: Vec<ContainerStatus> =
                serde_json::from_slice(&output.stdout)?;

            if containers.iter().all(|c| c.health == "healthy") {
                break;
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        Ok(())
    }

    async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations...");

        let status = Command::new("docker-compose")
            .args(&["exec", "-T", "app", "npm", "run", "migrate"])
            .status()
            .await?;

        if !status.success() {
            return Err(AutoFlowError::MigrationFailed);
        }

        Ok(())
    }
}
```

---

## 5. Per-Sprint Environment Isolation

### 5.1 Worktree-Specific Docker

**Each worktree gets its own Docker environment**:

```
<project>/                           # Main repo
├── docker-compose.yml               # Port 3000
└── .env

../sprint-5/                         # Sprint 5 worktree
├── docker-compose.yml               # Port 3005 (auto-adjusted)
└── .env                             # Isolated DB

../bugfix-login/                     # Bugfix worktree
├── docker-compose.yml               # Port 3010 (auto-adjusted)
└── .env                             # Isolated DB
```

### 5.2 Port Allocation

```rust
// crates/autoflow-git/src/worktree.rs

impl WorktreeManager {
    async fn setup_worktree_environment(&self, worktree: &Worktree) -> Result<()> {
        // 1. Allocate unique ports
        let base_port = 3000 + (worktree.id * 10);
        let ports = PortAllocation {
            app: base_port,
            db: base_port + 1,
            redis: base_port + 2,
        };

        // 2. Copy docker-compose.yml with new ports
        let compose_content = fs::read_to_string("docker-compose.yml")?;
        let modified = self.adjust_ports(&compose_content, &ports)?;
        fs::write(worktree.path.join("docker-compose.yml"), modified)?;

        // 3. Copy and modify .env
        let env_content = fs::read_to_string(".env.example")?;
        let modified_env = format!(
            "{}\nPORT={}\nDATABASE_NAME=myapp_sprint_{}\n",
            env_content,
            ports.app,
            worktree.id
        );
        fs::write(worktree.path.join(".env"), modified_env)?;

        // 4. Start containers for this worktree
        self.start_worktree_containers(worktree, &ports).await?;

        Ok(())
    }
}
```

---

## 6. Examples

### Example 1: E-Commerce from Scratch

```bash
# User creates BUILD_SPEC.md
cat > BUILD_SPEC.md <<EOF
# E-Commerce Platform
React + Node.js + PostgreSQL + Redis + Stripe
EOF

# Initialize
autoflow init

# AutoFlow:
# 1. Generates design docs
# 2. Creates Sprint 0 (infrastructure)
# 3. Executes Sprint 0:
#    → docker-compose.yml (PostgreSQL, Redis, app, test DB)
#    → Dockerfile (Node.js 20)
#    → .env.example
#    → Start containers
#    → Verify health
# 4. ✅ Environment ready!

# Start feature development
autoflow start

# Sprint 1: User auth (uses running environment)
# Sprint 2: Product catalog (uses running environment)
# ...
```

### Example 2: Microservices Architecture

```bash
# BUILD_SPEC.md
cat > BUILD_SPEC.md <<EOF
# Microservices Platform
- API Gateway (Node.js)
- Auth Service (Node.js)
- Product Service (Go)
- Order Service (Python)
- PostgreSQL, Redis, RabbitMQ
EOF

autoflow init

# AutoFlow generates:
# docker-compose.yml with:
# - api-gateway (port 3000)
# - auth-service (port 3001)
# - product-service (port 3002)
# - order-service (port 3003)
# - postgres
# - redis
# - rabbitmq
# - nginx (reverse proxy)

# All services configured and running!
```

### Example 3: Monorepo with Multiple Apps

```bash
# BUILD_SPEC.md
cat > BUILD_SPEC.md <<EOF
# Monorepo Platform
- Admin Dashboard (React)
- Customer App (React Native)
- Backend API (Node.js)
- Worker Processes (Node.js)
- PostgreSQL, Redis
EOF

autoflow init

# AutoFlow generates:
# docker-compose.yml:
# - admin-dashboard (port 3000)
# - backend-api (port 4000)
# - workers (background jobs)
# - postgres
# - redis
#
# All properly networked!
```

---

## 7. CLI Commands

```bash
# Initialize with environment setup
autoflow init                        # Auto-creates Sprint 0

# Start environment
autoflow env start                   # Start Docker containers
autoflow env stop                    # Stop containers
autoflow env restart                 # Restart containers
autoflow env logs                    # View logs
autoflow env health                  # Check health

# Per-worktree environments
autoflow worktree create sprint-5    # Auto-starts isolated environment
autoflow worktree env start sprint-5 # Start worktree containers
autoflow worktree env stop sprint-5  # Stop worktree containers

# Infrastructure validation
autoflow validate --infrastructure   # Check all systems operational
```

---

## 8. Key Decisions

### ✅ **Sprint 0 Auto-Created**
- Infrastructure setup is first sprint
- Must complete before feature development
- Fully autonomous (no manual setup)

### ✅ **Docker Everything**
- Development in containers (consistent environment)
- Testing in containers (isolated)
- Production-like locally

### ✅ **Per-Worktree Isolation**
- Each sprint/bugfix gets own containers
- Unique ports (3000, 3010, 3020, etc.)
- Isolated databases
- No conflicts

### ✅ **Health Check Required**
- All containers must be healthy
- Database must be connectable
- Tests must be runnable
- Blocks feature development until ready

### ✅ **CI/CD Auto-Generated**
- GitHub Actions workflow
- Tests run in containers
- Same environment locally and CI

---

## Next Steps

1. **Implement `devops-setup` agent enhancements** (Week 5)
2. **Add Sprint 0 auto-generation** (Week 5)
3. **Implement infrastructure validator** (Week 6)
4. **Add per-worktree Docker isolation** (Week 7)
5. **Add `autoflow env` commands** (Week 8)

**Ready for fully autonomous environment setup!** 🚀🐳
