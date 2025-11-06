# AutoFlow TODO List

**Last Updated**: 2025-11-06
**Current Phase**: Core System Complete → Ready for Integration Testing

---

## ✅ COMPLETED

### Phase 1: Foundation (Week 1)
- [x] Cargo workspace setup (7 crates)
- [x] Core data structures (Sprint, Task, Config, Error)
- [x] CLI framework with clap (13 commands)
- [x] Project templates (SPRINTS.yml, CLAUDE.md, settings.json)
- [x] `autoflow init` command (fully working)
- [x] `autoflow status` command (fully working)
- [x] Comprehensive documentation (10 files)
- [x] .gitignore and README
- [x] All tests passing

### Phase 2: Core Orchestration (Days 1-3) ✅
- [x] Orchestrator (`crates/autoflow-core/src/orchestrator.rs`)
  - [x] Load SPRINTS.yml with error handling
  - [x] Implement full `run_sprint()` logic
  - [x] Loop through status phases
  - [x] Execute phase based on current status
  - [x] Handle state transitions
  - [x] Track retry counts
  - [x] Detect BLOCKED status (3 retries exceeded)
  - [x] Save progress after each iteration
  - [x] Max iteration safety limit (50 iterations)
  - [x] Implement `run_parallel()` for multiple sprints
  - [x] Add progress callbacks/logging

- [x] Agent Executor (`crates/autoflow-agents/src/executor.rs`)
  - [x] Load agent definitions from ~/.claude/agents/
  - [x] Parse agent frontmatter (model, tools, description)
  - [x] Spawn Claude CLI with `tokio::process::Command`
  - [x] Pass agent system prompt and context via stdin
  - [x] Capture stdout/stderr streams
  - [x] Map SprintStatus to agent name
  - [x] Build agent context from sprint data

- [x] `autoflow start` (`crates/autoflow-cli/src/commands/start.rs`)
  - [x] Check if `.autoflow/SPRINTS.yml` exists
  - [x] Load sprints with error handling
  - [x] Filter sprints by status (PENDING or in-progress)
  - [x] `--sprint=N` flag to run specific sprint
  - [x] `--parallel` flag for concurrent execution
  - [x] Create Orchestrator instance
  - [x] Run orchestrator on sprints
  - [x] Save updated SPRINTS.yml
  - [x] Show completion summary

### Phase 3: Quality Gates (Days 4-6) ✅
- [x] Schema Validator (`crates/autoflow-quality/src/schema_validator.rs`)
  - [x] Load JSON schemas from `~/.autoflow/schemas/`
  - [x] Compile schemas with `jsonschema` crate
  - [x] Validate SPRINTS.yml against schema
  - [x] Return validation errors with line numbers
  - [x] Auto-fix for common issues (markdown in YAML)
  - [x] Normalize status values (Done → DONE)

- [x] Quality Gate Pipeline (`crates/autoflow-quality/src/pipeline.rs`)
  - [x] Create `QualityGate` trait
  - [x] Implement gates:
    - [x] SchemaValidator
    - [x] OutputFormatValidator (detect markdown in YAML)
    - [x] BlockerDetector (missing dependencies, APIs)
    - [x] CodeQualityValidator (basic checks)
  - [x] Run gates in sequence
  - [x] Stop on critical failures
  - [x] Generate QualityReport
  - [x] Auto-fix capability

- [x] `autoflow validate` command
  - [x] Run quality gate pipeline
  - [x] `--fix` flag for auto-fix
  - [x] `--infrastructure` flag (stub)
  - [x] `--integration` flag (stub)
  - [x] Display validation report

### Phase 6: Git Worktrees (Days 13-15) ✅
- [x] Worktree Manager (`crates/autoflow-git/src/worktree.rs`)
  - [x] Implement `create_worktree(branch_name)` using git CLI
  - [x] Calculate unique ports for Docker (3000 + sprint_id * 10)
  - [x] Copy docker-compose.yml with adjusted ports
  - [x] Create isolated .env file
  - [x] Implement `merge_worktree(branch_name)`
  - [x] Implement `delete_worktree(branch_name)`
  - [x] Implement `list_worktrees()`

- [x] `autoflow worktree` commands
  - [x] `worktree list` - Show all worktrees with filtering
  - [x] `worktree create <branch>` - Create new worktree
  - [x] `worktree merge <branch>` - Merge and clean up
  - [x] `worktree delete <branch>` - Delete worktree
  - [x] `worktree prune` - Clean up merged worktrees

- [x] `autoflow sprints` commands
  - [x] `sprints list` - Show all sprints grouped by status
  - [x] `sprints show <id>` - Show sprint details with --integration flag

---

## 🚧 IN PROGRESS / TODO

### Phase 4: Feature Addition (Days 7-9)

#### 7. Implement `autoflow analyze` (`crates/autoflow-cli/src/commands/analyze.rs`)
- [ ] Create `codebase-analyzer` agent definition
- [ ] Scan project structure (use `walkdir`)
- [ ] Detect tech stack (package.json, composer.json, Cargo.toml)
- [ ] Identify frameworks (React, Laravel, etc.)
- [ ] Find integration points (API endpoints, models)
- [ ] Generate INTEGRATION_GUIDE.md
- [ ] Store findings in memory MCP

#### 8. Implement `autoflow add` (`crates/autoflow-cli/src/commands/add.rs`)
- [ ] Check if project initialized
- [ ] Load existing SPRINTS.yml
- [ ] Load INTEGRATION_GUIDE.md (if exists)
- [ ] Create feature specification from description
- [ ] Spawn `make-sprints` agent with context
- [ ] Parse generated sprints
- [ ] Append to SPRINTS.yml
- [ ] Update project metadata (total_sprints)
- [ ] Save SPRINTS.yml
- [ ] Show next steps

### Phase 5: Bug Fixing (Days 10-12)

#### 9. Implement `autoflow fix` (`crates/autoflow-cli/src/commands/fix.rs`)
- [ ] Create bugfix worktree (use git2)
- [ ] Spawn `bug-investigator` agent
- [ ] Parse investigation results
- [ ] If root cause found:
  - [ ] Spawn `bug-fixer` agent
  - [ ] Run tests
  - [ ] If tests pass, offer to merge
  - [ ] If tests fail, retry or mark BLOCKED
- [ ] Save bug analysis to `.autoflow/bugs/`

#### 10. Playwright MCP Integration (`crates/autoflow-agents/src/playwright.rs`)
- [ ] Create PlaywrightClient
- [ ] Implement MCP tool calls:
  - [ ] `playwright__launch()`
  - [ ] `playwright__navigate()`
  - [ ] `playwright__click()`
  - [ ] `playwright__screenshot()`
  - [ ] `playwright__computed_styles()`
- [ ] Pass to bug-investigator agent

### Phase 6: Git Worktrees (Days 13-15)

#### 11. Worktree Manager (`crates/autoflow-git/src/worktree.rs`)
- [ ] Implement `create_worktree(branch_name)` using git2
- [ ] Calculate unique ports for Docker (3000 + sprint_id * 10)
- [ ] Copy docker-compose.yml with adjusted ports
- [ ] Create isolated .env file
- [ ] Start Docker containers for worktree
- [ ] Implement `merge_worktree(branch_name)`
- [ ] Implement `delete_worktree(branch_name)`
- [ ] Implement `list_worktrees()`

#### 12. Implement `autoflow worktree` commands
- [ ] `worktree list` - Show all worktrees
- [ ] `worktree create <branch>` - Create new worktree
- [ ] `worktree merge <branch>` - Merge and clean up
- [ ] `worktree delete <branch>` - Delete worktree
- [ ] `worktree prune` - Clean up merged worktrees

#### 13. Implement `autoflow rollback`
- [ ] Load current sprint
- [ ] Find worktree branch
- [ ] Delete worktree and branch
- [ ] Reset sprint status to previous state
- [ ] Save SPRINTS.yml

### Phase 7: Environment Setup (Days 16-18)

#### 14. DevOps Agent Enhancement (`agents/devops-setup.agent.md`)
- [ ] Move agent from old bash system to new rust system
- [ ] Generate docker-compose.yml from ARCHITECTURE.md
- [ ] Generate Dockerfile
- [ ] Generate .env.example
- [ ] Create database initialization scripts
- [ ] Set up test infrastructure (Playwright config)
- [ ] Generate CI/CD workflow (GitHub Actions)

#### 15. Infrastructure Validator (`crates/autoflow-quality/src/infrastructure_validator.rs`)
- [ ] Check Docker installed
- [ ] Check docker-compose.yml exists
- [ ] Check container health
- [ ] Test database connection
- [ ] Test Redis connection
- [ ] Test app health endpoint
- [ ] Verify test database isolation

#### 16. Implement `autoflow env` commands
- [ ] `env start` - Start Docker containers
- [ ] `env stop` - Stop containers
- [ ] `env restart` - Restart containers
- [ ] `env logs` - View logs (with --follow)
- [ ] `env health` - Run health checks

### Phase 8: Additional Commands (Days 19-21)

#### 17. Implement `autoflow validate`
- [ ] `--infrastructure` flag - Check Docker/services
- [ ] `--integration` flag - Check frontend/backend sync
- [ ] `--fix` flag - Auto-fix issues
- [ ] Run quality gate pipeline
- [ ] Display validation report

#### 18. Implement `autoflow sprints` commands
- [ ] `sprints list` - Show all sprints
- [ ] `sprints show <id>` - Show sprint details
  - [ ] `--integration` flag - Show integration points
- [ ] `sprints create` - Manual sprint creation

#### 19. Implement `autoflow agents`
- [ ] List all agents from `~/.autoflow/agents/`
- [ ] Parse frontmatter
- [ ] Display agent names, descriptions, tools
- [ ] `--detailed` flag for full info

#### 20. Implement `autoflow skills`
- [ ] List all skills from `~/.autoflow/skills/`
- [ ] Parse SKILL.md files
- [ ] Display skill names and descriptions

### Phase 9: Agent Definitions (Days 22-25)

#### 21. Port Agents from Bash System
- [ ] Copy 25 agent definitions from `/home/dan/.claude/agents/`
- [ ] Update for new Rust orchestrator
- [ ] Test each agent:
  - [ ] code-implementer
  - [ ] test-writer
  - [ ] reviewer
  - [ ] review-fixer
  - [ ] unit-fixer
  - [ ] e2e-writer
  - [ ] e2e-fixer
  - [ ] bug-investigator
  - [ ] bug-fixer
  - [ ] debug-blocker
  - [ ] health-check
  - [ ] health-check-fixer
  - [ ] make-docs
  - [ ] make-sprints
  - [ ] review-sprints
  - [ ] link-sprint-docs
  - [ ] codebase-analyzer (new)
  - [ ] frontend-react
  - [ ] backend-laravel
  - [ ] backend-nodejs (new)
  - [ ] backend-golang (new)
  - [ ] backend-python (new)
  - [ ] devops-setup
  - [ ] autoflow-learn

#### 22. Port Skills from Bash System
- [ ] Copy 13 skill definitions from `/home/dan/.claude/skills/`
- [ ] Test each skill:
  - [ ] react-vite-integration
  - [ ] vue-vite-integration
  - [ ] laravel-react-integration
  - [ ] playwright-wait-strategies
  - [ ] playwright-pointer-interception
  - [ ] react-state-timing
  - [ ] vue-reactivity-timing
  - [ ] async-race-conditions
  - [ ] frontend-integration-check
  - [ ] e2e-task-validation
  - [ ] sprint-validation
  - [ ] tailwind-v4-setup
  - [ ] typescript-strict-mode (new)

### Phase 10: Polish & Testing (Days 26-30)

#### 23. Progress Visualization
- [ ] Add progress bars (indicatif crate)
- [ ] Show current phase
- [ ] Show time elapsed
- [ ] Show estimated time remaining
- [ ] Add spinner for agent execution

#### 24. Error Handling Improvements
- [ ] Better error messages with context
- [ ] Suggestions for common errors
- [ ] Error recovery strategies
- [ ] Automatic retry on transient failures

#### 25. Integration Tests
- [ ] Test full pipeline (init → start → complete)
- [ ] Test with sample project
- [ ] Test parallel execution
- [ ] Test error handling
- [ ] Test rollback
- [ ] Test bug fix workflow

#### 26. Documentation
- [ ] API documentation (cargo doc)
- [ ] User guide for each command
- [ ] Troubleshooting guide
- [ ] Contributing guide
- [ ] Example projects

#### 27. Install Script (`scripts/install.sh`)
- [ ] Build release binary
- [ ] Create `~/.autoflow/` directory
- [ ] Copy binary to `~/.autoflow/bin/`
- [ ] Copy agents to `~/.autoflow/agents/`
- [ ] Copy skills to `~/.autoflow/skills/`
- [ ] Copy reference materials
- [ ] Copy schemas
- [ ] Create config.toml
- [ ] Add to PATH (symlink to /usr/local/bin)

#### 28. Performance Optimization
- [ ] Profile orchestrator
- [ ] Optimize YAML parsing (lazy loading?)
- [ ] Optimize agent spawning
- [ ] Add caching where appropriate
- [ ] Parallel sprint execution optimization

---

## 📦 LATER / NICE TO HAVE

### Advanced Features
- [ ] `autoflow update` - Update agents/skills/schemas
- [ ] `autoflow metrics` - Show performance metrics
- [ ] Web dashboard for sprint progress
- [ ] VS Code extension
- [ ] GitHub Action for CI/CD
- [ ] Template marketplace
- [ ] Agent marketplace
- [ ] Plugin system
- [ ] Hot reload (watch mode)
- [ ] Interactive mode (TUI)

### Quality Improvements
- [ ] Property-based testing (proptest)
- [ ] Fuzzing
- [ ] Coverage reporting (tarpaulin)
- [ ] Benchmarks
- [ ] Memory profiling
- [ ] Security audit

### Documentation
- [ ] Video tutorials
- [ ] Blog posts
- [ ] Case studies
- [ ] API reference site
- [ ] Interactive playground

---

## 🎯 PRIORITIES (Next 7 Days)

### Day 1-2: Make `autoflow start` Work
1. Implement Orchestrator run_sprint()
2. Implement Agent Executor
3. Implement autoflow start command
4. Test with simple sprint

### Day 3-4: Quality Gates
5. Schema validation
6. Output format validation
7. Basic quality gate pipeline

### Day 5-6: Feature Addition
8. Implement autoflow analyze
9. Implement autoflow add

### Day 7: Testing & Polish
10. Integration tests
11. Error handling improvements
12. Documentation updates

---

## 📝 NOTES

### Current Blockers
- None! Foundation complete and working

### Decisions Needed
- None at this time

### Technical Debt
- Remove unused import warning in CLI
- Add more unit tests for data structures
- Add integration tests

### Questions
- Should we use MCP servers for all external tools?
- How to handle agent failures gracefully?
- What's the best way to show real-time progress?

---

## 🔗 RELATED FILES

- Architecture: `ARCHITECTURE.md`
- Getting Started: `GETTING_STARTED.md`
- Progress: `PROGRESS.md`
- Demo: `DEMO.md`
- Status: `STATUS.md`

---

**Next Session**: Start with "Implement Orchestrator run_sprint()" (#1 in Phase 2)
