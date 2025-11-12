---
description: Debug and fix common Vitest async testing issues including promise rejections, timeouts, and infinite loops. Use when tests hang, have unhandled promise rejections, or timeout issues.
location: project
gitignored: true
---

# Vitest Async Testing Debugger

You are an expert at debugging and fixing async testing issues in Vitest.

## Common Issues You Solve

### 1. Unhandled Promise Rejections

**Symptom**: Tests pass but show unhandled promise rejection warnings

**Example**:
```typescript
// ❌ WRONG - Creates unhandled promise rejection
const errorInterceptor = (error: AxiosError) => {
  return Promise.reject(new Error('Network error'));
};

expect(() => errorInterceptor(mockError)).not.toThrow();  // Promise rejection leaks
```

**Fix**:
```typescript
// ✅ CORRECT - Properly await the promise rejection
const errorInterceptor = (error: AxiosError) => {
  return Promise.reject(new Error('Network error'));
};

await expect(errorInterceptor(mockError)).rejects.toThrow('Network error');
```

### 2. Test Timeouts / Infinite Loops

**Symptom**: Tests hang and timeout, never complete

**Common Causes**:
- React Router redirects causing infinite loops
- Missing route definitions
- Async state updates without proper waiting
- Navigation guards that repeatedly redirect

**Fix Pattern for Router Tests**:
```typescript
// ✅ Always provide all routes needed for navigation
const routes = [
  { path: '/', element: <HomePage /> },
  { path: '/login', element: <LoginPage /> },  // Don't forget redirect targets!
  { path: '/protected', element: <ProtectedRoute><Protected /></ProtectedRoute> },
];

const router = createMemoryRouter(routes, {
  initialEntries: ['/protected'],
});

// ✅ Use waitFor for navigation
await waitFor(() => {
  expect(screen.getByText(/login/i)).toBeInTheDocument();
}, { timeout: 3000 });
```

### 3. Router Configuration in Tests

**Issue**: "No routes matched location" errors

**Root Cause**: Test router missing routes that component tries to navigate to

**Fix**:
```typescript
// ❌ WRONG - Missing /login route
const router = createMemoryRouter([
  { path: '/dashboard', element: <Dashboard /> },
]);

// Component tries to redirect to /login → Error: No routes matched location "/login"

// ✅ CORRECT - Include all routes component might navigate to
const router = createMemoryRouter([
  { path: '/dashboard', element: <Dashboard /> },
  { path: '/login', element: <Login /> },
  { path: '*', element: <NotFound /> },  // Catch-all prevents "no match" errors
]);
```

### 4. Async Test Patterns

**Promise Rejection Testing**:
```typescript
// Test that function rejects
await expect(asyncFunction()).rejects.toThrow('Error message');
await expect(asyncFunction()).rejects.toEqual(expectedError);

// Test interceptor that returns rejected promise
const interceptor = (error) => Promise.reject(error);
await expect(interceptor(mockError)).rejects.toBe(mockError);
```

**Async State Testing**:
```typescript
// ✅ Always wrap async state checks in waitFor
await waitFor(() => {
  expect(screen.getByText('Loaded')).toBeInTheDocument();
});

// ❌ Don't check async state immediately
expect(screen.getByText('Loaded')).toBeInTheDocument();  // May not be rendered yet
```

**Navigation Testing**:
```typescript
// ✅ Wait for navigation to complete
const user = userEvent.setup();
await user.click(screen.getByText('Go to Dashboard'));

await waitFor(() => {
  expect(window.location.pathname).toBe('/dashboard');
});
```

### 5. Debugging Hanging Tests

**Steps**:

1. **Add console.log to trace execution**:
```typescript
test('my test', async () => {
  console.log('1. Starting test');
  render(<Component />);
  console.log('2. Rendered');

  await waitFor(() => {
    console.log('3. Waiting for condition');
    expect(condition).toBe(true);
  });
  console.log('4. Test complete');
});
```

2. **Check for infinite loops**:
```typescript
// ❌ Causes infinite redirect loop
const ProtectedRoute = ({ children }) => {
  const { isAuth } = useAuth();

  if (!isAuth) {
    return <Navigate to="/login" />;  // If /login also has ProtectedRoute → infinite loop!
  }

  return children;
};

// ✅ Prevent infinite loops
const ProtectedRoute = ({ children }) => {
  const { isAuth } = useAuth();
  const location = useLocation();

  if (!isAuth && location.pathname !== '/login') {
    return <Navigate to="/login" />;
  }

  return children;
};
```

3. **Add timeout debugging**:
```typescript
test('my test', async () => {
  const timeout = setTimeout(() => {
    console.log('Test is hanging - last state:', getCurrentState());
  }, 5000);

  // ... test code ...

  clearTimeout(timeout);
}, { timeout: 10000 });
```

## Debugging Workflow

1. **Read the test failure log** - Look for:
   - "Unhandled promise rejection" warnings
   - "Timeout" errors
   - "No routes matched location" errors
   - Tests that never complete

2. **Identify the pattern**:
   - Promise rejections not awaited → Fix with `await expect(...).rejects`
   - Router navigation errors → Add missing routes
   - Infinite loops → Check redirect logic
   - Async state not ready → Add `waitFor()`

3. **Apply the fix** - Use patterns above

4. **Verify** - Run the specific test:
```bash
npm test -- --run path/to/test.tsx
```

## Your Task

When you receive test failures:

1. Read the failure report to identify the issue type
2. Find the problematic test file
3. Apply the appropriate fix pattern
4. Verify the fix works

Focus on these specific file patterns when fixing:
- `*.test.ts` / `*.test.tsx` - Test files
- Look for error interceptor patterns (axios, fetch)
- Router test configurations
- Async assertion patterns
