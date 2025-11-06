---
name: owasp-security-audit
description: Comprehensive OWASP Top 10 security audit checklist for web applications. Use when reviewing code for security vulnerabilities, implementing authentication/authorization, handling user input, or before production deployment.
---

# OWASP Security Audit Skill

Perform comprehensive security audits against OWASP Top 10 vulnerabilities.

## When to Use

- Reviewing authentication/authorization code
- Implementing user input handling
- Before production deployment
- After security-sensitive code changes
- When handling sensitive data

## OWASP Top 10 (2021) Checklist

### 1. Broken Access Control

**Check for:**
- [ ] User can't access resources without proper authorization
- [ ] URL manipulation doesn't bypass security (e.g., `/admin` accessible to regular users)
- [ ] API endpoints validate user permissions
- [ ] CORS policies properly configured
- [ ] File upload restrictions enforced

**Examples:**

```javascript
// ❌ BAD - No authorization check
app.get('/api/users/:id', async (req, res) => {
  const user = await db.users.findById(req.params.id)
  res.json(user)
})

// ✅ GOOD - Authorization enforced
app.get('/api/users/:id', requireAuth, async (req, res) => {
  if (req.user.id !== req.params.id && !req.user.isAdmin) {
    return res.status(403).json({ error: 'Forbidden' })
  }
  const user = await db.users.findById(req.params.id)
  res.json(user)
})
```

### 2. Cryptographic Failures

**Check for:**
- [ ] Passwords hashed with bcrypt/argon2 (NOT MD5/SHA1)
- [ ] HTTPS enforced (no HTTP)
- [ ] Sensitive data encrypted at rest
- [ ] Secure session cookies (httpOnly, secure, sameSite)
- [ ] No hardcoded secrets

**Examples:**

```javascript
// ❌ BAD - Weak hashing
const hash = crypto.createHash('md5').update(password).digest('hex')

// ✅ GOOD - Strong hashing
const bcrypt = require('bcrypt')
const hash = await bcrypt.hash(password, 12)

// ❌ BAD - Hardcoded secret
const JWT_SECRET = 'mysecretkey123'

// ✅ GOOD - Environment variable
const JWT_SECRET = process.env.JWT_SECRET
if (!JWT_SECRET) throw new Error('JWT_SECRET not set')
```

### 3. Injection

**Check for:**
- [ ] SQL queries use parameterized statements
- [ ] No eval() or Function() with user input
- [ ] Shell commands validated/sanitized
- [ ] NoSQL queries properly escaped
- [ ] LDAP/XML queries parameterized

**Examples:**

```javascript
// ❌ BAD - SQL Injection
const query = `SELECT * FROM users WHERE email = '${email}'`
db.query(query)

// ✅ GOOD - Parameterized query
db.query('SELECT * FROM users WHERE email = $1', [email])

// ❌ BAD - Command injection
exec(`convert ${userFile} output.pdf`)

// ✅ GOOD - Validated input
const safeFile = path.basename(userFile)
if (!/^[a-zA-Z0-9._-]+$/.test(safeFile)) {
  throw new Error('Invalid filename')
}
execFile('convert', [safeFile, 'output.pdf'])
```

### 4. Insecure Design

**Check for:**
- [ ] Rate limiting on authentication endpoints
- [ ] Account lockout after failed attempts
- [ ] Multi-factor authentication available
- [ ] Secure password reset flow
- [ ] Security requirements in design docs

**Examples:**

```javascript
// ✅ GOOD - Rate limiting
const rateLimit = require('express-rate-limit')

const loginLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 5, // 5 attempts
  message: 'Too many login attempts, try again later'
})

app.post('/api/login', loginLimiter, async (req, res) => {
  // Login logic
})
```

### 5. Security Misconfiguration

**Check for:**
- [ ] Error messages don't leak sensitive info
- [ ] Debug mode disabled in production
- [ ] Unnecessary features/ports disabled
- [ ] Security headers set (CSP, X-Frame-Options, etc.)
- [ ] Default passwords changed

**Examples:**

```javascript
// ❌ BAD - Leaking error details
catch (error) {
  res.status(500).json({ error: error.stack })
}

// ✅ GOOD - Generic error
catch (error) {
  console.error('Database error:', error)
  res.status(500).json({ error: 'Internal server error' })
}

// ✅ GOOD - Security headers
const helmet = require('helmet')
app.use(helmet())
app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", "'unsafe-inline'"]
  }
}))
```

### 6. Vulnerable and Outdated Components

**Check for:**
- [ ] npm audit / cargo audit shows no HIGH/CRITICAL
- [ ] Dependencies regularly updated
- [ ] No deprecated packages
- [ ] Dependency version pinning in place

**Commands:**

```bash
# Check vulnerabilities
npm audit
npm audit fix

# Check outdated packages
npm outdated

# Update dependencies
npm update
```

### 7. Identification and Authentication Failures

**Check for:**
- [ ] Session IDs not in URLs
- [ ] Session timeout implemented
- [ ] Passwords meet complexity requirements
- [ ] Credential stuffing protection
- [ ] Secure "forgot password" flow

**Examples:**

```javascript
// ✅ GOOD - Secure session config
app.use(session({
  secret: process.env.SESSION_SECRET,
  resave: false,
  saveUninitialized: false,
  cookie: {
    httpOnly: true,
    secure: true, // HTTPS only
    sameSite: 'strict',
    maxAge: 30 * 60 * 1000 // 30 minutes
  }
}))

// ✅ GOOD - Password requirements
function validatePassword(password) {
  if (password.length < 12) return false
  if (!/[A-Z]/.test(password)) return false
  if (!/[a-z]/.test(password)) return false
  if (!/[0-9]/.test(password)) return false
  if (!/[^A-Za-z0-9]/.test(password)) return false
  return true
}
```

### 8. Software and Data Integrity Failures

**Check for:**
- [ ] CI/CD pipeline validates code integrity
- [ ] Dependencies from trusted sources only
- [ ] Digital signatures on updates
- [ ] No auto-update without verification

### 9. Security Logging and Monitoring Failures

**Check for:**
- [ ] Authentication failures logged
- [ ] Access control failures logged
- [ ] Input validation failures logged
- [ ] Logs stored securely
- [ ] Alerting on suspicious activity

**Examples:**

```javascript
// ✅ GOOD - Security logging
const logger = require('winston')

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body

  const user = await authenticate(email, password)

  if (!user) {
    logger.warn('Failed login attempt', {
      email,
      ip: req.ip,
      userAgent: req.headers['user-agent']
    })
    return res.status(401).json({ error: 'Invalid credentials' })
  }

  logger.info('Successful login', { userId: user.id, ip: req.ip })
  // Continue login...
})
```

### 10. Server-Side Request Forgery (SSRF)

**Check for:**
- [ ] User-supplied URLs validated
- [ ] Whitelist of allowed domains
- [ ] No internal network access from user input
- [ ] Response sanitization

**Examples:**

```javascript
// ❌ BAD - SSRF vulnerability
app.get('/fetch-url', async (req, res) => {
  const url = req.query.url
  const response = await fetch(url) // Can access internal services!
  res.send(await response.text())
})

// ✅ GOOD - URL validation
const ALLOWED_DOMAINS = ['api.example.com', 'cdn.example.com']

app.get('/fetch-url', async (req, res) => {
  const url = new URL(req.query.url)

  if (!ALLOWED_DOMAINS.includes(url.hostname)) {
    return res.status(400).json({ error: 'Domain not allowed' })
  }

  if (url.hostname === 'localhost' || url.hostname.startsWith('192.168.')) {
    return res.status(400).json({ error: 'Internal URLs forbidden' })
  }

  const response = await fetch(url.toString())
  res.send(await response.text())
})
```

## Quick Audit Process

1. **Grep for common vulnerabilities:**
```bash
# Find potential SQL injection
grep -r "query.*\${" src/

# Find eval usage
grep -r "eval\(" src/

# Find hardcoded secrets
grep -ri "password.*=.*['\"]" src/
grep -ri "api[_-]?key.*=.*['\"]" src/

# Find debug code
grep -ri "console\.log\|debugger" src/
```

2. **Run automated scanners:**
```bash
npm audit
npm run lint
```

3. **Manual review:**
- Check authentication flows
- Test authorization on all endpoints
- Verify input validation
- Review error handling
- Check session management

## Output Format

After audit, document findings:

```markdown
# Security Audit Results

## Critical Issues (fix immediately)
- [ ] SQL injection in login endpoint (line 42)
- [ ] Hardcoded API key (config.ts:15)

## High Priority
- [ ] No rate limiting on authentication
- [ ] Passwords not hashed properly

## Medium Priority
- [ ] Missing security headers
- [ ] Error messages leak info

## Low Priority
- [ ] Dependencies 2 versions behind
```

## Framework-Specific Notes

### Node.js/Express
- Use `helmet` for security headers
- Use `express-rate-limit` for rate limiting
- Use `express-validator` for input validation

### React/Frontend
- Use `DOMPurify` for HTML sanitization
- Never use `dangerouslySetInnerHTML` with user content
- Validate on backend, not just frontend

### Database
- Use ORMs with parameterized queries (Prisma, TypeORM)
- Enable SSL/TLS for database connections
- Use principle of least privilege for DB users
