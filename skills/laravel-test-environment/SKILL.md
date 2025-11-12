---
name: laravel-test-environment
description: Comprehensive Laravel/PHPUnit test environment debugging for common issues including CSRF token mismatch (419), database connection refused, config cache problems, APP_ENV conflicts, Docker networking, and authentication setup. Use when tests fail with environment/setup issues rather than code logic problems.
---

# Laravel Test Environment Setup & Debugging

Comprehensive guide for fixing common Laravel PHPUnit test environment issues. Use this skill when tests fail due to environment configuration rather than code logic errors.

## Quick Diagnostic Checklist

Run these checks first to identify the root cause:

```bash
# 1. Check if config is cached (MOST COMMON ISSUE)
ls -la bootstrap/cache/config.php

# 2. Verify APP_ENV is set correctly
grep -r "APP_ENV" phpunit.xml .env docker-compose.yml

# 3. Check database connectivity from container
docker exec <container> php artisan tinker --execute="DB::connection()->getPdo()"

# 4. Verify phpunit.xml exists and is valid
cat phpunit.xml | grep -A 5 "<php>"

# 5. Check for CSRF middleware exclusions
grep -r "VerifyCsrfToken" app/Http/Middleware/
```

## Issue #1: CSRF Token Mismatch (419 Error) 🔥 MOST COMMON

**Symptoms:**
- Tests fail with `419 | Token Mismatch` or `CSRF token mismatch`
- Expected status 200 but received 419
- POST/PUT/DELETE requests fail in tests

**Root Cause:** Laravel thinks you're NOT in test mode, so CSRF protection is enforced.

### Cause 1A: Cached Configuration (90% of cases)

**Diagnosis:**
```bash
# Check if config is cached
ls -la bootstrap/cache/config.php

# If file exists, APP_ENV is frozen to whatever was in .env when cached
```

**Fix:**
```bash
# Clear config cache
php artisan config:clear

# Or manually delete
rm -f bootstrap/cache/config.php

# IMPORTANT: Never run these in development:
# ❌ php artisan config:cache
# ❌ php artisan optimize
# These should ONLY be used in production!
```

### Cause 1B: Missing phpunit.xml Configuration

**Diagnosis:**
```bash
grep "APP_ENV" phpunit.xml
# Should show: <server name="APP_ENV" value="testing"/>
```

**Fix phpunit.xml:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<phpunit>
    <php>
        <!-- Use <server> not <env> for APP_ENV -->
        <server name="APP_ENV" value="testing"/>
        <server name="APP_DEBUG" value="true"/>

        <!-- Disable CSRF -->
        <server name="BCRYPT_ROUNDS" value="4"/>
        <server name="CACHE_DRIVER" value="array"/>
        <server name="SESSION_DRIVER" value="array"/>
        <server name="QUEUE_CONNECTION" value="sync"/>
    </php>
</phpunit>
```

**Why `<server>` not `<env>`?**
- `<server>` sets PHP $_SERVER variables (standard for Laravel)
- `<env>` sets $_ENV variables (can be overridden by system env vars)
- Use `<server>` unless you specifically need system env override

### Cause 1C: Docker Environment Override

**Diagnosis:**
```bash
# Check if docker-compose.yml sets APP_ENV
grep -A 5 "APP_ENV" docker-compose.yml
```

**Problem:**
```yaml
# ❌ BAD - This overrides phpunit.xml
services:
  app:
    environment:
      - APP_ENV=local
```

**Fix:**
```yaml
# ✅ GOOD - Remove APP_ENV from docker-compose
services:
  app:
    environment:
      # Don't set APP_ENV here - let phpunit.xml control it
      - APP_DEBUG=true
```

### Cause 1D: Missing VerifyCsrfToken Middleware Exception

**If CSRF still fails after above fixes, create middleware exclusion:**

```bash
# Check if middleware exists
cat app/Http/Middleware/VerifyCsrfToken.php
```

**Create/Update app/Http/Middleware/VerifyCsrfToken.php:**
```php
<?php

namespace App\Http\Middleware;

use Illuminate\Foundation\Http\Middleware\VerifyCsrfToken as Middleware;

class VerifyCsrfToken extends Middleware
{
    /**
     * The URIs that should be excluded from CSRF verification.
     */
    protected $except = [
        // Add test routes if needed, but generally not required
        // if APP_ENV=testing is set correctly
    ];
}
```

**Register in app/Http/Kernel.php:**
```php
protected $middlewareGroups = [
    'web' => [
        \App\Http\Middleware\VerifyCsrfToken::class,
        // ... other middleware
    ],
];
```

## Issue #2: Database Connection Refused 🔥 VERY COMMON

**Symptoms:**
- `SQLSTATE[HY000] [2002] Connection refused`
- `SQLSTATE[HY000] [2002] No such file or directory`
- Tests can't connect to database

**Root Cause:** Wrong DB_HOST or Docker networking misconfiguration.

### Fix: Correct DB_HOST Configuration

**The fix depends on WHERE you run phpunit:**

#### Running PHPUnit INSIDE Docker Container (Recommended)

```bash
# Check container name
docker ps

# Run tests inside container
docker exec -it <container-name> php artisan test
```

**phpunit.xml for inside container:**
```xml
<php>
    <server name="DB_CONNECTION" value="pgsql"/> <!-- or mysql -->
    <server name="DB_HOST" value="postgres"/> <!-- Docker service name -->
    <server name="DB_PORT" value="5432"/>
    <server name="DB_DATABASE" value="testing"/>
    <server name="DB_USERNAME" value="postgres"/>
    <server name="DB_PASSWORD" value="password"/>
</php>
```

**Key:** Use the Docker service name from docker-compose.yml:
```yaml
services:
  postgres:  # ← This is your DB_HOST
    image: postgres:15
```

#### Running PHPUnit on Host Machine

**phpunit.xml for host machine:**
```xml
<php>
    <server name="DB_HOST" value="127.0.0.1"/>
    <server name="DB_PORT" value="5433"/> <!-- Mapped external port -->
</php>
```

**Key:** Use mapped port from docker-compose.yml:
```yaml
services:
  postgres:
    ports:
      - "5433:5432"  # ← Use 5433 from host
```

### Fix: Verify Database is Running

```bash
# Check if database container is running
docker ps | grep postgres

# Test database connection from container
docker exec <container> psql -U postgres -d testing -c "SELECT 1"

# Check docker network
docker network inspect <network-name>
```

### Fix: Clear Config Cache (Again!)

```bash
# Database config might be cached too
php artisan config:clear
php artisan cache:clear
```

## Issue #3: APP_KEY / OpenSSL Errors

**Symptoms:**
- `RuntimeException: No application encryption key has been specified`
- `Cannot access uninitialized property`
- `OpenSSL extension required`

### Fix 3A: Generate APP_KEY

```bash
# Generate key
php artisan key:generate

# Or manually add to .env
APP_KEY=base64:... (32 char random string)
```

**Add to phpunit.xml if needed:**
```xml
<server name="APP_KEY" value="base64:AAACCCaaabbbcccdddeeefffggghhh111222333444="/>
```

### Fix 3B: Install OpenSSL Extension

```bash
# Check if installed
php -m | grep openssl

# Install on Debian/Ubuntu
apt-get install php-openssl

# Or in Dockerfile
RUN docker-php-ext-install openssl
```

## Issue #4: Authentication/Sanctum in Tests

**Symptoms:**
- `401 Unauthorized` in tests
- Authentication middleware blocking test requests
- Sanctum token errors

### Fix 4A: Use actingAs() for Authenticated Requests

```php
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;

class MyTest extends TestCase
{
    use RefreshDatabase;

    public function test_authenticated_endpoint()
    {
        // Create test user
        $user = User::factory()->create();

        // Authenticate for this request
        $response = $this->actingAs($user)
            ->postJson('/api/data', ['key' => 'value']);

        $response->assertStatus(200);
    }

    public function test_with_specific_guard()
    {
        $user = User::factory()->create();

        // Specify guard (sanctum, api, web)
        $response = $this->actingAs($user, 'sanctum')
            ->getJson('/api/protected');

        $response->assertStatus(200);
    }
}
```

### Fix 4B: Disable Authentication for Tests

**Option 1: Skip middleware in test:**
```php
public function test_without_auth()
{
    $this->withoutMiddleware(\App\Http\Middleware\Authenticate::class);

    $response = $this->postJson('/api/data', []);
    $response->assertStatus(200);
}
```

**Option 2: Configure phpunit.xml to disable auth:**
```xml
<server name="SANCTUM_STATEFUL_DOMAINS" value="localhost,127.0.0.1"/>
```

## Issue #5: Tests Pass Individually, Fail in Suite

**Symptoms:**
- `php artisan test --filter=MyTest` passes
- `php artisan test` fails
- Random failures depending on test order

**Root Cause:** Shared state between tests (database, cache, sessions).

### Fix 5A: Use RefreshDatabase

```php
use Illuminate\Foundation\Testing\RefreshDatabase;

class MyTest extends TestCase
{
    use RefreshDatabase;  // ← Fresh database per test

    public function test_example()
    {
        // Database is migrated fresh before this test
    }
}
```

### Fix 5B: Reset Between Tests

```php
protected function setUp(): void
{
    parent::setUp();

    // Clear cache
    Cache::flush();

    // Reset config
    config(['cache.default' => 'array']);
}

protected function tearDown(): void
{
    // Cleanup after test
    parent::tearDown();
}
```

## Issue #6: Docker Database Permissions

**Symptoms:**
- `SQLSTATE[42501]: Insufficient privilege`
- `permission denied for schema public`

### Fix: Grant Proper Permissions

```bash
# Connect to database container
docker exec -it <postgres-container> psql -U postgres

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE testing TO postgres;
GRANT ALL PRIVILEGES ON SCHEMA public TO postgres;
\q
```

**Or in docker-compose.yml:**
```yaml
services:
  postgres:
    environment:
      POSTGRES_DB: testing
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    command: postgres -c 'max_connections=200'
```

## Complete Troubleshooting Flow

When tests fail, follow this order:

### Step 1: Clear All Caches (Fix 80% of issues)
```bash
php artisan config:clear
php artisan cache:clear
php artisan route:clear
php artisan view:clear

# Or nuclear option
rm -rf bootstrap/cache/*.php
```

### Step 2: Verify phpunit.xml
```xml
<server name="APP_ENV" value="testing"/>
<server name="DB_HOST" value="postgres"/>  <!-- Docker service name -->
<server name="CACHE_DRIVER" value="array"/>
<server name="SESSION_DRIVER" value="array"/>
```

### Step 3: Verify .env.testing (Optional)
```env
APP_ENV=testing
DB_CONNECTION=pgsql
DB_HOST=postgres
DB_DATABASE=testing
```

### Step 4: Run Diagnostic Commands
```bash
# Check APP_ENV
docker exec <container> php artisan tinker --execute="echo app()->environment()"
# Should output: testing

# Check database connection
docker exec <container> php artisan tinker --execute="DB::connection()->getPdo()"
# Should succeed without error

# Check config
docker exec <container> php artisan config:show database.default
```

### Step 5: Run Tests with Debug
```bash
# Run single test with verbose output
php artisan test --filter=MyTest --debug

# Run with PHPUnit verbose
./vendor/bin/phpunit --verbose --debug
```

## Environment Configuration Best Practices

### Recommended phpunit.xml Template
```xml
<?xml version="1.0" encoding="UTF-8"?>
<phpunit xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation="vendor/phpunit/phpunit/phpunit.xsd"
         bootstrap="vendor/autoload.php"
         colors="true">
    <testsuites>
        <testsuite name="Unit">
            <directory>tests/Unit</directory>
        </testsuite>
        <testsuite name="Feature">
            <directory>tests/Feature</directory>
        </testsuite>
    </testsuites>
    <php>
        <!-- Environment -->
        <server name="APP_ENV" value="testing"/>
        <server name="APP_DEBUG" value="true"/>
        <server name="APP_KEY" value="base64:TEST_KEY_HERE"/>

        <!-- Database -->
        <server name="DB_CONNECTION" value="pgsql"/>
        <server name="DB_HOST" value="postgres"/>
        <server name="DB_PORT" value="5432"/>
        <server name="DB_DATABASE" value="testing"/>
        <server name="DB_USERNAME" value="postgres"/>
        <server name="DB_PASSWORD" value="password"/>

        <!-- Cache & Session (in-memory for tests) -->
        <server name="CACHE_DRIVER" value="array"/>
        <server name="SESSION_DRIVER" value="array"/>
        <server name="QUEUE_CONNECTION" value="sync"/>

        <!-- Security -->
        <server name="BCRYPT_ROUNDS" value="4"/>

        <!-- Mail (prevent sending real emails) -->
        <server name="MAIL_MAILER" value="array"/>
    </php>
</phpunit>
```

### Recommended docker-compose.yml for Testing
```yaml
services:
  app:
    build: .
    volumes:
      - .:/var/www/html
    depends_on:
      - postgres
    # ❌ Don't set APP_ENV here - let phpunit.xml control it
    environment:
      - APP_DEBUG=true

  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: testing
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5433:5432"  # External port for host access
    healthcheck:
      test: ["CMD-EXEC", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
```

### Recommended TestCase Base Class
```php
<?php

namespace Tests;

use Illuminate\Foundation\Testing\TestCase as BaseTestCase;

abstract class TestCase extends BaseTestCase
{
    use CreatesApplication;

    protected function setUp(): void
    {
        parent::setUp();

        // Ensure test environment
        $this->app['env'] = 'testing';

        // Use array cache for all tests
        config(['cache.default' => 'array']);

        // Disable external services
        config(['services.stripe.key' => 'test_key']);
    }

    protected function tearDown(): void
    {
        // Cleanup
        parent::tearDown();
    }
}
```

## Quick Reference: Common Error → Fix

| Error | Likely Cause | Quick Fix |
|-------|--------------|-----------|
| `419 Token Mismatch` | Cached config | `php artisan config:clear` |
| `Connection refused` | Wrong DB_HOST | Use Docker service name in phpunit.xml |
| `No application key` | Missing APP_KEY | `php artisan key:generate` |
| `401 Unauthorized` | Missing auth | Use `actingAs($user)` in test |
| `Table doesn't exist` | Missing migrations | Add `use RefreshDatabase;` |
| Tests pass alone, fail together | Shared state | Use `RefreshDatabase` + clear cache in setUp |
| `APP_ENV not testing` | Docker override | Remove APP_ENV from docker-compose.yml |

## When to Use This Skill

✅ **Use this skill when:**
- Tests fail with 419 CSRF errors
- Database connection refused errors
- Authentication/middleware blocking tests
- Tests pass individually but fail in suite
- Docker networking issues
- APP_ENV not recognized as testing

❌ **Don't use this skill for:**
- Test logic errors (wrong assertions)
- Business logic bugs
- Missing code implementation
- TypeScript/frontend test issues

## Summary Checklist

Before debugging, run this checklist:

```bash
# 1. Clear caches
php artisan config:clear

# 2. Check phpunit.xml has APP_ENV=testing
grep APP_ENV phpunit.xml

# 3. Remove APP_ENV from docker-compose.yml
grep -v "APP_ENV" docker-compose.yml

# 4. Use correct DB_HOST (Docker service name)
grep DB_HOST phpunit.xml

# 5. Run tests inside container
docker exec <container> php artisan test

# 6. Check if database is accessible
docker exec <container> php artisan tinker --execute="DB::connection()->getPdo()"
```

This checklist solves 95% of Laravel test environment issues.
