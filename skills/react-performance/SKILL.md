---
name: react-performance
description: Optimize React application performance using useMemo, useCallback, React.memo, code splitting, and profiling. Use when experiencing slow re-renders, large component trees, expensive computations, or laggy UI interactions.
---

# React Performance Optimization Skill

Comprehensive guide to diagnosing and fixing React performance issues.

## When to Use

- Components re-rendering unnecessarily
- Slow list rendering
- Laggy user interactions
- Expensive calculations causing jank
- Large component trees
- Bundle size too large

## Diagnosis Tools

### 1. React DevTools Profiler

```bash
# Install React DevTools extension
# Chrome: https://chrome.google.com/webstore (search "React Developer Tools")

# Enable profiling in development
# Then: Click "Profiler" tab → Record → Interact with app → Stop
```

**What to look for:**
- Yellow/red components (slow renders)
- Components rendering when they shouldn't
- Long flame graph bars
- "Why did this render?" info

### 2. Performance API

```typescript
// Measure component render time
import { useEffect } from 'react'

function MyComponent() {
  useEffect(() => {
    performance.mark('MyComponent-start')
    return () => {
      performance.mark('MyComponent-end')
      performance.measure('MyComponent', 'MyComponent-start', 'MyComponent-end')
      const measure = performance.getEntriesByName('MyComponent')[0]
      console.log(`MyComponent rendered in ${measure.duration}ms`)
    }
  })

  return <div>...</div>
}
```

## Optimization Techniques

### 1. React.memo - Prevent Unnecessary Re-renders

**Use when:** Parent re-renders but child props haven't changed

```typescript
// ❌ BAD - Re-renders every time parent renders
function ExpensiveChild({ data }) {
  console.log('Rendering ExpensiveChild')
  return <div>{/* Complex rendering */}</div>
}

function Parent() {
  const [count, setCount] = useState(0)
  return (
    <div>
      <button onClick={() => setCount(count + 1)}>Count: {count}</button>
      <ExpensiveChild data={someData} /> {/* Re-renders on every count change! */}
    </div>
  )
}

// ✅ GOOD - Only re-renders when data changes
const ExpensiveChild = React.memo(function ExpensiveChild({ data }) {
  console.log('Rendering ExpensiveChild')
  return <div>{/* Complex rendering */}</div>
})

// Custom comparison function
const ExpensiveChild = React.memo(
  function ExpensiveChild({ data }) {
    return <div>{/* ... */}</div>
  },
  (prevProps, nextProps) => {
    // Return true if props are equal (skip render)
    return prevProps.data.id === nextProps.data.id
  }
)
```

### 2. useMemo - Memoize Expensive Calculations

**Use when:** Expensive computation runs on every render

```typescript
// ❌ BAD - Recalculates on EVERY render
function UserList({ users }) {
  const sortedUsers = users
    .filter(u => u.active)
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(u => ({ ...u, displayName: `${u.firstName} ${u.lastName}` }))

  return <div>{/* Render sortedUsers */}</div>
}

// ✅ GOOD - Only recalculates when users change
function UserList({ users }) {
  const sortedUsers = useMemo(() => {
    return users
      .filter(u => u.active)
      .sort((a, b) => a.name.localeCompare(b.name))
      .map(u => ({ ...u, displayName: `${u.firstName} ${u.lastName}` }))
  }, [users])

  return <div>{/* Render sortedUsers */}</div>
}
```

**When NOT to use useMemo:**
- Simple calculations (faster without memo)
- Values used only once
- Premature optimization

```typescript
// ❌ DON'T - Unnecessary overhead
const doubled = useMemo(() => count * 2, [count])

// ✅ DO - Just calculate it
const doubled = count * 2
```

### 3. useCallback - Memoize Functions

**Use when:** Passing callbacks to memoized components

```typescript
// ❌ BAD - New function on every render breaks React.memo
const MemoizedChild = React.memo(Child)

function Parent() {
  const [count, setCount] = useState(0)

  const handleClick = () => {  // ← New function every render!
    console.log('Clicked')
  }

  return (
    <div>
      <button onClick={() => setCount(count + 1)}>Count: {count}</button>
      <MemoizedChild onClick={handleClick} /> {/* Still re-renders! */}
    </div>
  )
}

// ✅ GOOD - Stable function reference
const MemoizedChild = React.memo(Child)

function Parent() {
  const [count, setCount] = useState(0)

  const handleClick = useCallback(() => {
    console.log('Clicked')
  }, [])  // Empty deps = never changes

  return (
    <div>
      <button onClick={() => setCount(count + 1)}>Count: {count}</button>
      <MemoizedChild onClick={handleClick} /> {/* Doesn't re-render! */}
    </div>
  )
}
```

**Common mistake - stale closures:**

```typescript
// ❌ BAD - count is always 0
const handleClick = useCallback(() => {
  console.log(count)  // Always logs 0!
}, [])

// ✅ GOOD - Include dependencies
const handleClick = useCallback(() => {
  console.log(count)  // Logs current count
}, [count])

// ✅ BETTER - Use functional update if you only need to update state
const handleIncrement = useCallback(() => {
  setCount(c => c + 1)  // No dependency on count!
}, [])
```

### 4. Code Splitting - Reduce Initial Bundle

```typescript
// ❌ BAD - Loads heavy components immediately
import HeavyDashboard from './HeavyDashboard'
import HeavyChart from './HeavyChart'
import HeavyEditor from './HeavyEditor'

function App() {
  return (
    <Routes>
      <Route path="/dashboard" element={<HeavyDashboard />} />
      <Route path="/chart" element={<HeavyChart />} />
      <Route path="/editor" element={<HeavyEditor />} />
    </Routes>
  )
}

// ✅ GOOD - Lazy load on demand
import { lazy, Suspense } from 'react'

const HeavyDashboard = lazy(() => import('./HeavyDashboard'))
const HeavyChart = lazy(() => import('./HeavyChart'))
const HeavyEditor = lazy(() => import('./HeavyEditor'))

function App() {
  return (
    <Suspense fallback={<Loading />}>
      <Routes>
        <Route path="/dashboard" element={<HeavyDashboard />} />
        <Route path="/chart" element={<HeavyChart />} />
        <Route path="/editor" element={<HeavyEditor />} />
      </Routes>
    </Suspense>
  )
}
```

### 5. Virtualization - Render Only Visible Items

**Use when:** Rendering long lists (>100 items)

```bash
npm install react-window
# or
npm install @tanstack/react-virtual
```

```typescript
// ❌ BAD - Renders 10,000 DOM nodes
function UserList({ users }) {
  return (
    <div>
      {users.map(user => (
        <UserCard key={user.id} user={user} />
      ))}
    </div>
  )
}

// ✅ GOOD - Only renders visible items
import { FixedSizeList } from 'react-window'

function UserList({ users }) {
  return (
    <FixedSizeList
      height={600}
      itemCount={users.length}
      itemSize={80}
      width="100%"
    >
      {({ index, style }) => (
        <div style={style}>
          <UserCard user={users[index]} />
        </div>
      )}
    </FixedSizeList>
  )
}

// ✅ BETTER - Variable height items
import { VariableSizeList } from 'react-window'

function UserList({ users }) {
  const getItemSize = (index) => {
    return users[index].bio ? 120 : 80
  }

  return (
    <VariableSizeList
      height={600}
      itemCount={users.length}
      itemSize={getItemSize}
      width="100%"
    >
      {({ index, style }) => (
        <div style={style}>
          <UserCard user={users[index]} />
        </div>
      )}
    </VariableSizeList>
  )
}
```

### 6. Debouncing Expensive Operations

```typescript
import { useMemo, useState } from 'react'
import { debounce } from 'lodash-es'

// ❌ BAD - API call on every keystroke
function SearchUsers() {
  const [query, setQuery] = useState('')

  const handleChange = (e) => {
    setQuery(e.target.value)
    fetchUsers(e.target.value)  // Called 100 times while typing!
  }

  return <input onChange={handleChange} />
}

// ✅ GOOD - Debounced search
function SearchUsers() {
  const [query, setQuery] = useState('')

  const debouncedSearch = useMemo(
    () => debounce((value) => {
      fetchUsers(value)
    }, 300),
    []
  )

  const handleChange = (e) => {
    setQuery(e.target.value)
    debouncedSearch(e.target.value)  // Only calls after 300ms pause
  }

  return <input value={query} onChange={handleChange} />
}
```

### 7. Avoid Inline Objects/Arrays in Props

```typescript
// ❌ BAD - New object every render breaks React.memo
const MemoizedChild = React.memo(Child)

function Parent() {
  return (
    <MemoizedChild
      style={{ color: 'red' }}  // ← New object every render!
      items={[1, 2, 3]}  // ← New array every render!
    />
  )
}

// ✅ GOOD - Stable references
const MemoizedChild = React.memo(Child)

const style = { color: 'red' }
const items = [1, 2, 3]

function Parent() {
  return <MemoizedChild style={style} items={items} />
}

// ✅ ALSO GOOD - useMemo for dynamic values
function Parent({ color }) {
  const style = useMemo(() => ({ color }), [color])
  return <MemoizedChild style={style} />
}
```

### 8. Key Prop Optimization

```typescript
// ❌ BAD - Index as key causes re-renders
{items.map((item, index) => (
  <Item key={index} item={item} />
))}

// ✅ GOOD - Stable unique ID
{items.map((item) => (
  <Item key={item.id} item={item} />
))}

// When no ID available, create one
const itemsWithId = useMemo(() =>
  items.map((item, index) => ({ ...item, _key: `${item.name}-${index}` }))
, [items])

{itemsWithId.map((item) => (
  <Item key={item._key} item={item} />
))}
```

## Performance Checklist

### Before Optimizing
- [ ] Measure with React DevTools Profiler
- [ ] Identify actual bottlenecks (don't guess!)
- [ ] Check if it's a real user problem (>100ms perceived delay)

### Common Quick Wins
- [ ] Wrap expensive components with React.memo
- [ ] Use useMemo for expensive calculations
- [ ] Use useCallback for functions passed to memoized components
- [ ] Add keys to list items (not index)
- [ ] Lazy load heavy routes/components
- [ ] Virtualize long lists (>100 items)
- [ ] Debounce expensive operations (search, API calls)
- [ ] Code split vendor bundles

### Advanced Optimizations
- [ ] Use React.lazy + Suspense for code splitting
- [ ] Implement virtualization for lists
- [ ] Optimize images (lazy loading, WebP, responsive)
- [ ] Use web workers for heavy computations
- [ ] Enable production builds for testing
- [ ] Analyze bundle size (webpack-bundle-analyzer)

## Common Anti-Patterns

### 1. Premature useMemo/useCallback

```typescript
// ❌ DON'T - Simple value, no benefit
const doubled = useMemo(() => count * 2, [count])

// ✅ DO - Just calculate it
const doubled = count * 2
```

### 2. Incorrect Dependencies

```typescript
// ❌ BAD - Missing dependencies
const fetchData = useCallback(() => {
  api.get(`/users/${userId}`)  // userId not in deps!
}, [])

// ✅ GOOD - Include all dependencies
const fetchData = useCallback(() => {
  api.get(`/users/${userId}`)
}, [userId])
```

### 3. Too Many State Updates

```typescript
// ❌ BAD - Multiple re-renders
setFirstName(value)
setLastName(value)
setAge(value)

// ✅ GOOD - Single state update
setState(prev => ({
  ...prev,
  firstName: value,
  lastName: value,
  age: value
}))
```

## Measuring Impact

### Before/After Comparison

```typescript
// Add performance marks
import { useEffect } from 'react'

function Component() {
  useEffect(() => {
    performance.mark('render-start')
    return () => {
      performance.mark('render-end')
      performance.measure('render', 'render-start', 'render-end')
      const measure = performance.getEntriesByName('render')[0]
      console.log(`Render took ${measure.duration}ms`)
    }
  })

  return <div>...</div>
}
```

### Bundle Size Analysis

```bash
# Vite
npm run build
npx vite-bundle-visualizer

# Or install plugin
npm install -D rollup-plugin-visualizer
```

```typescript
// vite.config.ts
import { visualizer } from 'rollup-plugin-visualizer'

export default defineConfig({
  plugins: [
    react(),
    visualizer({ open: true })
  ]
})
```

## Expected Results

Good optimization should achieve:
- **Initial render**: <1s
- **Component updates**: <16ms (60fps)
- **User interactions**: <100ms perceived
- **Bundle size**: <250KB gzipped initial
- **Lighthouse score**: >90

## Debugging Slow Performance

1. **Profile in React DevTools**
2. **Check for:**
   - Large component trees (>1000 components)
   - Expensive calculations in render
   - Large lists without virtualization
   - Missing React.memo on expensive children
   - Inline objects/arrays in props
   - Too many state updates
3. **Fix highest impact issues first**
4. **Measure again to verify improvement**
