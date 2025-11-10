---
name: laravel-cache-configuration
description: Fix Laravel cache configuration issues including missing cache tables, driver mismatches, and test cache setup. Use when tests fail with "relation cache does not exist", cache operations fail, or RefreshDatabase doesn't set up cache properly.
---

# Laravel Cache Configuration & Testing

Fix cache-related test failures, missing cache tables, and driver configuration issues.

## Common Issue 1: Missing Cache Table in Tests

**Symptom**: `SQLSTATE[42P01]: Undefined table: 7 ERROR: relation "cache" does not exist`

**Root Cause**: Tests configured with `CACHE_STORE=array` in phpunit.xml, but code still tries to use database cache.

### Diagnosis

```bash
# Check current cache driver
php artisan tinker --execute="echo config('cache.default')"

# Check if cache table exists
php artisan tinker --execute="Schema::hasTable('cache')"

# Check phpunit.xml cache config
grep -A 2 "CACHE" phpunit.xml
```

### Fix Option 1: Create Cache Table Migration

**Best for production apps that need database cache:**

```bash
# Generate cache table migration
cd src/backend
php artisan cache:table

# This creates: database/migrations/XXXX_create_cache_table.php
```

**The generated migration looks like:**
```php
Schema::create('cache', function (Blueprint $table) {
    $table->string('key')->unique();
    $table->text('value');
    $table->integer('expiration');
});

Schema::create('cache_locks', function (Blueprint $table) {
    $table->string('key')->primary();
    $table->string('owner');
    $table->integer('expiration');
});
```

**Run migrations in test environment:**
```bash
# In phpunit.xml, ensure migrations run
# OR add to tests/bootstrap.php:
Artisan::call('migrate', ['--env' => 'testing']);
```

### Fix Option 2: Force Array Driver in Tests

**Best for unit tests that don't need persistent cache:**

**Update phpunit.xml:**
```xml
<php>
    <env name="CACHE_DRIVER" value="array"/>
    <env name="CACHE_STORE" value="array"/>
    <!-- NOT cache.default, use CACHE_DRIVER -->
</php>
```

**Update config/cache.php:**
```php
'default' => env('CACHE_DRIVER', 'file'),
```

**Clear config cache:**
```bash
php artisan config:clear
php artisan cache:clear
```

### Fix Option 3: Mock Cache in Test Setup

**Best for tests that explicitly test cache behavior:**

```php
use Illuminate\Support\Facades\Cache;

protected function setUp(): void
{
    parent::setUp();

    // Use array cache for all tests
    Cache::store('array');

    // OR completely fake it
    Cache::spy();
}

public function test_caches_data()
{
    Cache::shouldReceive('put')
        ->once()
        ->with('key', 'value', 3600);

    $this->service->cacheData('key', 'value');
}
```

## Common Issue 2: RefreshDatabase Doesn't Create Cache Table

**Symptom**: Tests pass individually but fail in suite, or cache errors occur randomly

**Root Cause**: Cache table migration exists but isn't run during test setup

### Fix: Ensure Cache Migration Runs

**Check migration exists:**
```bash
ls database/migrations/*cache*.php
```

**Ensure RefreshDatabase runs all migrations:**
```php
use Illuminate\Foundation\Testing\RefreshDatabase;

class MyTest extends TestCase
{
    use RefreshDatabase;

    // RefreshDatabase automatically runs ALL migrations
    // including cache table migration
}
```

**If cache migration is missing:**
```bash
php artisan cache:table
php artisan migrate
```

## Common Issue 3: Cache Driver Mismatch

**Symptom**: Tests fail with cache errors even with CACHE_DRIVER=array set

**Root Cause**: Code explicitly calls `Cache::store('database')` or `Cache::store('redis')`

### Fix: Make Cache Store Configurable

**Bad Code:**
```php
// ❌ Hard-coded store
Cache::store('database')->put('key', 'value');
```

**Good Code:**
```php
// ✅ Use default/configured store
Cache::put('key', 'value');

// OR make it configurable
$store = config('cache.default');
Cache::store($store)->put('key', 'value');
```

**In Tests:**
```php
protected function setUp(): void
{
    parent::setUp();

    // Override config for this test
    config(['cache.default' => 'array']);
}
```

## Common Issue 4: Cache Table Exists But Wrong Schema

**Symptom**: Cache operations fail with column errors

**Fix: Regenerate Cache Migration**
```bash
# Remove old migration
rm database/migrations/*cache*.php

# Generate fresh one
php artisan cache:table

# Migrate
php artisan migrate:fresh
```

## Cache Configuration Best Practices

### Environment-Specific Cache Drivers

**.env (production):**
```env
CACHE_DRIVER=redis
```

**.env.testing (tests):**
```env
CACHE_DRIVER=array
```

**config/cache.php:**
```php
'default' => env('CACHE_DRIVER', 'file'),

'stores' => [
    'array' => [
        'driver' => 'array',
        'serialize' => false,
    ],
    'database' => [
        'driver' => 'database',
        'table' => 'cache',
        'connection' => null,
        'lock_connection' => null,
    ],
    'redis' => [
        'driver' => 'redis',
        'connection' => 'cache',
    ],
],
```

### Testing Cache Behavior

**Test with Array Driver:**
```php
public function test_caches_expensive_computation()
{
    config(['cache.default' => 'array']);

    $result1 = $this->service->expensiveOperation();
    $result2 = $this->service->expensiveOperation();

    // Second call should be cached
    $this->assertEquals($result1, $result2);
}
```

**Test Cache Expiration:**
```php
public function test_cache_expires_after_ttl()
{
    Cache::put('key', 'value', now()->addSeconds(5));

    $this->assertEquals('value', Cache::get('key'));

    // Travel forward in time
    $this->travel(6)->seconds();

    $this->assertNull(Cache::get('key'));
}
```

**Test Cache Clearing:**
```php
public function test_flush_clears_all_cache()
{
    Cache::put('key1', 'value1');
    Cache::put('key2', 'value2');

    Cache::flush();

    $this->assertNull(Cache::get('key1'));
    $this->assertNull(Cache::get('key2'));
}
```

## Debugging Cache Issues

```php
// Check current cache driver
dump(config('cache.default'));

// Check if key exists
dump(Cache::has('my-key'));

// Get all cache keys (array driver only)
dump(Cache::getStore()->getMemory());

// Enable query log to see database cache queries
DB::enableQueryLog();
Cache::put('key', 'value');
dd(DB::getQueryLog());
```

## Quick Fix Checklist

When seeing "cache table does not exist" in tests:

1. ✅ Check phpunit.xml has `CACHE_DRIVER=array`
2. ✅ Check config/cache.php reads from `CACHE_DRIVER` env var
3. ✅ If using database cache, run `php artisan cache:table && php artisan migrate`
4. ✅ Ensure `RefreshDatabase` trait is used in tests
5. ✅ Check code doesn't hard-code cache store names
6. ✅ Clear config cache: `php artisan config:clear`
7. ✅ Re-run tests

## Complete Fix Example

**Scenario**: Tests fail with "cache does not exist" error

**Step 1: Check phpunit.xml**
```xml
<env name="CACHE_DRIVER" value="array"/>
```

**Step 2: Update config/cache.php**
```php
'default' => env('CACHE_DRIVER', 'file'),
```

**Step 3: If using database cache in production**
```bash
php artisan cache:table
php artisan migrate
```

**Step 4: Update test base class**
```php
abstract class TestCase extends BaseTestCase
{
    use RefreshDatabase;

    protected function setUp(): void
    {
        parent::setUp();

        // Ensure array cache for tests
        config(['cache.default' => 'array']);
    }
}
```

**Step 5: Clear and re-run**
```bash
php artisan config:clear
php artisan test
```
