---
name: jest-to-vitest
description: Migrate Jest tests to Vitest with proper configuration, API changes, and performance optimizations. Use when migrating from Create React App to Vite, modernizing test infrastructure, or experiencing slow Jest performance.
---

# Jest to Vitest Migration Skill

Complete guide for migrating from Jest to Vitest.

## When to Use

- Migrating from Create React App (CRA) to Vite
- Jest tests running slowly
- Want modern ESM-native testing
- Need faster CI/CD pipelines
- Switching to Vite for development

## Why Vitest?

- **10-100x faster** than Jest (ESM-native, Vite-powered)
- **Compatible** with Jest API (minimal code changes)
- **Better DX** - instant HMR for tests
- **Modern** - native ESM, top-level await
- **Smaller** - no need for babel/ts-jest transforms

## Migration Steps

### 1. Install Vitest

```bash
# Remove Jest
npm uninstall jest @types/jest jest-environment-jsdom \
  ts-jest @testing-library/jest-dom

# Install Vitest
npm install -D vitest @vitest/ui jsdom \
  @testing-library/react @testing-library/user-event \
  @testing-library/jest-dom
```

### 2. Create vitest.config.ts

```typescript
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
    css: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
        '**/mockData',
        'src/main.tsx'
      ]
    }
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  }
})
```

### 3. Create setup file

```typescript
// src/test/setup.ts
import { expect, afterEach } from 'vitest'
import { cleanup } from '@testing-library/react'
import * as matchers from '@testing-library/jest-dom/matchers'

expect.extend(matchers)

afterEach(() => {
  cleanup()
})
```

### 4. Update package.json

```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest run --coverage",
    "test:run": "vitest run"
  }
}
```

### 5. Update TypeScript config

```json
// tsconfig.json
{
  "compilerOptions": {
    "types": ["vitest/globals", "@testing-library/jest-dom"]
  }
}
```

## API Changes

### Imports

```typescript
// Before (Jest)
import { describe, it, expect } from '@jest/globals'

// After (Vitest with globals: true)
// No imports needed!
describe('test', () => {
  it('works', () => expect(true).toBe(true))
})

// After (Vitest without globals)
import { describe, it, expect } from 'vitest'
```

### Mocks

```typescript
// Before (Jest)
jest.mock('./utils')
jest.fn()
jest.spyOn(obj, 'method')

// After (Vitest) - SAME API!
vi.mock('./utils')
vi.fn()
vi.spyOn(obj, 'method')
```

**Automatic replacement:**
```bash
# Replace all jest. with vi.
find src -name "*.test.ts*" -exec sed -i 's/jest\./vi./g' {} +
find src -name "*.test.tsx*" -exec sed -i 's/jest\./vi./g' {} +
```

### Timers

```typescript
// Before (Jest)
jest.useFakeTimers()
jest.runAllTimers()
jest.advanceTimersByTime(1000)

// After (Vitest) - SAME!
vi.useFakeTimers()
vi.runAllTimers()
vi.advanceTimersByTime(1000)
```

### Module Mocks

```typescript
// Before (Jest)
jest.mock('axios', () => ({
  get: jest.fn()
}))

// After (Vitest)
vi.mock('axios', () => ({
  default: {
    get: vi.fn()
  }
}))

// Or use factory helper
vi.mock('axios', () => {
  return {
    default: {
      get: vi.fn(() => Promise.resolve({ data: {} }))
    }
  }
})
```

## Common Issues & Fixes

### Issue 1: "vi is not defined"

```typescript
// Add to test file or setup
import { vi } from 'vitest'

// OR enable globals in vitest.config.ts
export default defineConfig({
  test: {
    globals: true  // ← This allows using vi/describe/it without imports
  }
})
```

### Issue 2: jest-dom matchers not working

```typescript
// src/test/setup.ts
import { expect } from 'vitest'
import * as matchers from '@testing-library/jest-dom/matchers'

expect.extend(matchers)

// Now these work:
expect(element).toBeInTheDocument()
expect(element).toHaveTextContent('Hello')
```

### Issue 3: CSS imports failing

```typescript
// vitest.config.ts
export default defineConfig({
  test: {
    css: true  // ← Enable CSS processing
  }
})

// Or mock CSS modules
// vitest.config.ts
export default defineConfig({
  test: {
    css: {
      modules: {
        classNameStrategy: 'non-scoped'
      }
    }
  }
})
```

### Issue 4: Absolute imports not working

```typescript
// vitest.config.ts
import path from 'path'

export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components')
    }
  }
})
```

### Issue 5: __tests__ directory not found

Vitest uses different default patterns:

```typescript
// vitest.config.ts
export default defineConfig({
  test: {
    include: [
      '**/__tests__/**/*.{test,spec}.{js,ts,jsx,tsx}',
      '**/*.{test,spec}.{js,ts,jsx,tsx}'
    ]
  }
})
```

## Performance Optimization

### Parallel Execution

```typescript
// vitest.config.ts
export default defineConfig({
  test: {
    threads: true,  // Enable parallel execution
    isolate: true,  // Isolate test environment per file
    maxConcurrency: 5  // Limit concurrent tests
  }
})
```

### Watch Mode

```bash
# Vitest watch is MUCH faster than Jest
vitest --watch

# Watch only changed files
vitest --changed
```

### Coverage

```bash
# Install coverage provider
npm install -D @vitest/coverage-v8

# Run with coverage
vitest run --coverage
```

## Migration Checklist

- [ ] Install Vitest dependencies
- [ ] Create `vitest.config.ts`
- [ ] Create `src/test/setup.ts`
- [ ] Update `package.json` scripts
- [ ] Update `tsconfig.json` types
- [ ] Replace `jest.` with `vi.` in all test files
- [ ] Fix module mocks (add `default` export)
- [ ] Enable CSS processing if needed
- [ ] Configure path aliases
- [ ] Run tests: `npm test`
- [ ] Fix any remaining issues
- [ ] Remove Jest dependencies
- [ ] Delete `jest.config.js`
- [ ] Update CI/CD scripts

## Example Test (Before/After)

```typescript
// Before (Jest)
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import '@testing-library/jest-dom'
import { Button } from './Button'

describe('Button', () => {
  it('calls onClick when clicked', async () => {
    const onClick = jest.fn()
    render(<Button onClick={onClick}>Click me</Button>)

    await userEvent.click(screen.getByRole('button'))

    expect(onClick).toHaveBeenCalledTimes(1)
  })
})

// After (Vitest) - ALMOST IDENTICAL!
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
// No @testing-library/jest-dom import needed (in setup.ts)
import { Button } from './Button'

describe('Button', () => {
  it('calls onClick when clicked', async () => {
    const onClick = vi.fn()  // ← Only change: jest.fn() → vi.fn()
    render(<Button onClick={onClick}>Click me</Button>)

    await userEvent.click(screen.getByRole('button'))

    expect(onClick).toHaveBeenCalledTimes(1)
  })
})
```

## Automated Migration Script

```bash
#!/bin/bash
# migrate-to-vitest.sh

echo "🚀 Migrating from Jest to Vitest..."

# 1. Replace jest with vi in test files
echo "📝 Replacing jest. with vi. in test files..."
find src -type f \( -name "*.test.ts" -o -name "*.test.tsx" -o -name "*.spec.ts" -o -name "*.spec.tsx" \) -exec sed -i 's/jest\./vi./g' {} +

# 2. Update imports
echo "📝 Adding vitest imports..."
find src -type f \( -name "*.test.ts" -o -name "*.test.tsx" \) -exec sed -i '1s/^/import { vi } from "vitest";\n/' {} +

# 3. Remove jest dependencies
echo "📦 Removing Jest..."
npm uninstall jest @types/jest jest-environment-jsdom ts-jest @testing-library/jest-dom

# 4. Install Vitest
echo "📦 Installing Vitest..."
npm install -D vitest @vitest/ui @vitest/coverage-v8 jsdom @testing-library/jest-dom

# 5. Create config
echo "⚙️  Creating vitest.config.ts..."
cat > vitest.config.ts << 'EOF'
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts'
  }
})
EOF

# 6. Create setup file
echo "⚙️  Creating test setup..."
mkdir -p src/test
cat > src/test/setup.ts << 'EOF'
import { expect, afterEach } from 'vitest'
import { cleanup } from '@testing-library/react'
import * as matchers from '@testing-library/jest-dom/matchers'

expect.extend(matchers)
afterEach(() => cleanup())
EOF

# 7. Update package.json
echo "📝 Updating package.json scripts..."
npm pkg set scripts.test="vitest"
npm pkg set scripts.test:ui="vitest --ui"
npm pkg set scripts.test:coverage="vitest run --coverage"

# 8. Delete jest config
rm -f jest.config.js jest.config.ts

echo "✅ Migration complete! Run 'npm test' to verify."
```

## Performance Comparison

Typical improvements after migration:

- **Test execution**: 10-20x faster
- **Watch mode**: Instant (<100ms)
- **CI/CD**: 50-80% faster pipeline
- **Developer experience**: Much better with instant feedback

## Troubleshooting

If tests fail after migration:

1. Check vitest.config.ts matches your project structure
2. Verify setup.ts is being loaded
3. Check for ESM vs CommonJS issues
4. Look for async/await issues (Vitest is stricter)
5. Verify all `jest.` replaced with `vi.`
6. Check module mocks have `default` export

Run with debug:
```bash
DEBUG=vitest:* vitest
```
