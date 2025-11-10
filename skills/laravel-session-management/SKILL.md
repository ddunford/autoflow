---
name: laravel-session-management
description: Fix session management logic including concurrent session limits, TTL/expiration, cleanup of expired sessions, and Redis connection handling. Use when session tests fail with limit enforcement, timeout checks, or cleanup issues.
---

# Laravel Session Management Patterns

Fix session limit enforcement, TTL/expiration logic, cleanup mechanisms, and error handling.

## ⚠️ CRITICAL WARNING: Avoid Breaking Existing Flows

**BEFORE making ANY changes to SessionService:**

1. **Run ALL tests first** - not just the failing unit tests:
   ```bash
   # Run full test suite to establish baseline
   php artisan test
   ```

2. **Check for dependent feature tests**:
   - OAuth callback flows (OAuthCallbackTest)
   - Token refresh flows (TokenRefreshTest)
   - Authentication endpoints
   - Any code that calls SessionService methods

3. **Make changes BACKWARD COMPATIBLE**:
   - Add optional parameters with defaults instead of required parameters
   - Keep existing method signatures working
   - Add new methods instead of modifying existing ones when possible

4. **Test incrementally**:
   - Fix one issue at a time
   - Run full test suite after EACH change
   - If feature tests break, STOP and analyze why

**Example of Safe vs Unsafe Changes:**

```php
// ❌ UNSAFE - Changes return type/behavior
public function getSession($token) {
    // Now returns null for expired sessions
    // This BREAKS code expecting expired session objects
    if ($session->expires_at < now()) {
        return null; // ← Breaking change!
    }
}

// ✅ SAFE - Adds new method, keeps old one
public function getSession($token) {
    // Keep existing behavior
    return Session::where('token', $token)->first();
}

public function getActiveSession($token) {
    // New method with stricter validation
    return Session::where('token', $token)
        ->where('expires_at', '>', now())
        ->first();
}
```

**If unit tests pass but feature tests fail:**
- You likely broke backward compatibility
- Consider reverting and using a different approach
- Or update feature tests to match new behavior

## Common Issue 1: Concurrent Session Limit Not Enforced

**Symptom**: Test expects max N sessions, but more are created

**Example Test Failure:**
```php
// test_limits_concurrent_sessions_for_same_user
Failed asserting that 5 is equal to 3 or is less than 3.
```

### Root Cause

Session creation doesn't check existing session count before creating new ones.

### Fix Pattern

**Bad Implementation:**
```php
public function createSession($userId, $token)
{
    // ❌ No limit checking
    return Session::create([
        'user_id' => $userId,
        'token' => $token,
        'expires_at' => now()->addHours(2),
    ]);
}
```

**Good Implementation:**
```php
public function createSession($userId, $token, $maxSessions = 3)
{
    // Get active sessions for user
    $activeSessions = Session::where('user_id', $userId)
        ->where('expires_at', '>', now())
        ->orderBy('created_at', 'desc')
        ->get();

    // If at or over limit, remove oldest sessions
    if ($activeSessions->count() >= $maxSessions) {
        $sessionsToRemove = $activeSessions->count() - $maxSessions + 1;

        $activeSessions->slice($maxSessions - 1)
            ->each(fn($session) => $session->delete());
    }

    // Create new session
    return Session::create([
        'user_id' => $userId,
        'token' => $token,
        'expires_at' => now()->addHours(2),
    ]);
}
```

**Alternative: Use Database Transaction**
```php
public function createSession($userId, $token, $maxSessions = 3)
{
    return DB::transaction(function () use ($userId, $token, $maxSessions) {
        // Lock user's sessions to prevent race conditions
        $count = Session::where('user_id', $userId)
            ->where('expires_at', '>', now())
            ->lockForUpdate()
            ->count();

        if ($count >= $maxSessions) {
            // Delete oldest session(s)
            Session::where('user_id', $userId)
                ->where('expires_at', '>', now())
                ->orderBy('created_at', 'asc')
                ->limit($count - $maxSessions + 1)
                ->delete();
        }

        return Session::create([
            'user_id' => $userId,
            'token' => $token,
            'expires_at' => now()->addHours(2),
        ]);
    });
}
```

## Common Issue 2: Session Timeout/TTL Not Enforced

**Symptom**: Session data still accessible after expiration

**Example Test Failure:**
```php
// test_handles_session_timeout_after_inactivity
Failed asserting that session data is null after timeout.
Actual: Session data still returned
```

### Root Cause

`getSession()` doesn't check if session is expired before returning it.

### Fix Pattern

**Bad Implementation:**
```php
public function getSession($token)
{
    // ❌ Returns expired sessions
    return Session::where('token', $token)->first();
}
```

**Good Implementation:**
```php
public function getSession($token)
{
    // ✅ Check expiration
    $session = Session::where('token', $token)
        ->where('expires_at', '>', now())
        ->first();

    return $session;
}
```

**With Automatic Cleanup:**
```php
public function getSession($token)
{
    $session = Session::where('token', $token)->first();

    if (!$session) {
        return null;
    }

    // Check if expired
    if ($session->expires_at < now()) {
        $session->delete();
        return null;
    }

    // Update last_activity timestamp
    $session->update(['last_activity' => now()]);

    return $session;
}
```

**With Query Scope:**
```php
// In Session model
public function scopeActive($query)
{
    return $query->where('expires_at', '>', now());
}

// In service
public function getSession($token)
{
    return Session::active()
        ->where('token', $token)
        ->first();
}
```

## Common Issue 3: Cleanup Method Not Working

**Symptom**: `cleanupExpiredSessions()` returns 0 instead of removing expired sessions

**Example Test Failure:**
```php
// test_cleans_up_expired_sessions
Failed asserting that 0 is equal to 2 or is greater than 2.
```

### Root Cause

Cleanup logic has incorrect date comparison or doesn't return count.

### Fix Pattern

**Bad Implementation:**
```php
public function cleanupExpiredSessions()
{
    // ❌ Wrong comparison or no return value
    Session::where('expires_at', '<', now())->delete();
    return 0; // Wrong!
}
```

**Good Implementation:**
```php
public function cleanupExpiredSessions(): int
{
    // ✅ Correct comparison and returns affected count
    $count = Session::where('expires_at', '<', now())->count();

    Session::where('expires_at', '<', now())->delete();

    return $count;
}
```

**Alternative: Use deleteAndReturn**
```php
public function cleanupExpiredSessions(): int
{
    $sessions = Session::where('expires_at', '<', now())->get();

    $count = $sessions->count();

    $sessions->each->delete();

    return $count;
}
```

**With Logging:**
```php
public function cleanupExpiredSessions(): int
{
    $expiredSessions = Session::where('expires_at', '<', now())->get();

    $count = $expiredSessions->count();

    if ($count > 0) {
        \Log::info("Cleaning up {$count} expired sessions");

        $expiredSessions->each(function ($session) {
            \Log::debug("Removing expired session", [
                'user_id' => $session->user_id,
                'expired_at' => $session->expires_at,
            ]);
            $session->delete();
        });
    }

    return $count;
}
```

## Common Issue 4: Wrong Error Messages

**Symptom**: Exception thrown with unexpected message

**Example Test Failure:**
```php
// test_handles_redis_connection_failures
Failed asserting that exception message 'Redis connection failed' contains 'Session storage failed'.
```

### Fix Pattern

**Bad Implementation:**
```php
try {
    Redis::set($key, $value);
} catch (\Exception $e) {
    // ❌ Exposes internal error details
    throw new \Exception('Redis connection failed');
}
```

**Good Implementation:**
```php
try {
    Redis::set($key, $value);
} catch (\Exception $e) {
    // ✅ Generic error message, log details internally
    \Log::error('Redis connection failed', [
        'error' => $e->getMessage(),
        'key' => $key,
    ]);

    throw new SessionStorageException('Session storage failed', 0, $e);
}
```

**Custom Exception:**
```php
// app/Exceptions/SessionStorageException.php
class SessionStorageException extends \Exception
{
    public function __construct($message = 'Session storage failed', $code = 0, \Throwable $previous = null)
    {
        parent::__construct($message, $code, $previous);
    }
}

// In service
try {
    $this->storage->save($session);
} catch (RedisException $e) {
    throw new SessionStorageException('Session storage failed', 0, $e);
} catch (DatabaseException $e) {
    throw new SessionStorageException('Session storage failed', 0, $e);
}
```

## Complete Session Service Example

```php
namespace App\Services;

use App\Models\Session;
use App\Exceptions\SessionStorageException;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

class SessionService
{
    private int $maxConcurrentSessions = 3;
    private int $sessionTtlHours = 2;

    /**
     * Create a new session, enforcing concurrent session limit
     */
    public function createSession(string $userId, string $token): Session
    {
        return DB::transaction(function () use ($userId, $token) {
            // Get count of active sessions
            $activeCount = Session::where('user_id', $userId)
                ->where('expires_at', '>', now())
                ->lockForUpdate()
                ->count();

            // Remove oldest sessions if at limit
            if ($activeCount >= $this->maxConcurrentSessions) {
                $toRemove = $activeCount - $this->maxConcurrentSessions + 1;

                Session::where('user_id', $userId)
                    ->where('expires_at', '>', now())
                    ->orderBy('created_at', 'asc')
                    ->limit($toRemove)
                    ->delete();
            }

            // Create new session
            return Session::create([
                'id' => Str::uuid(),
                'user_id' => $userId,
                'token' => $token,
                'expires_at' => now()->addHours($this->sessionTtlHours),
                'last_activity' => now(),
            ]);
        });
    }

    /**
     * Get session, checking expiration
     */
    public function getSession(string $token): ?Session
    {
        $session = Session::where('token', $token)->first();

        if (!$session) {
            return null;
        }

        // Check if expired
        if ($session->expires_at < now()) {
            $session->delete();
            return null;
        }

        // Update activity timestamp
        $session->touch('last_activity');

        return $session;
    }

    /**
     * Validate session is active
     */
    public function validateSession(string $token): bool
    {
        return Session::where('token', $token)
            ->where('expires_at', '>', now())
            ->exists();
    }

    /**
     * Cleanup expired sessions
     */
    public function cleanupExpiredSessions(): int
    {
        $count = Session::where('expires_at', '<', now())->count();

        if ($count > 0) {
            Session::where('expires_at', '<', now())->delete();
        }

        return $count;
    }

    /**
     * Revoke specific session
     */
    public function revokeSession(string $token): bool
    {
        return Session::where('token', $token)->delete() > 0;
    }

    /**
     * Revoke all user sessions except current
     */
    public function revokeOtherSessions(string $userId, string $currentToken): int
    {
        return Session::where('user_id', $userId)
            ->where('token', '!=', $currentToken)
            ->delete();
    }

    /**
     * Handle storage failures gracefully
     */
    private function handleStorageFailure(\Exception $e): void
    {
        \Log::error('Session storage failed', [
            'error' => $e->getMessage(),
            'trace' => $e->getTraceAsString(),
        ]);

        throw new SessionStorageException('Session storage failed', 0, $e);
    }
}
```

## Testing Session Management

```php
use Illuminate\Foundation\Testing\RefreshDatabase;

class SessionServiceTest extends TestCase
{
    use RefreshDatabase;

    public function test_limits_concurrent_sessions_for_same_user()
    {
        $userId = 'user-123';
        $service = new SessionService();

        // Create 5 sessions
        for ($i = 0; $i < 5; $i++) {
            $service->createSession($userId, "token-{$i}");
        }

        // Should only have 3 active sessions (the limit)
        $activeCount = Session::where('user_id', $userId)
            ->where('expires_at', '>', now())
            ->count();

        $this->assertLessThanOrEqual(3, $activeCount);
    }

    public function test_handles_session_timeout_after_inactivity()
    {
        $service = new SessionService();
        $session = $service->createSession('user-123', 'token-abc');

        // Session should be valid now
        $this->assertNotNull($service->getSession('token-abc'));

        // Travel past expiration
        $this->travel(3)->hours();

        // Session should be expired
        $this->assertNull($service->getSession('token-abc'));
    }

    public function test_cleans_up_expired_sessions()
    {
        $service = new SessionService();

        // Create sessions with past expiration
        Session::factory()->count(5)->create([
            'expires_at' => now()->subHour(),
        ]);

        $count = $service->cleanupExpiredSessions();

        $this->assertGreaterThanOrEqual(5, $count);
        $this->assertEquals(0, Session::where('expires_at', '<', now())->count());
    }

    public function test_handles_redis_connection_failures()
    {
        $this->expectException(SessionStorageException::class);
        $this->expectExceptionMessage('Session storage failed');

        // Mock Redis failure
        Redis::shouldReceive('set')->andThrow(new RedisException('Connection refused'));

        $service = new SessionService();
        $service->createSession('user-123', 'token-abc');
    }
}
```

## Quick Fix Checklist

When session tests are failing:

1. ✅ **Concurrent limit**: Check if `createSession()` enforces max sessions
2. ✅ **TTL enforcement**: Check if `getSession()` validates `expires_at > now()`
3. ✅ **Cleanup**: Ensure `cleanupExpiredSessions()` returns deleted count
4. ✅ **Error messages**: Verify exception messages match test expectations
5. ✅ **Transactions**: Use DB transactions for session limit enforcement
6. ✅ **Time travel**: Use `$this->travel()` in tests to test expiration
7. ✅ **Factories**: Ensure Session factory creates valid test data
