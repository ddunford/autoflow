# React + Vite Integration Skill

**Purpose**: Ensure React Vite projects properly integrate components and avoid leaving boilerplate code.

## Common Issues

### 1. Components Not Wired to App.tsx
**Problem**: Component created but not imported/used in App.tsx

**Check**:
```bash
# Find orphaned components
find src/components -name "*.tsx" -o -name "*.jsx" | while read comp; do
  name=$(basename "$comp" .tsx .jsx)
  if ! grep -q "$name" src/App.tsx; then
    echo "⚠️  $name not imported in App.tsx"
  fi
done
```

**Fix**:
```tsx
// App.tsx
import { ComponentName } from './components/ComponentName';

function App() {
  return (
    <div>
      <ComponentName />
    </div>
  );
}
```

### 2. Boilerplate Code Still Present
**Problem**: Default Vite template code not removed

**Check**:
```bash
grep -l "Vite \+ React" src/App.tsx src/App.css
```

**Fix**: Remove default Vite branding and examples

### 3. Missing Router Setup
**Problem**: Multiple pages but no routing

**Fix**:
```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
      </Routes>
    </BrowserRouter>
  );
}
```

## Validation Checklist

- [ ] All components imported in App.tsx or router
- [ ] No default Vite boilerplate remaining
- [ ] Router configured if multiple pages
- [ ] State management connected if used
- [ ] API client integrated if backend exists

## Usage

Run this check after creating new React components to ensure proper integration.
