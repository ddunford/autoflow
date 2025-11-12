---
name: backward-compatibility
description: Prevent breaking changes when fixing bugs or adding features. Use when fixing unit tests to ensure feature tests and existing integrations don't break. CRITICAL for avoiding regressions when modifying shared services or core functionality.
---

# Backward Compatibility & Safe Refactoring

Prevent regressions when fixing bugs by maintaining backward compatibility.

## 🚨 THE GOLDEN RULE

**If you fix unit tests but break feature tests, you've made things WORSE, not better.**

## Pre-Change Checklist

Before modifying ANY shared service, model, or core functionality:

### 1. Establish Baseline
```bash
# Run FULL test suite first
php artisan test

# Save results
# - How many tests pass?
# - Which tests fail?
# - Are feature tests passing?
```

### 2. Identify Dependencies
Ask yourself:
- What other code calls this method/class?
- Are there feature tests that use this?
- Will changing this method signature break existing code?
- Are there integration points (APIs, external services)?

### 3. Plan Backward Compatible Changes

## Safe Change Patterns

### ✅ Pattern 1: Add New Methods (Safest)

Instead of modifying existing methods, add new ones:

```php
// ❌ UNSAFE - Modifies existing method
class UserService {
    public function getUser($id) {
        // Changed to only return active users
        return User::where('id', $id)
            ->where('active', true) // ← Breaking change!
            ->first();
    }
}

// ✅ SAFE - Adds new method, keeps old one
class UserService {
    public function getUser($id) {
        // Keep existing behavior for compatibility
        return User::find($id);
    }

    public function getActiveUser($id) {
        // New method with stricter filtering
        return User::where('id', $id)
            ->where('active', true)
            ->first();
    }
}
```

### ✅ Pattern 2: Optional Parameters with Defaults

```php
// ❌ UNSAFE - Adds required parameter
public function createSession($userId, $ipAddress) {
    // Now requires IP address - breaks all existing calls!
}

// ✅ SAFE - Makes new parameter optional
public function createSession($userId, $ipAddress = null) {
    // Works with both old and new call signatures
    if ($ipAddress) {
        $this->logIp($ipAddress);
    }
}
```

### ✅ Pattern 3: Feature Flags for Behavior Changes

```php
class SessionService {
    // ❌ UNSAFE - Changes default behavior
    public function getSession($token) {
        // Now returns null for expired - breaks existing code!
        if ($this->isExpired($token)) {
            return null;
        }
    }

    // ✅ SAFE - Makes behavior configurable
    public function getSession($token, $checkExpiry = false) {
        $session = Session::where('token', $token)->first();

        if ($checkExpiry && $session && $this->isExpired($session)) {
            return null;
        }

        return $session;
    }
}
```

### ✅ Pattern 4: Deprecate Gradually

```php
class PaymentService {
    /**
     * @deprecated Use processPaymentV2() instead
     */
    public function processPayment($amount) {
        \Log::warning('processPayment() is deprecated');
        return $this->processPaymentV2($amount, []);
    }

    public function processPaymentV2($amount, $options = []) {
        // New implementation with more features
    }
}
```

## Unsafe Change Patterns (Avoid!)

### ❌ Changing Return Types

```php
// Before: Returns User|null
public function findUser($id) {
    return User::find($id);
}

// After: Returns array (BREAKING!)
public function findUser($id) {
    $user = User::find($id);
    return $user ? $user->toArray() : [];
}
```

**Why it breaks:** Code expecting `$user->name` now gets array access errors.

### ❌ Removing/Renaming Methods

```php
// Before
public function getUserData($id) { ... }

// After (BREAKING!)
public function fetchUserInformation($id) { ... }
```

**Why it breaks:** All existing calls to `getUserData()` fail.

### ❌ Adding Required Parameters

```php
// Before
public function sendEmail($to, $subject, $body) { ... }

// After (BREAKING!)
public function sendEmail($to, $subject, $body, $template) { ... }
```

**Why it breaks:** All existing calls missing 4th parameter fail.

### ❌ Changing Method Behavior Without Warning

```php
// Before: Always creates session
public function createSession($userId) {
    return Session::create(['user_id' => $userId]);
}

// After: Now enforces limits (BREAKING!)
public function createSession($userId) {
    if ($this->getSessionCount($userId) >= 3) {
        throw new TooManySessionsException(); // ← Breaks code!
    }
    return Session::create(['user_id' => $userId]);
}
```

## Incremental Testing Strategy

### Step 1: Test Before Changes
```bash
php artisan test --filter=SessionService
php artisan test --filter=OAuthCallback
php artisan test --filter=TokenRefresh
```

### Step 2: Make ONE Small Change

### Step 3: Test After Change
```bash
# Run same tests again
php artisan test --filter=SessionService  # Should still pass
php artisan test --filter=OAuthCallback    # Should still pass!
php artisan test --filter=TokenRefresh     # Should still pass!
```

### Step 4: If Feature Tests Break - STOP!

**Red flags:**
- Unit tests pass ✓
- Feature tests fail ✗

**This means:** Your "fix" broke something important!

**Action:**
1. Analyze which feature tests broke
2. Understand why they broke
3. Choose one:
   - Revert your change
   - Make change backward compatible
   - Update feature tests (only if behavior change is intentional)

## Real-World Example: Sprint 5 Regression

**What happened:**
1. Unit tests failed: SessionService missing timeout checks
2. Agent fixed: Added `->where('expires_at', '>', now())` to `getSession()`
3. Result: 4 unit tests passed ✓, but 75 feature tests broke ✗

**Why it broke:**
- OAuth flows expected to get expired sessions to show error messages
- Token refresh expected to detect expiry and trigger refresh flow
- Changing `getSession()` broke these assumptions

**Correct fix:**
```php
// Keep existing method
public function getSession($token) {
    return Session::where('token', $token)->first();
}

// Add new method for tests
public function getActiveSession($token) {
    return Session::where('token', $token)
        ->where('expires_at', '>', now())
        ->first();
}

// Update ONLY the unit tests to use new method
public function test_handles_session_timeout() {
    $session = $this->service->getActiveSession($token);
    $this->assertNull($session); // Now tests the right thing
}
```

## When Breaking Changes Are OK

Sometimes breaking changes are necessary:

1. **Security vulnerabilities** - Fix immediately, update dependents
2. **Major version releases** - Document breaking changes clearly
3. **Internal/private APIs** - Only used in one place
4. **Deprecated code removal** - After sufficient warning period

**But always:**
- Document the change
- Update all call sites
- Run full test suite
- Consider migration path

## Quick Decision Tree

```
Are you modifying an existing method?
├─ YES → Will this change behavior for existing callers?
│   ├─ YES → Can you add a new method instead?
│   │   ├─ YES → ✅ Add new method (safest)
│   │   └─ NO → Can you make it opt-in via parameter?
│   │       ├─ YES → ✅ Add optional parameter
│   │       └─ NO → ⚠️  Breaking change - need approval
│   └─ NO → ✅ Safe to modify
└─ NO → ✅ Adding new functionality (safe)
```

## Summary

**DO:**
- ✅ Run full test suite before and after changes
- ✅ Add new methods instead of modifying existing ones
- ✅ Use optional parameters with sensible defaults
- ✅ Make behavior changes opt-in
- ✅ Test incrementally

**DON'T:**
- ❌ Change method signatures without checking callers
- ❌ Change return types
- ❌ Assume only unit tests matter
- ❌ Make behavior changes in existing methods
- ❌ Fix unit tests at the expense of feature tests

**Remember:** The goal is to fix bugs WITHOUT introducing new ones!
