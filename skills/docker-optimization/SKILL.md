---
name: docker-optimization
description: Optimize Docker images using multi-stage builds, layer caching, and minimal base images. Use when building production containers, reducing image sizes, improving CI/CD pipeline speed, or deploying to Kubernetes.
---

# Docker Optimization Skill

Optimize Docker images for size, build speed, and security.

## When to Use

- Docker images too large (>500MB)
- Slow Docker builds in CI/CD
- Deploying to Kubernetes/cloud
- Security vulnerabilities in base images
- Need faster container startup

## Multi-Stage Builds

### Before (Single Stage - BAD)

```dockerfile
FROM node:20
WORKDIR /app

# Install ALL dependencies (dev + prod)
COPY package*.json ./
RUN npm install  # Includes devDependencies!

# Copy source
COPY . .

# Build
RUN npm run build

# Run
CMD ["node", "dist/index.js"]

# Result: ~1.2GB image with build tools!
```

### After (Multi-Stage - GOOD)

```dockerfile
# Stage 1: Dependencies
FROM node:20-slim AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

# Stage 2: Build
FROM node:20-slim AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci  # Include devDependencies for build
COPY . .
RUN npm run build

# Stage 3: Production
FROM node:20-slim
WORKDIR /app

# Copy only production dependencies
COPY --from=deps /app/node_modules ./node_modules

# Copy built application
COPY --from=builder /app/dist ./dist
COPY package*.json ./

# Run as non-root
USER node
CMD ["node", "dist/index.js"]

# Result: ~200MB image, 6x smaller!
```

## Layer Caching Strategy

### Optimize Layer Order

```dockerfile
# ❌ BAD - Cache breaks on any code change
FROM node:20-slim
WORKDIR /app
COPY . .              # Everything changes = no cache
RUN npm install
RUN npm run build

# ✅ GOOD - Dependencies cached separately
FROM node:20-slim
WORKDIR /app

# 1. Copy only package files first
COPY package*.json ./
RUN npm ci  # Cached unless package.json changes

# 2. Then copy source (changes frequently)
COPY . .
RUN npm run build  # Rebuilds only when source changes
```

## Minimal Base Images

### Size Comparison

```dockerfile
# node:20 (default)        = 1.1GB
# node:20-slim             = 240MB (no python, gcc)
# node:20-alpine           = 180MB (Alpine Linux)
# gcr.io/distroless/nodejs = 120MB (Google's minimal)
```

### Alpine Linux

```dockerfile
FROM node:20-alpine
WORKDIR /app

# Alpine uses apk instead of apt
RUN apk add --no-cache python3 make g++  # If needed for native deps

COPY package*.json ./
RUN npm ci --only=production

COPY . .

CMD ["node", "index.js"]
```

### Distroless (Most Secure)

```dockerfile
# Build stage
FROM node:20-slim AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Production stage
FROM gcr.io/distroless/nodejs20-debian12
WORKDIR /app

COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist

CMD ["dist/index.js"]

# No shell, no package manager = ultra secure!
```

## .dockerignore (Critical!)

```
# .dockerignore
node_modules
npm-debug.log
dist
build
.git
.gitignore
.env
.env.local
README.md
.vscode
.idea
*.md
.DS_Store
.github
coverage
.nyc_output

# Test files
*.test.js
*.spec.js
__tests__
__mocks__

# Documentation
docs/
*.md

# CI/CD
.gitlab-ci.yml
.github/
.circleci/

# Reduces COPY context by 90%+!
```

## Full-Stack Examples

### Node.js + TypeScript

```dockerfile
# Build dependencies
FROM node:20-alpine AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production && npm cache clean --force

# Build application
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build
RUN npm prune --production

# Production
FROM node:20-alpine
WORKDIR /app

# Security: Run as non-root
RUN addgroup -g 1001 -S nodejs && \
    adduser -S nodejs -u 1001

COPY --from=deps --chown=nodejs:nodejs /app/node_modules ./node_modules
COPY --from=builder --chown=nodejs:nodejs /app/dist ./dist
COPY --chown=nodejs:nodejs package*.json ./

USER nodejs
EXPOSE 3000

CMD ["node", "dist/index.js"]
```

### Next.js

```dockerfile
FROM node:20-alpine AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package*.json ./
RUN npm ci

FROM node:20-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

ENV NEXT_TELEMETRY_DISABLED 1
RUN npm run build

FROM node:20-alpine AS runner
WORKDIR /app

ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1

RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# Copy only necessary files
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs
EXPOSE 3000

CMD ["node", "server.js"]
```

### Python + Flask

```dockerfile
# Build stage
FROM python:3.11-slim AS builder
WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --user --no-cache-dir -r requirements.txt

# Production stage
FROM python:3.11-slim
WORKDIR /app

# Copy dependencies from builder
COPY --from=builder /root/.local /root/.local

# Copy application
COPY . .

# Make sure scripts in .local are usable
ENV PATH=/root/.local/bin:$PATH

# Run as non-root
RUN useradd -m -u 1000 appuser && chown -R appuser /app
USER appuser

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "app:app"]
```

### Go

```dockerfile
# Build stage
FROM golang:1.21-alpine AS builder
WORKDIR /app

# Download dependencies
COPY go.mod go.sum ./
RUN go mod download

# Build
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o main .

# Production stage
FROM scratch  # Empty image!

# Copy CA certificates for HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary
COPY --from=builder /app/main /main

EXPOSE 8080
CMD ["/main"]

# Result: ~10-20MB image!
```

## Build Optimizations

### BuildKit (Faster Builds)

```bash
# Enable BuildKit
export DOCKER_BUILDKIT=1

# Or set in daemon.json
{
  "features": {
    "buildkit": true
  }
}
```

### Cache Mounts (Speed Up npm install)

```dockerfile
FROM node:20-alpine
WORKDIR /app

COPY package*.json ./

# Use cache mount for npm cache
RUN --mount=type=cache,target=/root/.npm \
    npm ci --only=production

COPY . .
CMD ["node", "index.js"]
```

### Parallel Stages

```dockerfile
# Stage 1 & 2 run in parallel!
FROM node:20-slim AS frontend-deps
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci

FROM node:20-slim AS backend-deps
WORKDIR /app/backend
COPY backend/package*.json ./
RUN npm ci

# Final stage combines both
FROM node:20-slim
COPY --from=frontend-deps /app/frontend/node_modules /app/frontend/node_modules
COPY --from=backend-deps /app/backend/node_modules /app/backend/node_modules
# ...
```

## Security Best Practices

### 1. Use Specific Versions

```dockerfile
# ❌ BAD - Version can change
FROM node:latest

# ✅ GOOD - Pinned version
FROM node:20.10.0-alpine

# ✅ BETTER - SHA256 digest
FROM node:20.10.0-alpine@sha256:abc123...
```

### 2. Run as Non-Root

```dockerfile
FROM node:20-alpine
WORKDIR /app

# Create user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S nodejs -u 1001

# Install/build as root
COPY package*.json ./
RUN npm ci

# Switch to user for runtime
COPY --chown=nodejs:nodejs . .

USER nodejs
CMD ["node", "index.js"]
```

### 3. Scan for Vulnerabilities

```bash
# Docker Scout
docker scout cves myimage:latest

# Trivy
trivy image myimage:latest

# Snyk
snyk container test myimage:latest
```

### 4. Use Read-Only Filesystem

```dockerfile
FROM node:20-alpine
WORKDIR /app

# ... build steps ...

# Make filesystem read-only
USER nodejs
CMD ["node", "index.js"]

# In docker run:
# docker run --read-only myimage
```

## Compose Optimization

```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
      # Use cache from registry
      cache_from:
        - myregistry.com/myapp:latest
      # BuildKit features
      target: production
      args:
        BUILDKIT_INLINE_CACHE: 1
    image: myapp:${VERSION:-latest}
    restart: unless-stopped
    # Health check
    healthcheck:
      test: ["CMD", "node", "healthcheck.js"]
      interval: 30s
      timeout: 3s
      retries: 3
    # Resource limits
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

## CI/CD Optimization

### GitHub Actions

```yaml
# .github/workflows/docker.yml
name: Docker Build

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Enable BuildKit
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      # Cache Docker layers
      - name: Cache Docker layers
        uses: actions/cache@v3
        with:
          path: /tmp/.buildx-cache
          key: ${{ runner.os }}-buildx-${{ github.sha }}
          restore-keys: |
            ${{ runner.os }}-buildx-

      # Build with cache
      - name: Build
        uses: docker/build-push-action@v5
        with:
          context: .
          push: false
          cache-from: type=local,src=/tmp/.buildx-cache
          cache-to: type=local,dest=/tmp/.buildx-cache-new,mode=max

      # Rotate cache
      - name: Move cache
        run: |
          rm -rf /tmp/.buildx-cache
          mv /tmp/.buildx-cache-new /tmp/.buildx-cache
```

## Metrics

### Measure Image Size

```bash
# Check image size
docker images myapp

# Analyze layers
docker history myapp:latest

# See what's taking space
dive myapp:latest  # Install: https://github.com/wagoodman/dive
```

### Expected Results

| Type | Before | After | Improvement |
|------|--------|-------|-------------|
| Node.js | 1.2GB | 200MB | 6x smaller |
| Next.js | 1.5GB | 150MB | 10x smaller |
| Python | 800MB | 100MB | 8x smaller |
| Go | 800MB | 15MB | 53x smaller |

## Checklist

- [ ] Use multi-stage builds
- [ ] Use minimal base image (alpine/distroless)
- [ ] Add .dockerignore
- [ ] Optimize layer order (deps before source)
- [ ] Run as non-root user
- [ ] Pin image versions
- [ ] Scan for vulnerabilities
- [ ] Use cache mounts where possible
- [ ] Enable BuildKit
- [ ] Test with `--read-only` flag
- [ ] Add health checks
- [ ] Set resource limits
