# AutoFlow Update Strategy

## User Experience Goals

1. **Non-Destructive** - Never overwrite user customizations
2. **Transparent** - Show what will change before changing it
3. **Reversible** - Automatic backups of replaced files
4. **Selective** - Update only what's needed
5. **Safe** - Dry-run mode to preview

## Update Mechanisms

### 1. Update Script

```bash
# Preview changes (safe)
./scripts/update.sh --dry-run

# Update everything
./scripts/update.sh

# Update only agents
./scripts/update.sh --agents

# Update only skills
./scripts/update.sh --skills

# Force update even customized files
./scripts/update.sh --force
```

### 2. Detection of Customization

A file is considered "customized" if:
- Has `.backup-*` files (was previously updated)
- Modified within last 7 days
- Use `--force` to override

### 3. Backup Strategy

Every replaced file is backed up:
```
~/.claude/agents/reviewer.agent.md.backup-20251106-150530
~/.claude/skills/owasp-security-audit/SKILL.md.backup-20251106-150545
```

Restore if needed:
```bash
cp ~/.claude/agents/reviewer.agent.md.backup-20251106-150530 \
   ~/.claude/agents/reviewer.agent.md
```

### 4. Version Tracking

```
autoflow/
├── VERSION                    # 0.1.0
├── agents/
│   ├── reviewer.md
│   └── .meta/
│       └── reviewer.version   # 1.2.0
└── skills/
    └── owasp-security-audit/
        ├── SKILL.md
        └── .version           # 1.0.1
```

## Migration Paths

### Scenario 1: Fresh Install
```bash
git clone repo
./scripts/install.sh
# All agents/skills installed fresh
```

### Scenario 2: Update Existing Installation
```bash
cd autoflow
git pull
./scripts/update.sh --dry-run  # Preview
./scripts/update.sh            # Apply
```

### Scenario 3: User Customized Files
```bash
# User edited reviewer.agent.md
./scripts/update.sh
# Output: ⚠ Skipping reviewer (appears customized)

# To update anyway:
./scripts/update.sh --force
# Output: ↻ Updating reviewer
#         → Backed up to: reviewer.agent.md.backup-20251106-150530
```

### Scenario 4: New Agent/Skill Added
```bash
git pull  # New "prisma-best-practices" skill added
./scripts/update.sh
# Output: + Installing new skill: prisma-best-practices
```

## Future: CLI Integration

```bash
# Check for updates
autoflow update --check

# Update from latest release
autoflow update

# Update from specific version
autoflow update --version 0.2.0

# Update from local repo
autoflow update --local /path/to/autoflow
```

## Distribution Strategy

### Option 1: Git-Based (Current)
```bash
cd ~/.autoflow/repo
git pull
./scripts/update.sh
```

**Pros:**
- Simple
- Users can contribute
- See history

**Cons:**
- Requires repo checkout
- Manual process

### Option 2: Package Registry (Future)
```bash
# npm (if we publish)
npm install -g @autoflow/cli

# cargo (if we publish)
cargo install autoflow

# Update built-in
autoflow update
```

**Pros:**
- Standard distribution
- Automatic updates
- Version management

**Cons:**
- More complex
- Publishing overhead

### Option 3: Self-Updating Binary (Recommended)
```bash
# Check for updates
autoflow update --check
# Output: New version available: 0.2.0 (you have: 0.1.0)

# Update
autoflow update
# Downloads new binary
# Updates agents/skills from GitHub release
```

**Pros:**
- Best UX
- Standard for CLI tools
- Can bundle agents/skills

**Cons:**
- Need release infrastructure

## Versioning Scheme

### AutoFlow Binary
- **Major**: Breaking changes to CLI/workflow
- **Minor**: New features, agents, skills
- **Patch**: Bug fixes

Example: `0.2.1`
- 0 = Alpha/beta
- 2 = Added agent orchestration
- 1 = Fixed skill loading bug

### Agents/Skills
Independent versioning:
- **Major**: Breaking API changes
- **Minor**: New features
- **Patch**: Bug fixes, improvements

Example: `reviewer.agent.md` v2.1.0
- 2 = Changed to JSON output format
- 1 = Added OWASP Top 10 checks
- 0 = Initial 2.x release

## Changelog

### VERSION file
```
0.1.0
```

### CHANGELOG.md
```markdown
# Changelog

## [0.2.0] - 2025-01-15

### Added
- New `prisma-best-practices` skill
- New `graphql-optimization` skill
- Update command: `autoflow update`

### Changed
- `reviewer` agent now outputs JSON instead of markdown
- `owasp-security-audit` skill updated with 2025 OWASP Top 10

### Fixed
- Skills installation with directory structure

## [0.1.0] - 2025-01-06

### Added
- Initial release
- 13 agents (complete TDD pipeline)
- 19 skills
```

## User Communication

### On Update Available
```
$ autoflow status

⚠️  Update available: 0.2.0 (you have 0.1.0)

What's new:
  • 2 new skills (prisma-best-practices, graphql-optimization)
  • reviewer agent now includes security checks
  • Bug fixes for SPRINTS.yml generation

Run `autoflow update` to update
Run `autoflow update --dry-run` to preview changes
```

### During Update
```
$ autoflow update

🔄 Updating AutoFlow 0.1.0 → 0.2.0...

🤖 Updating agents...
  ✓ reviewer (already up to date)
  ↻ Updating code-implementer
  → Backed up to: code-implementer.agent.md.backup-20251106-150530
  + Installing new agent: api-designer
  ⚠ Skipping test-writer (appears customized, use --force to override)

  ✓ 2 agents updated
  → 1 agent skipped (customized)

🛠️  Updating skills...
  + Installing new skill: prisma-best-practices
  + Installing new skill: graphql-optimization
  ✓ owasp-security-audit (already up to date)

  ✓ 2 skills added

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✅ Update complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 Tips:
  • Backups stored in ~/.claude/agents/*.backup-*
  • Use `autoflow update --dry-run` next time to preview
  • Customized files not updated (use --force if needed)
```

## Implementation Phases

### Phase 1: Manual Update Script ✅ (Done)
- `scripts/update.sh` with dry-run
- Automatic backups
- Customization detection

### Phase 2: CLI Integration (Next)
- `autoflow update` command
- Integrate update.sh logic into Rust
- Better UX with colored output

### Phase 3: Version Checking
- Check GitHub releases for updates
- Notify on `autoflow status`
- Download and verify releases

### Phase 4: Self-Updating Binary
- Download new binary
- Atomic replacement
- Rollback on failure

### Phase 5: Plugin System
- Third-party agents/skills
- Registry/marketplace
- `autoflow install plugin-name`

## Security Considerations

1. **Verify checksums** - SHA256 for downloads
2. **HTTPS only** - For update checks
3. **Signed releases** - GPG signatures
4. **User consent** - Always ask before updating
5. **Rollback** - Keep previous version as backup

## Testing Update Flow

```bash
# Test 1: Fresh install
./scripts/install.sh

# Test 2: No changes
./scripts/update.sh --dry-run
# Expected: All up to date

# Test 3: Simulate new skill
echo "new content" > skills/test-skill/SKILL.md
./scripts/update.sh --dry-run
# Expected: + Installing new skill: test-skill

# Test 4: Simulate customization
touch -t 202511061500 ~/.claude/skills/owasp-security-audit/SKILL.md
./scripts/update.sh --dry-run
# Expected: ⚠ Skipping owasp-security-audit (recently modified)

# Test 5: Force update
./scripts/update.sh --force
# Expected: ↻ Updating owasp-security-audit (with backup)
```

## Backward Compatibility

### Breaking Changes
If we make breaking changes:
1. Bump major version (0.x.x → 1.0.0)
2. Add migration guide
3. Keep backward compatibility for 1 minor version

### Deprecation
```markdown
## agent/old-pattern.md

⚠️ DEPRECATED: This agent will be removed in v0.3.0
Use `new-pattern` agent instead.

Migration:
  old: autoflow run old-pattern
  new: autoflow run new-pattern --mode legacy
```

## Future: Plugin Ecosystem

```bash
# Install community plugin
autoflow plugin install community/fastapi-crud

# List installed
autoflow plugin list

# Update plugins
autoflow plugin update

# Create plugin
autoflow plugin create my-custom-agent
```

Registry structure:
```
https://autoflow.dev/plugins/
├── official/
│   ├── owasp-security-audit
│   └── jest-to-vitest
└── community/
    ├── fastapi-crud
    └── vue-composables
```
