# AutoFlow Release & Update Strategy

## Release Distribution Methods

### 1. Pre-built Binaries (Primary Method)

**Platforms:**
- `x86_64-unknown-linux-gnu` (Linux)
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-pc-windows-msvc` (Windows)

**Build Process:**
- GitHub Actions CI/CD builds all platforms on tag push
- Releases published to GitHub Releases
- Each release includes: binary + agents + skills + templates

**Installation Script:**
```bash
curl -sSL https://raw.githubusercontent.com/autoflow/autoflow/main/install.sh | sh
```

The install script:
1. Detects platform
2. Downloads appropriate release from GitHub
3. Extracts to `~/.autoflow/`
4. Sets up PATH
5. Syncs agents/skills to `~/.claude/`

### 2. Cargo Install (Secondary Method)

```bash
cargo install autoflow
```

For users who:
- Want to build from source
- Need specific compile flags
- Are developing/contributing

### 3. Package Managers (Future)

**Homebrew (High Priority):**
```bash
brew install autoflow
```
- Large user base
- Auto-update support
- Familiar to developers

**Other Package Managers (Lower Priority):**
- APT/Debian packages
- AUR (Arch User Repository)
- Chocolatey (Windows)
- Scoop (Windows)

---

## Version Numbering

Follow Semantic Versioning (SemVer):
```
MAJOR.MINOR.PATCH
```

- **MAJOR**: Breaking changes (sprint format changes, API changes)
- **MINOR**: New features (new agents, new workflow types)
- **PATCH**: Bug fixes, agent improvements

Examples:
- `0.1.0` → `0.2.0`: Add new workflow type
- `0.1.0` → `0.1.1`: Fix agent bug
- `1.0.0` → `2.0.0`: Change SPRINTS.yml format

---

## Update Mechanism

### Auto-Update Check

Every `autoflow` command automatically:
1. Checks for updates (cached daily)
2. Shows notification if newer version available
3. **Does NOT auto-install** (user consent required)

```
╭─────────────────────────────────────╮
│ Update Available: v0.2.0            │
│ Current: v0.1.0                     │
│                                     │
│ Run: autoflow update                │
╰─────────────────────────────────────╯
```

### Manual Update Command

```bash
autoflow update
```

Process:
1. Check current version vs latest GitHub release
2. Show changelog/release notes
3. Ask for confirmation
4. Download new binary for current platform
5. Replace binary (with backup)
6. Auto-sync agents/skills
7. Verify installation

**Safety:**
- Backup old binary to `~/.autoflow/bin/autoflow.backup`
- Rollback on failure
- Verify checksums

---

## Agent & Skill Updates

### Automatic Sync (Current Implementation)

**On Every Command:**
- Agents/skills synced from bundled directory to `~/.claude/`
- Only updates if source is newer (timestamp-based)
- Silent operation (no user notification)

**Benefits:**
- Always up-to-date
- No manual action required
- Works seamlessly with binary updates

### User Override

**Project-Level Agents:**
```
project/
  .autoflow/
    agents/
      custom-implementer.md  # Overrides bundled agent
```

**Search Order:**
1. `./agents/` (project-local)
2. `~/.claude/agents/` (user's agents, auto-synced)

This allows:
- Custom agents per project
- Testing new agent versions
- Project-specific modifications

---

## Release Checklist

### Pre-Release

- [ ] Update version in `Cargo.toml`
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Test on all platforms (Linux, macOS, Windows)
- [ ] Review agent changes
- [ ] Update documentation

### Release

- [ ] Create Git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] GitHub Actions builds binaries
- [ ] Create GitHub Release with:
  - Release notes
  - Binaries for all platforms
  - Checksums (SHA256)
  - Bundled agents/skills

### Post-Release

- [ ] Test install script on fresh system
- [ ] Verify auto-update notification works
- [ ] Update website/documentation
- [ ] Announce on Twitter/Discord/Reddit

---

## GitHub Actions CI/CD

### Build Workflow

**Trigger:** Push to tag matching `v*`

**Jobs:**
1. **Build Matrix:**
   - Build for all platforms in parallel
   - Run tests on each platform
   - Generate checksums

2. **Bundle Assets:**
   - Package binary with agents/skills/templates
   - Create `.tar.gz` archives

3. **Create Release:**
   - Upload all platform binaries
   - Upload checksums
   - Auto-generate release notes from commits

### Example `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Bundle
        run: |
          mkdir -p release
          cp target/${{ matrix.target }}/release/autoflow release/
          cp -r agents skills templates schemas release/
          tar czf autoflow-${{ matrix.target }}.tar.gz -C release .

      - name: Upload
        uses: actions/upload-artifact@v3
        with:
          name: autoflow-${{ matrix.target }}
          path: autoflow-${{ matrix.target }}.tar.gz
```

---

## Update Server / API

### Version Check Endpoint

Users need to check for updates. Options:

**Option 1: GitHub Releases API (Recommended)**
```
GET https://api.github.com/repos/autoflow/autoflow/releases/latest
```

Pros:
- No infrastructure needed
- Built-in caching
- Reliable

Cons:
- Rate limiting (60 req/hour unauthenticated)
- Need to cache locally

**Option 2: Self-Hosted API**
```
GET https://api.autoflow.dev/version/latest
```

Pros:
- No rate limits
- Can track usage metrics
- Custom response format

Cons:
- Infrastructure costs
- Maintenance overhead

**Recommendation:** Start with GitHub API, move to self-hosted if needed.

---

## Distribution Channels

### GitHub (Primary)
- Source code
- Pre-built binaries
- Issue tracking
- Community discussions

### Website
- Documentation
- Tutorials
- Blog/changelog
- Download links

### Package Managers
- Homebrew tap
- crates.io (Cargo)
- Future: APT, AUR, etc.

---

## Backward Compatibility

### Sprint Format Changes

**CRITICAL:** SPRINTS.yml format changes require migration.

**Strategy:**
1. Detect old format version
2. Auto-migrate with backup
3. Warn user of changes
4. Only in MAJOR version bumps

**Example Migration:**
```bash
autoflow migrate --from 0.1.0 --to 1.0.0
```

Creates:
- `.autoflow/SPRINTS.yml.backup`
- Migrated `.autoflow/SPRINTS.yml`

### Agent API Changes

Agents use Claude CLI, which is stable. Changes:
- Add new tools → Minor version
- Change agent behavior → Minor version
- Remove tools → Major version

---

## Monitoring & Analytics (Optional)

### Telemetry (Opt-In)

```bash
autoflow telemetry enable
```

Collect (anonymized):
- Version usage
- Platform distribution
- Command usage frequency
- Error rates

**Privacy:**
- Opt-in only
- No code or project data
- Anonymized identifiers
- Open source telemetry code

### Update Success Rates

Track:
- Update check failures
- Download failures
- Installation failures

Use for:
- Improving install script
- Better error messages
- Platform-specific fixes

---

## Future Enhancements

### 1. Plugin System

```bash
autoflow plugin install github.com/user/custom-workflow
```

Allow:
- Custom workflow types
- Custom agents
- Custom quality gates

### 2. Enterprise Support

- Private agent repositories
- Air-gapped installations
- License management
- Priority support

### 3. Docker Distribution

```bash
docker run -v $(pwd):/project autoflow/autoflow start
```

Benefits:
- Consistent environment
- No installation needed
- Easy CI/CD integration

---

## Questions to Resolve

1. **License:** MIT? Apache 2.0? GPL?
   - Recommendation: MIT (most permissive, best for adoption)

2. **Branding:** Name, logo, domain
   - Domain: autoflow.dev?
   - Logo needed for package managers

3. **Support:** Issue tracker only, or Discord/forum?
   - Start with GitHub Issues
   - Add Discord when community grows

4. **Pricing:** Free forever? Paid enterprise features?
   - Recommendation: Open source core, optional enterprise features

5. **Hosting:** Where to host agents/templates?
   - GitHub repo (included in release)
   - Separate repo for extensibility?

6. **Update Frequency:** Patch releases weekly? Monthly?
   - Recommendation:
     - Patches: As needed (critical bugs)
     - Minors: Monthly (new features)
     - Majors: Quarterly (breaking changes)

---

## Implementation Priority

### Phase 1 (Pre-Launch)
- [x] Auto-sync agents/skills
- [ ] GitHub Actions release workflow
- [ ] Install script for all platforms
- [ ] Update check mechanism
- [ ] CHANGELOG.md

### Phase 2 (Launch)
- [ ] v0.1.0 release
- [ ] GitHub Release with binaries
- [ ] Documentation site
- [ ] `autoflow update` command

### Phase 3 (Post-Launch)
- [ ] Homebrew formula
- [ ] Usage analytics (opt-in)
- [ ] Community Discord
- [ ] Tutorial videos

### Phase 4 (Growth)
- [ ] More package managers
- [ ] Plugin system
- [ ] Enterprise features
