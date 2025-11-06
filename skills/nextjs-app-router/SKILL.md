---
name: nextjs-app-router
description: Next.js App Router (app/) patterns including Server Components, Server Actions, streaming, metadata, and route handlers. Use when building new Next.js 13+ apps, migrating from Pages Router, or implementing server-first features.
---

# Next.js App Router Skill

Complete guide to Next.js App Router (app directory) patterns and best practices.

## When to Use

- Building new Next.js 13+ applications
- Migrating from Pages Router to App Router
- Implementing server-first features
- Need better performance (React Server Components)
- Want simplified data fetching

## Key Concepts

### Server vs Client Components

**Server Components (default):**
- Run on server only
- Can use async/await directly
- Can access backend resources
- Zero JS sent to client
- Cannot use hooks (useState, useEffect, etc.)

**Client Components:**
- Run on client (and server for SSR)
- Can use hooks
- Can handle user interactions
- Use 'use client' directive

## Directory Structure

```
app/
├── layout.tsx          # Root layout (required)
├── page.tsx           # Home page
├── loading.tsx        # Loading UI
├── error.tsx          # Error UI
├── not-found.tsx      # 404 UI
├── template.tsx       # Re-mounted layout
├── global.css         # Global styles
│
├── (auth)/           # Route group (doesn't affect URL)
│   ├── login/
│   │   └── page.tsx  # /login
│   └── register/
│       └── page.tsx  # /register
│
├── blog/
│   ├── layout.tsx    # Blog layout
│   ├── page.tsx      # /blog
│   └── [slug]/
│       └── page.tsx  # /blog/my-post
│
└── api/
    └── users/
        └── route.ts  # API endpoint /api/users
```

## Patterns

### 1. Server Component (Default)

```typescript
// app/users/page.tsx
// NO 'use client' directive = Server Component

async function getUsers() {
  const res = await fetch('https://api.example.com/users', {
    cache: 'no-store' // or 'force-cache' for static
  })
  return res.json()
}

// Can be async!
export default async function UsersPage() {
  const users = await getUsers()

  return (
    <div>
      {users.map(user => (
        <div key={user.id}>{user.name}</div>
      ))}
    </div>
  )
}
```

### 2. Client Component (Interactive)

```typescript
// app/counter/page.tsx
'use client'  // ← Required for hooks

import { useState } from 'react'

export default function Counter() {
  const [count, setCount] = useState(0)

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={() => setCount(count + 1)}>
        Increment
      </button>
    </div>
  )
}
```

### 3. Mixing Server + Client

```typescript
// ✅ GOOD - Server Component wraps Client Component
// app/users/page.tsx (Server Component)
import UserList from './UserList'

async function getUsers() {
  const res = await fetch('https://api.example.com/users')
  return res.json()
}

export default async function UsersPage() {
  const users = await getUsers()  // Fetched on server

  return <UserList initialUsers={users} />  // Pass to client
}

// app/users/UserList.tsx (Client Component)
'use client'

import { useState } from 'react'

export default function UserList({ initialUsers }) {
  const [users, setUsers] = useState(initialUsers)

  const handleDelete = (id) => {
    setUsers(users.filter(u => u.id !== id))
  }

  return (
    <div>
      {users.map(user => (
        <div key={user.id}>
          {user.name}
          <button onClick={() => handleDelete(user.id)}>Delete</button>
        </div>
      ))}
    </div>
  )
}
```

### 4. Server Actions

```typescript
// app/todos/actions.ts
'use server'  // ← Server Action

import { revalidatePath } from 'next/cache'

export async function createTodo(formData: FormData) {
  const title = formData.get('title')

  await db.todos.create({
    data: { title, completed: false }
  })

  revalidatePath('/todos')  // Refresh the page data
}

export async function deleteTodo(id: string) {
  await db.todos.delete({ where: { id } })
  revalidatePath('/todos')
}

// app/todos/page.tsx (Server Component)
import { createTodo, deleteTodo } from './actions'

export default async function TodosPage() {
  const todos = await db.todos.findMany()

  return (
    <div>
      <form action={createTodo}>
        <input name="title" required />
        <button type="submit">Add</button>
      </form>

      {todos.map(todo => (
        <div key={todo.id}>
          {todo.title}
          <form action={deleteTodo.bind(null, todo.id)}>
            <button>Delete</button>
          </form>
        </div>
      ))}
    </div>
  )
}
```

### 5. Route Handlers (API Routes)

```typescript
// app/api/users/route.ts
import { NextResponse } from 'next/server'

export async function GET(request: Request) {
  const users = await db.users.findMany()
  return NextResponse.json(users)
}

export async function POST(request: Request) {
  const body = await request.json()

  const user = await db.users.create({
    data: body
  })

  return NextResponse.json(user, { status: 201 })
}

// app/api/users/[id]/route.ts
export async function GET(
  request: Request,
  { params }: { params: { id: string } }
) {
  const user = await db.users.findUnique({
    where: { id: params.id }
  })

  if (!user) {
    return NextResponse.json({ error: 'Not found' }, { status: 404 })
  }

  return NextResponse.json(user)
}
```

### 6. Dynamic Routes

```typescript
// app/blog/[slug]/page.tsx
export default async function BlogPost({
  params
}: {
  params: { slug: string }
}) {
  const post = await getPost(params.slug)

  return (
    <article>
      <h1>{post.title}</h1>
      <div>{post.content}</div>
    </article>
  )
}

// Generate static pages at build time
export async function generateStaticParams() {
  const posts = await getPosts()

  return posts.map((post) => ({
    slug: post.slug
  }))
}

// Catch-all: app/docs/[...slug]/page.tsx
// Matches: /docs/a, /docs/a/b, /docs/a/b/c
export default async function Docs({
  params
}: {
  params: { slug: string[] }
}) {
  const path = params.slug.join('/')
  return <div>Path: {path}</div>
}
```

### 7. Loading & Streaming

```typescript
// app/dashboard/loading.tsx
export default function Loading() {
  return <div>Loading dashboard...</div>
}

// app/dashboard/page.tsx
import { Suspense } from 'react'
import Analytics from './Analytics'
import RecentOrders from './RecentOrders'

export default function Dashboard() {
  return (
    <div>
      <h1>Dashboard</h1>

      {/* Show skeleton while Analytics loads */}
      <Suspense fallback={<AnalyticsSkeleton />}>
        <Analytics />
      </Suspense>

      {/* Show skeleton while Orders loads */}
      <Suspense fallback={<OrdersSkeleton />}>
        <RecentOrders />
      </Suspense>
    </div>
  )
}
```

### 8. Error Handling

```typescript
// app/error.tsx
'use client'  // Error boundaries must be Client Components

export default function Error({
  error,
  reset
}: {
  error: Error
  reset: () => void
}) {
  return (
    <div>
      <h2>Something went wrong!</h2>
      <button onClick={reset}>Try again</button>
    </div>
  )
}

// app/not-found.tsx
export default function NotFound() {
  return (
    <div>
      <h2>404 - Page Not Found</h2>
      <Link href="/">Go home</Link>
    </div>
  )
}
```

### 9. Metadata

```typescript
// app/layout.tsx
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: {
    default: 'My App',
    template: '%s | My App'  // "Page Title | My App"
  },
  description: 'My awesome app',
  openGraph: {
    title: 'My App',
    description: 'My awesome app',
    images: ['/og-image.png']
  }
}

// app/blog/[slug]/page.tsx
export async function generateMetadata({
  params
}: {
  params: { slug: string }
}): Promise<Metadata> {
  const post = await getPost(params.slug)

  return {
    title: post.title,
    description: post.excerpt,
    openGraph: {
      images: [post.coverImage]
    }
  }
}
```

### 10. Middleware

```typescript
// middleware.ts (root of project)
import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  // Authentication check
  const token = request.cookies.get('token')

  if (!token && request.nextUrl.pathname.startsWith('/dashboard')) {
    return NextResponse.redirect(new URL('/login', request.url))
  }

  return NextResponse.next()
}

export const config = {
  matcher: '/dashboard/:path*'
}
```

## Data Fetching Patterns

### Fetch with Caching

```typescript
// Static (cached indefinitely)
const res = await fetch('https://api.example.com/posts', {
  cache: 'force-cache'  // default
})

// Dynamic (no cache)
const res = await fetch('https://api.example.com/posts', {
  cache: 'no-store'
})

// Revalidate (ISR - Incremental Static Regeneration)
const res = await fetch('https://api.example.com/posts', {
  next: { revalidate: 3600 }  // Revalidate every hour
})
```

### Parallel Data Fetching

```typescript
// ✅ GOOD - Parallel fetching
export default async function Page() {
  const [users, posts] = await Promise.all([
    getUsers(),
    getPosts()
  ])

  return <div>{/* ... */}</div>
}

// ❌ BAD - Sequential (slower)
export default async function Page() {
  const users = await getUsers()
  const posts = await getPosts()  // Waits for users first!

  return <div>{/* ... */}</div>
}
```

## Migration from Pages Router

### Before (Pages Router)

```typescript
// pages/blog/[slug].tsx
import { GetStaticProps } from 'next'

export default function BlogPost({ post }) {
  return <div>{post.title}</div>
}

export const getStaticProps: GetStaticProps = async ({ params }) => {
  const post = await getPost(params.slug)
  return { props: { post } }
}

export const getStaticPaths = async () => {
  const posts = await getPosts()
  return {
    paths: posts.map(p => ({ params: { slug: p.slug } })),
    fallback: false
  }
}
```

### After (App Router)

```typescript
// app/blog/[slug]/page.tsx
export default async function BlogPost({
  params
}: {
  params: { slug: string }
}) {
  const post = await getPost(params.slug)
  return <div>{post.title}</div>
}

export async function generateStaticParams() {
  const posts = await getPosts()
  return posts.map(p => ({ slug: p.slug }))
}
```

## Common Pitfalls

### 1. Using hooks in Server Components

```typescript
// ❌ BAD
export default function Page() {
  const [count, setCount] = useState(0)  // Error!
  return <div>{count}</div>
}

// ✅ GOOD - Add 'use client'
'use client'

export default function Page() {
  const [count, setCount] = useState(0)
  return <div>{count}</div>
}
```

### 2. Importing Server Component in Client Component

```typescript
// ❌ BAD
'use client'

import ServerComponent from './ServerComponent'  // Error!

export default function ClientComponent() {
  return <ServerComponent />
}

// ✅ GOOD - Pass as children
'use client'

export default function ClientComponent({ children }) {
  return <div>{children}</div>
}

// Parent (Server Component)
import ClientComponent from './ClientComponent'
import ServerComponent from './ServerComponent'

export default function Parent() {
  return (
    <ClientComponent>
      <ServerComponent />  {/* Passed as children */}
    </ClientComponent>
  )
}
```

### 3. Forgetting revalidatePath after mutations

```typescript
// ❌ BAD - Data won't refresh
'use server'

export async function createPost(data) {
  await db.posts.create({ data })
  // Missing revalidatePath!
}

// ✅ GOOD
'use server'

import { revalidatePath } from 'next/cache'

export async function createPost(data) {
  await db.posts.create({ data })
  revalidatePath('/blog')  // Refresh blog page
}
```

## Performance Best Practices

1. **Use Server Components by default** - Less JS to client
2. **Push 'use client' down** - Only mark interactive parts
3. **Parallel fetch when possible** - Use Promise.all
4. **Use Suspense for streaming** - Better UX
5. **Implement loading states** - loading.tsx files
6. **Cache aggressively** - Use force-cache where possible
7. **Use generateStaticParams** - Pre-render dynamic pages

## Checklist

- [ ] Use Server Components by default
- [ ] Only add 'use client' when needed (hooks, events)
- [ ] Implement loading.tsx for better UX
- [ ] Add error.tsx for error boundaries
- [ ] Use Server Actions for mutations
- [ ] Implement metadata for SEO
- [ ] Use Suspense for streaming
- [ ] Cache fetch requests appropriately
- [ ] Use route groups for organization
- [ ] Add middleware for auth/redirects
