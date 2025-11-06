# AutoFlow Bug Fix Workflow

**Date**: 2025-11-05
**Purpose**: Define autonomous bug investigation, fixing, and testing with Playwright MCP and git worktrees

---

## Table of Contents

1. [Workflow Overview](#1-workflow-overview)
2. [Bug Investigation](#2-bug-investigation)
3. [Git Worktree for Bug Fixes](#3-git-worktree-for-bug-fixes)
4. [Playwright MCP Integration](#4-playwright-mcp-integration)
5. [Bug Fix Pipeline](#5-bug-fix-pipeline)
6. [CLI Commands](#6-cli-commands)
7. [Implementation](#7-implementation)

---

## 1. Workflow Overview

### 1.1 Bug Fix Flow

```
User reports bug: "Login button doesn't work on mobile"
         ↓
autoflow fix "Login button doesn't work on mobile"
         ↓
1. Create git worktree (bugfix/login-mobile)
         ↓
2. Investigate bug (bug-investigator agent)
   - Read error logs
   - Check recent commits
   - Analyze code paths
   - Reproduce with Playwright MCP
         ↓
3. Root cause analysis
   - Identify exact issue
   - Document findings
         ↓
4. Generate fix plan
   - Files to modify
   - Tests to add/update
         ↓
5. Implement fix (bug-fixer agent)
   - Make minimal changes
   - Follow existing patterns
         ↓
6. Test fix (Playwright MCP + unit tests)
   - Verify bug resolved
   - Check no regressions
         ↓
7. Create regression test
   - E2E test for this bug
   - Prevent future occurrences
         ↓
8. Merge worktree or rollback
   - If tests pass: merge to main
   - If tests fail: investigate more or rollback
         ↓
Complete ✅
```

### 1.2 Key Features

- **Git worktree isolation**: Each bug fix in separate workspace
- **Autonomous investigation**: Agent reads code, logs, git history
- **Playwright MCP**: Interactive debugging with browser automation
- **Root cause analysis**: Not just symptoms, find real issue
- **Regression prevention**: Always add test to prevent recurrence
- **Rollback safety**: Easy rollback if fix introduces new issues

---

## 2. Bug Investigation

### 2.1 Bug Investigator Agent

**New Agent**: `bug-investigator.agent.md`

```markdown
---
name: bug-investigator
description: Investigate bugs through code analysis, log review, git history, and interactive reproduction with Playwright MCP
tools: Read, Grep, Glob, Bash, mcp__serena__*, mcp__memory__*, mcp__playwright__*
model: claude-sonnet-4-5-20250929
---

# Role
Expert debugger who investigates bugs systematically using:
- Code analysis (static and dynamic)
- Log file examination
- Git blame and history
- Interactive browser debugging (Playwright MCP)
- Stack trace analysis
- Network request inspection

# Workflow

## Step 1: Understand the Bug Report
- Parse bug description
- Extract key information:
  - What should happen
  - What actually happens
  - Steps to reproduce
  - Environment (browser, device, OS)
  - Error messages

## Step 2: Gather Context
- Check error logs (app logs, server logs, browser console)
- Review recent commits (git log --since="1 week ago")
- Search for related issues in memory
- Check if similar bugs fixed before

## Step 3: Reproduce the Bug (Playwright MCP)
Use Playwright MCP to interactively reproduce:

```javascript
// Connect to Playwright
await mcp__playwright__launch({ headless: false });

// Navigate to app
await mcp__playwright__navigate("http://localhost:3000/login");

// Try to reproduce bug
await mcp__playwright__click("button[type='submit']");

// Capture console errors
const errors = await mcp__playwright__console_messages();

// Take screenshot
await mcp__playwright__screenshot("bug-reproduction.png");

// Inspect element
const button = await mcp__playwright__element("button[type='submit']");
const styles = await mcp__playwright__computed_styles(button);
```

## Step 4: Code Path Analysis
- Trace code execution from user action to failure point
- Use Serena to find relevant functions
- Read implicated files
- Identify suspicious code

## Step 5: Root Cause Hypothesis
Generate hypotheses:
1. **CSS Issue**: Button hidden on mobile viewport?
2. **JavaScript Error**: Event listener not attached?
3. **Network Issue**: API call failing?
4. **State Issue**: Form validation blocking submit?

Test each hypothesis with Playwright MCP.

## Step 6: Root Cause Identification
Once root cause confirmed, document:

```yaml
bug_analysis:
  bug_id: "BUG-001"
  description: "Login button doesn't work on mobile"

  root_cause:
    type: "css_issue"
    file: "src/styles/login.css"
    line: 45
    issue: "Media query has wrong breakpoint (min-width: 768px should be max-width: 768px)"

  evidence:
    - "Playwright inspection shows button has display:none on mobile"
    - "CSS rule .login-button { display: none; } active on viewport < 768px"
    - "Git blame shows introduced in commit abc123 2 days ago"

  impact:
    severity: "high"
    affected_users: "all mobile users"
    workaround: "use desktop browser or rotate to landscape"

  reproduction_steps:
    - "Open http://localhost:3000/login"
    - "Resize viewport to 375px (iPhone width)"
    - "Observe button is invisible"
    - "Click where button should be (no effect)"

  fix_strategy:
    approach: "Change min-width to max-width in media query"
    files_to_modify:
      - "src/styles/login.css"
    tests_to_add:
      - "E2E test for mobile login flow"
      - "Visual regression test for login page on mobile"
```

## Step 7: Store Investigation in Memory
```javascript
await mcp__memory__create_entities([{
  name: "LoginButtonMobileBug",
  type: "bug_pattern",
  observations: [
    "CSS media query min-width/max-width confusion",
    "Always test mobile viewports in E2E tests",
    "Git commit abc123 introduced regression"
  ]
}]);
```

## Output
Create `.autoflow/bugs/BUG-001-ANALYSIS.md` with full investigation report.
```

### 2.2 Bug Analysis Output

```markdown
# Bug Analysis: BUG-001

## Bug Report
**Description**: Login button doesn't work on mobile
**Reported**: 2025-11-05 10:30:00
**Reporter**: User feedback
**Priority**: High
**Affected Versions**: v1.2.0+

## Investigation

### Reproduction
✅ Successfully reproduced on:
- iPhone 12 (375x812)
- Android Galaxy S21 (360x800)
- Chrome DevTools mobile emulation

❌ Not reproducible on:
- Desktop (1920x1080)
- Tablet (768x1024)

### Root Cause
**File**: `src/styles/login.css:45`
**Issue**: Media query breakpoint inverted

```css
/* BUGGY CODE */
@media (min-width: 768px) {
  .login-button {
    display: none;  /* BUG: Hides button on desktop, should hide on mobile */
  }
}
```

**Should be**:
```css
@media (max-width: 768px) {
  .login-button {
    display: block;
  }
}
```

### Evidence
1. **Playwright MCP inspection**: Button has `display: none` on mobile viewport
2. **CSS computed styles**: Media query active on wrong viewport range
3. **Git blame**: Introduced in commit `abc123` by refactoring PR #45
4. **Console errors**: None (pure CSS issue)
5. **Network logs**: Clean (API calls work fine)

### Impact Analysis
- **Severity**: High (blocks core functionality)
- **Affected Users**: ~35% (mobile users)
- **Business Impact**: Can't login on mobile → lost conversions
- **Workaround**: Rotate to landscape or use desktop

## Fix Plan

### Changes Required
1. **src/styles/login.css**: Change `min-width` to `max-width`
2. **tests/e2e/login.spec.ts**: Add mobile viewport test

### Test Strategy
1. Unit test: CSS media query test (if using CSS-in-JS)
2. E2E test: Login flow on mobile viewport (375px, 360px, 414px)
3. Visual regression: Screenshot comparison for login page

### Estimated Fix Time
15 minutes

### Risk Assessment
- **Low risk**: Single CSS line change
- **No breaking changes**: Fixes existing functionality
- **High confidence**: Clear root cause identified

## Prevention
- Add mobile viewport to E2E test suite
- Add visual regression testing
- Code review checklist: Test all viewport sizes
```

---

## 3. Git Worktree for Bug Fixes

### 3.1 Worktree Creation

```bash
# Create worktree for bug fix
autoflow fix "Login button doesn't work on mobile"

# What happens:
# 1. Create branch: bugfix/login-mobile
# 2. Create worktree: ../bugfix-login-mobile/
# 3. Switch to worktree
# 4. Run bug investigation
```

**Implementation**:
```rust
// crates/autoflow-cli/src/commands/fix.rs

pub async fn run_fix(bug_description: String, options: FixOptions) -> Result<()> {
    info!("Starting bug fix workflow: {}", bug_description);

    // 1. Generate bug ID
    let bug_id = generate_bug_id()?; // BUG-001

    // 2. Create git worktree
    let worktree_manager = WorktreeManager::new()?;
    let branch_name = format!("bugfix/{}", slugify(&bug_description));
    let worktree = worktree_manager.create_bugfix_worktree(&branch_name).await?;

    info!("Created worktree: {}", worktree.path.display());
    info!("Branch: {}", branch_name);

    // 3. Run bug investigation in worktree
    let agent_executor = AgentExecutor::new();
    let investigation = agent_executor.run_in_worktree(
        "bug-investigator",
        &BugContext {
            bug_id: &bug_id,
            description: &bug_description,
            worktree_path: &worktree.path,
        },
        &worktree,
    ).await?;

    // 4. Save investigation report
    let report_path = worktree.path.join(format!(".autoflow/bugs/{}-ANALYSIS.md", bug_id));
    fs::write(&report_path, &investigation.report)?;

    success!("Investigation complete: {}", report_path.display());

    // 5. If root cause found, offer to implement fix
    if investigation.has_root_cause() {
        if options.auto_fix || prompt_user("Implement fix now?")? {
            run_bug_fix(&bug_id, &investigation, &worktree).await?;
        }
    } else {
        warn!("Could not identify root cause. Manual investigation required.");
    }

    Ok(())
}
```

### 3.2 Worktree Structure

```
<project>/                          # Main repo
├── .git/
├── src/
└── .autoflow/

../bugfix-login-mobile/             # Bug fix worktree
├── .git -> <project>/.git/worktrees/bugfix-login-mobile/
├── src/
│   └── styles/
│       └── login.css              # File being fixed
├── .autoflow/
│   ├── bugs/
│   │   ├── BUG-001-ANALYSIS.md    # Investigation report
│   │   └── BUG-001-FIX.md         # Fix documentation
│   └── BUGFIX.yml                 # Bug tracking
└── tests/
    └── e2e/
        └── login-mobile.spec.ts   # Regression test
```

### 3.3 Multiple Concurrent Bug Fixes

```bash
# Terminal 1: Fix login bug
autoflow fix "Login button mobile"
# → worktree: ../bugfix-login-mobile/

# Terminal 2: Fix checkout bug (concurrent)
autoflow fix "Checkout total calculation wrong"
# → worktree: ../bugfix-checkout-total/

# Terminal 3: Continue feature development
autoflow start --sprint=5
# → worktree: ../sprint-5/

# All isolated, no conflicts!
```

---

## 4. Playwright MCP Integration

### 4.1 Playwright MCP Server

**Setup**: Install Playwright MCP server globally

```bash
# Install Playwright MCP
npm install -g @playwright/mcp-server

# Configure in ~/.autoflow/config.toml
[mcp]
servers = [
  { name = "playwright", type = "stdio", command = "playwright-mcp" }
]
```

### 4.2 Playwright MCP Tools

```typescript
// Available Playwright MCP tools

// Launch browser
mcp__playwright__launch({
  headless: false,
  viewport: { width: 375, height: 812 }  // iPhone size
})

// Navigate
mcp__playwright__navigate("http://localhost:3000/login")

// Interact with page
mcp__playwright__click("button[type='submit']")
mcp__playwright__fill("input[name='email']", "user@example.com")
mcp__playwright__press("Enter")

// Inspect elements
mcp__playwright__element("button[type='submit']")
mcp__playwright__computed_styles("button[type='submit']")
mcp__playwright__get_attribute("button", "disabled")

// Get page state
mcp__playwright__console_messages()
mcp__playwright__network_requests()
mcp__playwright__localstorage()
mcp__playwright__cookies()

// Visual debugging
mcp__playwright__screenshot("bug-state.png")
mcp__playwright__record_video({ path: "reproduction.webm" })

// Execute JavaScript
mcp__playwright__evaluate("window.location.href")
mcp__playwright__evaluate("document.querySelector('button').offsetWidth")

// Wait for conditions
mcp__playwright__wait_for_selector("button:visible")
mcp__playwright__wait_for_network_idle()
mcp__playwright__wait_for_function("document.readyState === 'complete'")
```

### 4.3 Interactive Debugging Session

**Bug investigator uses Playwright MCP**:

```markdown
## Investigation Session

### Step 1: Launch browser
```javascript
await mcp__playwright__launch({ headless: false });
// ✅ Browser launched
```

### Step 2: Navigate to login
```javascript
await mcp__playwright__navigate("http://localhost:3000/login");
// ✅ Page loaded
```

### Step 3: Set mobile viewport
```javascript
await mcp__playwright__set_viewport({ width: 375, height: 812 });
// ✅ Viewport: iPhone 12
```

### Step 4: Inspect login button
```javascript
const button = await mcp__playwright__element("button[type='submit']");
// Result: Button found

const styles = await mcp__playwright__computed_styles(button);
// Result: { display: "none", visibility: "visible", ... }
```

**🔴 FOUND**: Button has `display: none` on mobile!

### Step 5: Find CSS rule
```javascript
const rules = await mcp__playwright__matching_css_rules(button);
// Result:
// [
//   {
//     selector: ".login-button",
//     property: "display",
//     value: "none",
//     source: "src/styles/login.css:45",
//     media: "@media (min-width: 768px)"
//   }
// ]
```

**🎯 ROOT CAUSE**: Media query inverted! Should be `max-width`, not `min-width`.

### Step 6: Verify fix hypothesis
```javascript
// Override CSS to test fix
await mcp__playwright__add_style_tag(`
  @media (max-width: 768px) {
    .login-button { display: block !important; }
  }
`);

// Try clicking now
await mcp__playwright__click("button[type='submit']");
// ✅ Click succeeded! Button now works.
```

**✅ FIX CONFIRMED**: Changing media query fixes the issue.
```

### 4.4 Automated Test Generation

After bug is fixed, generate regression test:

```typescript
// tests/e2e/login-mobile.spec.ts (auto-generated)

import { test, expect } from '@playwright/test';

test.describe('Login on Mobile', () => {
  test('login button visible and clickable on mobile viewport', async ({ page }) => {
    // Set mobile viewport (iPhone 12)
    await page.setViewportSize({ width: 375, height: 812 });

    // Navigate to login
    await page.goto('http://localhost:3000/login');

    // Verify button visible
    const button = page.locator('button[type="submit"]');
    await expect(button).toBeVisible();

    // Verify button clickable
    await expect(button).toBeEnabled();

    // Fill form
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');

    // Click login button
    await button.click();

    // Verify navigation to dashboard
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test('login button visible on various mobile viewports', async ({ page }) => {
    const viewports = [
      { width: 375, height: 812, name: 'iPhone 12' },
      { width: 360, height: 800, name: 'Galaxy S21' },
      { width: 414, height: 896, name: 'iPhone 14 Pro Max' },
    ];

    for (const viewport of viewports) {
      await page.setViewportSize(viewport);
      await page.goto('http://localhost:3000/login');

      const button = page.locator('button[type="submit"]');
      await expect(button).toBeVisible();
    }
  });
});
```

---

## 5. Bug Fix Pipeline

### 5.1 Bug Fixer Agent

**New Agent**: `bug-fixer.agent.md`

```markdown
---
name: bug-fixer
description: Implement bug fixes based on investigation findings with minimal changes
tools: Read, Edit, Write, Bash, mcp__serena__*, mcp__context7__*
model: claude-sonnet-4-5-20250929
---

# Role
Expert bug fixer who implements minimal, surgical fixes based on root cause analysis.

# Principles
1. **Minimal changes**: Only modify what's necessary
2. **Preserve behavior**: Don't refactor while fixing
3. **Add tests**: Always add regression test
4. **Document**: Comment why fix is needed

# Workflow

## Step 1: Load Investigation Report
Read `.autoflow/bugs/BUG-{id}-ANALYSIS.md`:
- Root cause
- Files to modify
- Fix strategy
- Test strategy

## Step 2: Implement Fix
Make minimal changes to fix root cause:

```diff
# src/styles/login.css

- @media (min-width: 768px) {
+ @media (max-width: 768px) {
    .login-button {
-     display: none;
+     display: block;
    }
  }
```

## Step 3: Add Regression Test
Create E2E test to prevent recurrence:
- Test the exact bug scenario
- Cover edge cases identified in investigation
- Use Playwright with mobile viewports

## Step 4: Update Documentation
Add comment explaining the fix:

```css
/* Fix: BUG-001 - Login button hidden on mobile
 * Issue: Media query was inverted (min-width instead of max-width)
 * Date: 2025-11-05
 * Note: Always test mobile viewports in E2E tests
 */
@media (max-width: 768px) {
  .login-button {
    display: block;
  }
}
```

## Step 5: Verify Fix
Run tests to confirm:
1. Bug is fixed (reproduction steps now work)
2. No regressions (existing tests pass)
3. New test catches the bug (if CSS reverted, test fails)

## Output
Create `.autoflow/bugs/BUG-{id}-FIX.md` documenting the fix.
```

### 5.2 Bug Fix Execution

```rust
// crates/autoflow-core/src/bugfix/executor.rs

pub async fn run_bug_fix(
    bug_id: &str,
    investigation: &Investigation,
    worktree: &Worktree,
) -> Result<()> {
    info!("Implementing fix for {}", bug_id);

    // 1. Run bug-fixer agent
    let agent_executor = AgentExecutor::new();
    let fix_result = agent_executor.run_in_worktree(
        "bug-fixer",
        &BugFixContext {
            bug_id,
            investigation,
        },
        worktree,
    ).await?;

    // 2. Run tests
    info!("Running tests to verify fix...");
    let test_runner = TestRunner::new();

    // 2a. Run reproduction test (should now pass)
    let repro_test = test_runner.run_reproduction_test(bug_id, worktree).await?;
    if !repro_test.passed {
        return Err(AutoFlowError::BugNotFixed {
            bug_id: bug_id.to_string(),
            reason: "Reproduction test still fails".into(),
        });
    }

    // 2b. Run existing tests (check for regressions)
    let existing_tests = test_runner.run_all_tests(worktree).await?;
    if existing_tests.has_failures() {
        return Err(AutoFlowError::RegressionDetected {
            bug_id: bug_id.to_string(),
            failing_tests: existing_tests.failures(),
        });
    }

    // 2c. Run new regression test
    let regression_test = test_runner.run_new_tests(worktree).await?;
    if !regression_test.passed {
        return Err(AutoFlowError::RegressionTestFailed {
            bug_id: bug_id.to_string(),
        });
    }

    success!("Fix verified! All tests pass.");

    // 3. Commit fix
    let git = GitOperations::new();
    git.commit_in_worktree(
        worktree,
        &format!("fix: {} ({})", investigation.description, bug_id),
    ).await?;

    // 4. Offer to merge
    if prompt_user("Merge fix to main branch?")? {
        worktree_manager.merge_and_cleanup(worktree).await?;
        success!("Bug fix merged to main!");
    } else {
        info!("Worktree preserved at: {}", worktree.path.display());
        info!("Merge later with: autoflow worktree merge {}", worktree.branch);
    }

    Ok(())
}
```

### 5.3 Bug Fix Validation

```rust
// crates/autoflow-quality/src/bugfix_validator.rs

pub struct BugfixValidator;

impl BugfixValidator {
    pub async fn validate(&self, bug_id: &str, worktree: &Worktree) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();

        // 1. Verify root cause addressed
        let analysis = load_bug_analysis(bug_id)?;
        for file in &analysis.root_cause.files_to_modify {
            if !was_file_modified_in_worktree(file, worktree)? {
                report.add_error(format!(
                    "Expected to modify {} but file unchanged",
                    file
                ));
            }
        }

        // 2. Verify minimal changes
        let diff = git_diff_worktree(worktree)?;
        if diff.changed_lines > analysis.root_cause.expected_changes * 2 {
            report.add_warning(format!(
                "Fix modified {} lines, expected ~{}. Consider if changes are minimal.",
                diff.changed_lines,
                analysis.root_cause.expected_changes
            ));
        }

        // 3. Verify regression test added
        let new_tests = find_new_test_files(worktree)?;
        if new_tests.is_empty() {
            report.add_error("No regression test added. Bug may recur.");
        }

        // 4. Verify documentation
        let has_comment = check_for_bug_comment(bug_id, worktree)?;
        if !has_comment {
            report.add_warning("No comment documenting the fix in code.");
        }

        Ok(report)
    }
}
```

---

## 6. CLI Commands

### 6.1 Bug Fix Commands

```bash
# Start bug fix workflow
autoflow fix "Login button doesn't work on mobile"

# With options
autoflow fix "Checkout total wrong" \
  --auto-fix \                    # Auto-implement fix after investigation
  --no-worktree \                 # Fix in main branch (not recommended)
  --playwright-headed             # Launch browser in headed mode

# Interactive investigation only (no auto-fix)
autoflow investigate "Why is page loading slow?"

# List active bug fix worktrees
autoflow worktree list --type=bugfix

# Show bug investigation
autoflow bug show BUG-001

# Merge bug fix
autoflow worktree merge bugfix/login-mobile

# Rollback bug fix
autoflow worktree rollback bugfix/login-mobile
```

### 6.2 Playwright Commands

```bash
# Launch Playwright in investigation mode
autoflow debug --playwright

# Record reproduction steps
autoflow debug --record "Login button mobile bug"

# Generate test from recording
autoflow test generate --from-recording bug-reproduction.json
```

### 6.3 Worktree Commands

```bash
# List all worktrees
autoflow worktree list

# Output:
# main            /path/to/project       (main branch)
# sprint-5        ../sprint-5/           (feature development)
# bugfix-login    ../bugfix-login/       (bug fix in progress)
# bugfix-checkout ../bugfix-checkout/    (bug fix in progress)

# Switch to worktree
autoflow worktree switch bugfix-login

# Merge worktree
autoflow worktree merge bugfix-login

# Delete worktree
autoflow worktree delete bugfix-login --force

# Clean up merged worktrees
autoflow worktree prune
```

---

## 7. Implementation

### 7.1 Core Data Structures

```rust
// crates/autoflow-data/src/bugfix.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugAnalysis {
    pub bug_id: String,
    pub description: String,
    pub reported_at: DateTime<Utc>,
    pub priority: BugPriority,

    pub reproduction: Reproduction,
    pub root_cause: RootCause,
    pub fix_plan: FixPlan,
    pub impact: Impact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reproduction {
    pub steps: Vec<String>,
    pub reproducible: bool,
    pub environments: Vec<Environment>,
    pub playwright_session: Option<String>,  // Path to recorded session
    pub screenshots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    pub identified: bool,
    pub cause_type: CauseType,
    pub file: String,
    pub line: Option<u32>,
    pub description: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CauseType {
    CssIssue,
    JavaScriptError,
    NetworkError,
    StateManagement,
    DatabaseQuery,
    RaceCondition,
    ConfigurationError,
    DependencyIssue,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPlan {
    pub approach: String,
    pub files_to_modify: Vec<String>,
    pub tests_to_add: Vec<String>,
    pub estimated_time: String,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugFix {
    pub bug_id: String,
    pub fix_id: String,
    pub implemented_at: DateTime<Utc>,
    pub worktree: String,
    pub branch: String,

    pub changes: Vec<FileChange>,
    pub tests_added: Vec<String>,
    pub verification: TestResults,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BugPriority {
    Critical,  // Production broken
    High,      // Major feature broken
    Medium,    // Minor feature broken
    Low,       // Cosmetic issue
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,      // Single line change, well tested
    Medium,   // Multiple files, good test coverage
    High,     // Core functionality, limited tests
    Critical, // High impact area, complex change
}
```

### 7.2 Worktree Manager Enhancement

```rust
// crates/autoflow-git/src/worktree.rs

impl WorktreeManager {
    pub async fn create_bugfix_worktree(&self, bug_description: &str) -> Result<Worktree> {
        let branch_name = format!("bugfix/{}", slugify(bug_description));
        let worktree_path = self.worktree_base.join(&branch_name);

        // Create from current main branch
        let repo = Repository::open(&self.repo_path)?;
        let main = repo.find_branch("main", BranchType::Local)?;
        let commit = main.get().peel_to_commit()?;

        // Create branch
        repo.branch(&branch_name, &commit, false)?;

        // Create worktree
        repo.worktree(&branch_name, &worktree_path, None)?;

        // Set up environment (Docker, MCP servers)
        self.setup_worktree_environment(&worktree_path).await?;

        Ok(Worktree {
            branch: branch_name,
            path: worktree_path,
            worktree_type: WorktreeType::Bugfix,
            created_at: Utc::now(),
        })
    }

    pub async fn list_worktrees(&self) -> Result<Vec<Worktree>> {
        let repo = Repository::open(&self.repo_path)?;
        let mut worktrees = vec![];

        for worktree_name in repo.worktrees()? {
            let wt = repo.find_worktree(&worktree_name)?;
            let path = wt.path().to_path_buf();

            // Determine worktree type from branch name
            let worktree_type = if worktree_name.starts_with("sprint-") {
                WorktreeType::Sprint
            } else if worktree_name.starts_with("bugfix-") {
                WorktreeType::Bugfix
            } else {
                WorktreeType::Other
            };

            worktrees.push(Worktree {
                branch: worktree_name,
                path,
                worktree_type,
                created_at: Utc::now(), // Would read from git
            });
        }

        Ok(worktrees)
    }

    pub async fn merge_bugfix(&self, worktree: &Worktree) -> Result<()> {
        let repo = Repository::open(&self.repo_path)?;

        // Switch to main
        let main = repo.find_branch("main", BranchType::Local)?;
        repo.set_head(main.get().name().unwrap())?;

        // Merge bugfix branch
        let bugfix = repo.find_branch(&worktree.branch, BranchType::Local)?;
        let bugfix_commit = bugfix.get().peel_to_commit()?;

        let mut index = repo.index()?;
        let main_commit = main.get().peel_to_commit()?;

        repo.merge_commits(&main_commit, &bugfix_commit, None)?;

        // If no conflicts, commit merge
        if !index.has_conflicts() {
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            repo.commit(
                Some("HEAD"),
                &repo.signature()?,
                &repo.signature()?,
                &format!("Merge {}", worktree.branch),
                &tree,
                &[&main_commit, &bugfix_commit],
            )?;

            // Clean up worktree and branch
            repo.worktree_prune(&worktree.branch, None)?;
            let mut branch = repo.find_branch(&worktree.branch, BranchType::Local)?;
            branch.delete()?;

            success!("Bug fix merged and worktree cleaned up");
        } else {
            return Err(AutoFlowError::MergeConflict {
                branch: worktree.branch.clone(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorktreeType {
    Sprint,   // Feature development
    Bugfix,   // Bug fixes
    Other,    // Manual worktrees
}
```

### 7.3 Playwright MCP Client

```rust
// crates/autoflow-agents/src/playwright_client.rs

pub struct PlaywrightClient {
    mcp_client: McpClient,
}

impl PlaywrightClient {
    pub async fn launch(&self, options: LaunchOptions) -> Result<BrowserContext> {
        let result = self.mcp_client.call_tool(
            "playwright__launch",
            json!({
                "headless": options.headless,
                "viewport": {
                    "width": options.viewport.width,
                    "height": options.viewport.height,
                }
            })
        ).await?;

        Ok(BrowserContext {
            context_id: result["contextId"].as_str().unwrap().to_string(),
        })
    }

    pub async fn navigate(&self, url: &str) -> Result<()> {
        self.mcp_client.call_tool(
            "playwright__navigate",
            json!({ "url": url })
        ).await?;

        Ok(())
    }

    pub async fn click(&self, selector: &str) -> Result<()> {
        self.mcp_client.call_tool(
            "playwright__click",
            json!({ "selector": selector })
        ).await?;

        Ok(())
    }

    pub async fn screenshot(&self, path: &str) -> Result<()> {
        self.mcp_client.call_tool(
            "playwright__screenshot",
            json!({ "path": path })
        ).await?;

        Ok(())
    }

    pub async fn computed_styles(&self, selector: &str) -> Result<HashMap<String, String>> {
        let result = self.mcp_client.call_tool(
            "playwright__computed_styles",
            json!({ "selector": selector })
        ).await?;

        Ok(serde_json::from_value(result)?)
    }

    pub async fn console_messages(&self) -> Result<Vec<ConsoleMessage>> {
        let result = self.mcp_client.call_tool(
            "playwright__console_messages",
            json!({})
        ).await?;

        Ok(serde_json::from_value(result)?)
    }
}

pub struct LaunchOptions {
    pub headless: bool,
    pub viewport: Viewport,
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
```

---

## 8. Example: Complete Bug Fix Flow

```bash
# User reports bug
autoflow fix "Checkout total shows $0.00 instead of cart total"

# AutoFlow creates worktree
# → Created worktree: ../bugfix-checkout-total/
# → Branch: bugfix/checkout-total

# Investigation starts (bug-investigator agent)
# 🔍 Investigating bug...
# 🔍 Reading error logs...
# 🔍 Checking recent commits...
# 🔍 Launching Playwright to reproduce...

# Playwright MCP session
# → Navigating to checkout page
# → Adding items to cart
# → Proceeding to checkout
# 🔴 BUG CONFIRMED: Total shows $0.00
# → Inspecting checkout component
# → Checking state management
# → Found: cartSlice.selectTotal returns undefined
# 🎯 ROOT CAUSE: Selector typo in Redux slice

# Investigation complete
# ✅ Root cause: src/store/cartSlice.ts:45
# ✅ Issue: Selector uses 'cart.totals' should be 'cart.total'
# 📄 Analysis saved: .autoflow/bugs/BUG-002-ANALYSIS.md

# Implement fix? [Y/n] y

# Fixing bug (bug-fixer agent)
# → Modifying src/store/cartSlice.ts
# → Adding regression test: tests/e2e/checkout-total.spec.ts
# ✅ Fix implemented

# Running tests...
# → Reproduction test: ✅ PASS (checkout total now correct)
# → Existing tests: ✅ 245/245 PASS
# → Regression test: ✅ PASS

# All tests passed!

# Merge fix to main? [Y/n] y

# Merging bugfix/checkout-total to main...
# ✅ Merged successfully
# ✅ Worktree cleaned up
#
# Bug fix complete! 🎉
# - Bug ID: BUG-002
# - Fix: src/store/cartSlice.ts (1 line changed)
# - Test: tests/e2e/checkout-total.spec.ts (added)
# - Time: 8 minutes
```

---

## 9. Key Decisions

### ✅ **Git Worktree Per Bug**
- Isolated bug fix workspace
- No conflicts with main development
- Easy rollback if fix fails
- Multiple concurrent bug fixes

### ✅ **Playwright MCP for Investigation**
- Interactive browser debugging
- Element inspection
- Console/network monitoring
- Visual verification

### ✅ **Always Add Regression Test**
- Prevent bug from recurring
- Document expected behavior
- Test exact bug scenario

### ✅ **Minimal Changes**
- Only fix root cause
- No refactoring while fixing
- Preserve existing behavior

### ✅ **Autonomous Investigation**
- Agent reads code/logs/git history
- Uses Playwright to reproduce
- Identifies root cause
- Generates fix plan

---

## Next Steps

1. **Implement `bug-investigator` agent** (Week 6)
2. **Implement `bug-fixer` agent** (Week 6)
3. **Add `autoflow fix` command** (Week 6)
4. **Integrate Playwright MCP** (Week 6)
5. **Enhance worktree manager for bug fixes** (Week 7)
6. **Add bugfix quality gate** (Week 8)

**Ready to autonomously fix bugs!** 🐛🔧
