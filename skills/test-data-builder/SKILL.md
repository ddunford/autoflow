---
name: test-data-builder
description: Generate realistic test data using factories, builders, and fixtures. Use when writing tests that need complex objects, avoiding duplicate test data setup, or implementing the Builder pattern for test objects.
---

# Test Data Builder Skill

Create maintainable, realistic test data using factories and builders.

## When to Use

- Writing tests that need complex objects
- Multiple tests need similar data
- Reducing duplication in test setup
- Need realistic but random data
- Testing edge cases systematically

## Patterns

### 1. Factory Functions (Simple)

```typescript
// tests/factories/user.ts
export function createUser(overrides = {}) {
  return {
    id: Math.random().toString(36),
    email: `test-${Date.now()}@example.com`,
    name: 'Test User',
    age: 25,
    role: 'user',
    createdAt: new Date(),
    ...overrides  // Override any fields
  }
}

// Usage in tests
import { createUser } from './factories/user'

test('can update user email', () => {
  const user = createUser({ email: 'specific@example.com' })
  expect(user.email).toBe('specific@example.com')
})

test('admin can delete users', () => {
  const admin = createUser({ role: 'admin' })
  const user = createUser()

  expect(admin.canDelete(user)).toBe(true)
})
```

### 2. Builder Pattern (Complex)

```typescript
// tests/builders/UserBuilder.ts
export class UserBuilder {
  private data = {
    id: Math.random().toString(36),
    email: `test-${Date.now()}@example.com`,
    name: 'Test User',
    age: 25,
    role: 'user' as 'user' | 'admin',
    posts: [] as Post[],
    createdAt: new Date()
  }

  withEmail(email: string) {
    this.data.email = email
    return this
  }

  withName(name: string) {
    this.data.name = name
    return this
  }

  asAdmin() {
    this.data.role = 'admin'
    return this
  }

  withPosts(count: number) {
    this.data.posts = Array.from({ length: count }, (_, i) =>
      new PostBuilder().withTitle(`Post ${i}`).build()
    )
    return this
  }

  build() {
    return { ...this.data }
  }
}

// Usage
import { UserBuilder } from './builders/UserBuilder'

test('admin with posts can publish', () => {
  const user = new UserBuilder()
    .withEmail('admin@example.com')
    .asAdmin()
    .withPosts(5)
    .build()

  expect(user.role).toBe('admin')
  expect(user.posts).toHaveLength(5)
})
```

### 3. Faker.js (Realistic Data)

```bash
npm install -D @faker-js/faker
```

```typescript
// tests/factories/user.ts
import { faker } from '@faker-js/faker'

export function createUser(overrides = {}) {
  return {
    id: faker.string.uuid(),
    email: faker.internet.email(),
    name: faker.person.fullName(),
    age: faker.number.int({ min: 18, max: 80 }),
    avatar: faker.image.avatar(),
    bio: faker.lorem.paragraph(),
    address: {
      street: faker.location.streetAddress(),
      city: faker.location.city(),
      country: faker.location.country()
    },
    createdAt: faker.date.past(),
    ...overrides
  }
}

// Generate multiple users
export function createUsers(count: number, overrides = {}) {
  return Array.from({ length: count }, () => createUser(overrides))
}

// Usage
test('can search users by name', () => {
  const users = createUsers(10)
  const result = searchUsers(users, users[0].name)
  expect(result).toContain(users[0])
})
```

### 4. Fixtures (Static Data)

```typescript
// tests/fixtures/users.json
[
  {
    "id": "1",
    "email": "john@example.com",
    "name": "John Doe",
    "role": "admin"
  },
  {
    "id": "2",
    "email": "jane@example.com",
    "name": "Jane Smith",
    "role": "user"
  }
]

// tests/fixtures/index.ts
import users from './users.json'

export function getFixture(name: string) {
  const fixtures = { users }
  return fixtures[name]
}

// Usage
import { getFixture } from './fixtures'

test('loads users from fixture', () => {
  const users = getFixture('users')
  expect(users).toHaveLength(2)
  expect(users[0].email).toBe('john@example.com')
})
```

### 5. Database Seeding (E2E Tests)

```typescript
// tests/seed.ts
import { PrismaClient } from '@prisma/client'
import { createUser } from './factories/user'

const prisma = new PrismaClient()

export async function seedDatabase() {
  // Clear database
  await prisma.post.deleteMany()
  await prisma.user.deleteMany()

  // Create test users
  const admin = await prisma.user.create({
    data: createUser({ email: 'admin@example.com', role: 'admin' })
  })

  const user = await prisma.user.create({
    data: createUser({ email: 'user@example.com', role: 'user' })
  })

  // Create posts
  await prisma.post.createMany({
    data: [
      { title: 'Post 1', authorId: admin.id },
      { title: 'Post 2', authorId: user.id }
    ]
  })

  return { admin, user }
}

// Usage in E2E tests
import { test } from '@playwright/test'
import { seedDatabase } from './seed'

test.beforeEach(async () => {
  await seedDatabase()
})

test('admin can see all posts', async ({ page }) => {
  await page.goto('/admin/posts')
  await expect(page.locator('.post')).toHaveCount(2)
})
```

### 6. Traits/Modifiers

```typescript
// tests/factories/user.ts
export function createUser(traits = [], overrides = {}) {
  let data = {
    id: faker.string.uuid(),
    email: faker.internet.email(),
    name: faker.person.fullName(),
    role: 'user',
    verified: false,
    banned: false,
    posts: []
  }

  // Apply traits
  traits.forEach(trait => {
    if (trait === 'admin') {
      data.role = 'admin'
    }
    if (trait === 'verified') {
      data.verified = true
    }
    if (trait === 'banned') {
      data.banned = true
    }
    if (trait === 'with_posts') {
      data.posts = Array.from({ length: 5 }, () => createPost())
    }
  })

  return { ...data, ...overrides }
}

// Usage
const admin = createUser(['admin', 'verified'])
const bannedUser = createUser(['banned'])
const userWithPosts = createUser(['with_posts'])
```

### 7. Sequence Numbers

```typescript
// tests/factories/user.ts
let userSequence = 0

export function createUser(overrides = {}) {
  const seq = ++userSequence

  return {
    id: `user-${seq}`,
    email: `user${seq}@example.com`,
    name: `Test User ${seq}`,
    ...overrides
  }
}

export function resetSequence() {
  userSequence = 0
}

// Usage
beforeEach(() => {
  resetSequence()
})

test('creates users with unique emails', () => {
  const user1 = createUser()
  const user2 = createUser()

  expect(user1.email).toBe('user1@example.com')
  expect(user2.email).toBe('user2@example.com')
})
```

## Libraries

### fishery (Recommended for TypeScript)

```bash
npm install -D fishery
```

```typescript
// tests/factories/user.factory.ts
import { Factory } from 'fishery'
import { User } from '../types'
import { faker } from '@faker-js/faker'

export const userFactory = Factory.define<User>(({ sequence }) => ({
  id: `user-${sequence}`,
  email: faker.internet.email(),
  name: faker.person.fullName(),
  role: 'user',
  createdAt: new Date()
}))

// Usage
import { userFactory } from './factories/user.factory'

const user = userFactory.build()
const admin = userFactory.build({ role: 'admin' })
const users = userFactory.buildList(10)

// With database
const persistedUser = await userFactory.create()  // Saves to DB
const persistedUsers = await userFactory.createList(10)
```

### factory-bot (Ruby-style for JS)

```bash
npm install -D factory-bot
```

```typescript
// tests/factories/index.ts
import { factory } from 'factory-bot'
import { faker } from '@faker-js/faker'

factory.define('user', () => ({
  id: faker.string.uuid(),
  email: faker.internet.email(),
  name: faker.person.fullName(),
  role: 'user'
}))

factory.extend('user', 'admin', () => ({
  role: 'admin'
}))

// Usage
const user = await factory.build('user')
const admin = await factory.build('admin')
const users = await factory.buildMany('user', 10)
```

## Best Practices

### 1. One Factory Per Model

```
tests/
├── factories/
│   ├── user.ts
│   ├── post.ts
│   ├── comment.ts
│   └── index.ts    # Export all
```

### 2. Sensible Defaults

```typescript
// ✅ GOOD - Realistic defaults
export function createUser(overrides = {}) {
  return {
    id: faker.string.uuid(),
    email: faker.internet.email(),
    name: faker.person.fullName(),
    role: 'user',  // Most common case
    verified: true,  // Typical state
    ...overrides
  }
}

// ❌ BAD - Unrealistic defaults
export function createUser(overrides = {}) {
  return {
    id: '123',  // Duplicate IDs!
    email: 'test@test.com',  // Duplicate emails!
    name: 'Test',
    ...overrides
  }
}
```

### 3. Clear Override Syntax

```typescript
// ✅ GOOD - Explicit overrides
const admin = createUser({ role: 'admin', verified: true })

// ❌ BAD - Magic strings
const admin = createUser('admin', 'verified')
```

### 4. Associations

```typescript
// tests/factories/post.ts
export function createPost(overrides = {}) {
  return {
    id: faker.string.uuid(),
    title: faker.lorem.sentence(),
    content: faker.lorem.paragraphs(3),
    author: createUser(),  // Create associated user
    ...overrides
  }
}

// Override association
const post = createPost({
  author: createUser({ name: 'Specific Author' })
})
```

### 5. Cleanup Between Tests

```typescript
beforeEach(() => {
  resetAllSequences()
})

afterEach(async () => {
  await clearDatabase()
})
```

## Edge Cases Testing

```typescript
// tests/factories/user.ts
export function createInvalidUser(type: 'email' | 'age' | 'name') {
  const base = createUser()

  switch (type) {
    case 'email':
      return { ...base, email: 'invalid-email' }
    case 'age':
      return { ...base, age: -1 }
    case 'name':
      return { ...base, name: '' }
  }
}

// Usage
test('validates email format', () => {
  const user = createInvalidUser('email')
  expect(() => validateUser(user)).toThrow('Invalid email')
})
```

## Performance Tips

1. **Use build() not create()** - Create in-memory, don't hit DB unless needed
2. **Reset sequences** - Avoid collisions between tests
3. **Lazy generation** - Only generate what's needed
4. **Reuse factories** - Don't duplicate factory logic

## Complete Example

```typescript
// tests/factories/UserFactory.ts
import { Factory } from 'fishery'
import { faker } from '@faker-js/faker'
import { User } from '@/types'
import { postFactory } from './PostFactory'

interface UserTransientParams {
  postCount?: number
}

export const userFactory = Factory.define<User, UserTransientParams>(
  ({ sequence, params, transientParams }) => ({
    id: `user-${sequence}`,
    email: params.email || faker.internet.email(),
    name: params.name || faker.person.fullName(),
    role: params.role || 'user',
    verified: params.verified ?? true,
    posts: transientParams.postCount
      ? postFactory.buildList(transientParams.postCount, { authorId: `user-${sequence}` })
      : [],
    createdAt: params.createdAt || new Date()
  })
)

// Traits
export const adminFactory = userFactory.params({ role: 'admin' })
export const unverifiedFactory = userFactory.params({ verified: false })

// Usage
const user = userFactory.build()
const admin = adminFactory.build()
const userWithPosts = userFactory.build({}, { transient: { postCount: 5 } })
const specificUser = userFactory.build({ email: 'test@example.com' })
```
