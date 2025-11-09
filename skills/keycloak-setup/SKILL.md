---
name: keycloak-setup
description: Configure Keycloak realm, OAuth 2.0/OIDC clients, multi-tenant user attributes, TOTP/2FA, and test users via REST API. Use when setting up Keycloak authentication, implementing tenant isolation with custom claims, or troubleshooting Keycloak configuration issues.
---

# Keycloak Setup Skill

Complete guide for setting up and configuring Keycloak for production use.

## When to Use This Skill

- Setting up a new Keycloak realm for your application
- Configuring OAuth 2.0 / OpenID Connect authentication
- Implementing multi-tenant authentication with custom claims
- Setting up 2FA/TOTP requirements
- Configuring client applications (backend/frontend)
- Troubleshooting Keycloak configuration issues

## Prerequisites

1. Keycloak container running and healthy
2. Admin credentials configured
3. Docker Compose setup with Keycloak service
4. `jq` installed for JSON parsing

**IMPORTANT NOTES:**
- This guide uses Keycloak 17+ (Quarkus distribution) which removed the `/auth` prefix from URLs
- If using older versions (16 and below), add `/auth` after the port: `http://localhost:8080/auth/admin/realms`
- All REST API examples use `Bearer` token authentication
- Token expires after a short period - regenerate if requests return 401

## Step 1: Verify Keycloak is Running

```bash
# Check Keycloak container status
docker compose ps keycloak

# Verify Keycloak is responding
curl -s http://localhost:8080/realms/master | jq .

# Get admin token (verify credentials work)
curl -X POST \
  "http://localhost:8080/realms/master/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin" \
  -d "password=${KEYCLOAK_ADMIN_PASSWORD}" \
  -d "grant_type=password" \
  -d "client_id=admin-cli"
```

## Step 2: Create Custom Realm

```bash
# Set environment variables
KEYCLOAK_URL="http://localhost:8080"
REALM_NAME="login-system"
ADMIN_TOKEN="<get-from-step-1>"

# Create realm
curl -X POST \
  "$KEYCLOAK_URL/admin/realms" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "realm": "'"$REALM_NAME"'",
    "enabled": true,
    "displayName": "Login System",
    "resetPasswordAllowed": true,
    "loginWithEmailAllowed": true,
    "duplicateEmailsAllowed": false,
    "registrationAllowed": false,
    "accessTokenLifespan": 900,
    "ssoSessionIdleTimeout": 604800,
    "browserSecurityHeaders": {
      "xFrameOptions": "SAMEORIGIN",
      "contentSecurityPolicy": "frame-src '\''self'\''; frame-ancestors '\''self'\''; object-src '\''none'\'';",
      "xContentTypeOptions": "nosniff",
      "strictTransportSecurity": "max-age=31536000; includeSubDomains"
    }
  }'
```

## Step 3: Configure Password Policy

```bash
# Set strong password policy
curl -X PUT \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "passwordPolicy": "length(12) and upperCase(1) and lowerCase(1) and digits(1) and specialChars(1)"
  }'
```

## Step 4: Configure TOTP/2FA

```bash
# Enable TOTP
curl -X PUT \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "otpPolicyType": "totp",
    "otpPolicyAlgorithm": "HmacSHA1",
    "otpPolicyDigits": 6,
    "otpPolicyPeriod": 30
  }'

# Make TOTP a required action
curl -X PUT \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/authentication/required-actions/CONFIGURE_TOTP" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "alias": "CONFIGURE_TOTP",
    "enabled": true,
    "defaultAction": true
  }'
```

## Step 5: Create Client Applications

### Backend Client (Confidential)

```bash
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/clients" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "laravel-backend",
    "enabled": true,
    "publicClient": false,
    "standardFlowEnabled": true,
    "directAccessGrantsEnabled": true,
    "serviceAccountsEnabled": true,
    "redirectUris": ["http://localhost:8000/auth/callback"],
    "webOrigins": ["http://localhost:8000"],
    "defaultClientScopes": ["openid", "profile", "email", "tenant-info"]
  }'

# Get client secret
CLIENT_ID=$(curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/clients?clientId=laravel-backend" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[0].id')

curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/clients/$CLIENT_ID/client-secret" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.value'
```

### Frontend Client (Public with PKCE)

```bash
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/clients" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "react-frontend",
    "enabled": true,
    "publicClient": true,
    "standardFlowEnabled": true,
    "directAccessGrantsEnabled": false,
    "redirectUris": ["http://localhost:3000/*"],
    "webOrigins": ["http://localhost:3000"],
    "attributes": {
      "pkce.code.challenge.method": "S256"
    },
    "defaultClientScopes": ["openid", "profile", "email", "tenant-info"]
  }'
```

## Step 6: Create Custom Client Scope (Multi-Tenancy)

**Best Practice for Multi-Tenancy:**
- Use namespaced claim names to avoid collision with standard OIDC claims (e.g., `tenant.id` instead of `tenant_id`)
- Only enable claims in tokens where needed (access token vs ID token vs UserInfo)
- Always validate tenant claims in your application on every request
- Consider using groups to assign tenant attributes automatically

```bash
# Create tenant-info scope
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/client-scopes" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant-info",
    "protocol": "openid-connect",
    "description": "Multi-tenant isolation claims",
    "attributes": {
      "include.in.token.scope": "true",
      "display.on.consent.screen": "false"
    }
  }'

# Get scope ID
SCOPE_ID=$(curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/client-scopes" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[] | select(.name=="tenant-info") | .id')

# Add tenant_id mapper
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/client-scopes/$SCOPE_ID/protocol-mappers/models" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant_id",
    "protocol": "openid-connect",
    "protocolMapper": "oidc-usermodel-attribute-mapper",
    "config": {
      "user.attribute": "tenant_id",
      "claim.name": "tenant_id",
      "jsonType.label": "String",
      "id.token.claim": "true",
      "access.token.claim": "true",
      "userinfo.token.claim": "true"
    }
  }'

# Add tenant_name mapper
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/client-scopes/$SCOPE_ID/protocol-mappers/models" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant_name",
    "protocol": "openid-connect",
    "protocolMapper": "oidc-usermodel-attribute-mapper",
    "config": {
      "user.attribute": "tenant_name",
      "claim.name": "tenant_name",
      "jsonType.label": "String",
      "id.token.claim": "true",
      "access.token.claim": "true",
      "userinfo.token.claim": "true"
    }
  }'

# Add groups mapper
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/client-scopes/$SCOPE_ID/protocol-mappers/models" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "groups",
    "protocol": "openid-connect",
    "protocolMapper": "oidc-group-membership-mapper",
    "config": {
      "claim.name": "groups",
      "full.path": "false",
      "id.token.claim": "true",
      "access.token.claim": "true",
      "userinfo.token.claim": "true"
    }
  }'
```

## Step 7: Create Realm Roles

```bash
# Create admin role
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/roles" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "admin",
    "description": "Administrator role"
  }'

# Create user role
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/roles" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "user",
    "description": "Standard user role"
  }'
```

## Step 8: Create Tenant Groups

```bash
# Create ACME Corp group
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/groups" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant-acme-corp",
    "attributes": {
      "tenant_id": ["acme-corp"],
      "tenant_name": ["ACME Corporation"]
    }
  }'

# Create Tech Startup group
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/groups" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "tenant-tech-startup",
    "attributes": {
      "tenant_id": ["tech-startup"],
      "tenant_name": ["Tech Startup Inc"]
    }
  }'
```

## Step 9: Create Test Users

```bash
# Create test user for ACME Corp
curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/users" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john.doe@acme-corp.test",
    "email": "john.doe@acme-corp.test",
    "firstName": "John",
    "lastName": "Doe",
    "enabled": true,
    "emailVerified": true,
    "attributes": {
      "tenant_id": ["acme-corp"],
      "tenant_name": ["ACME Corporation"]
    },
    "requiredActions": ["CONFIGURE_TOTP"],
    "credentials": [{
      "type": "password",
      "value": "Test@123456",
      "temporary": false
    }]
  }'

# Get user ID and assign to group
USER_ID=$(curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/users?username=john.doe@acme-corp.test" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[0].id')

GROUP_ID=$(curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/groups" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[] | select(.name=="tenant-acme-corp") | .id')

curl -X PUT \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/users/$USER_ID/groups/$GROUP_ID" \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Assign user role
USER_ROLE_ID=$(curl -s "$KEYCLOAK_URL/admin/realms/$REALM_NAME/roles/user" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.id')

curl -X POST \
  "$KEYCLOAK_URL/admin/realms/$REALM_NAME/users/$USER_ID/role-mappings/realm" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{
    "id": "'"$USER_ROLE_ID"'",
    "name": "user"
  }]'
```

## Step 10: Verify Configuration

```bash
# Test authentication
curl -X POST \
  "$KEYCLOAK_URL/realms/$REALM_NAME/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=john.doe@acme-corp.test" \
  -d "password=Test@123456" \
  -d "grant_type=password" \
  -d "client_id=laravel-backend" \
  -d "scope=openid profile email tenant-info"

# Decode token to verify claims
# (Token will have tenant_id, tenant_name, groups in the payload)
```

## Common Issues and Solutions

### Issue: "Realm not found"
**Solution**: Ensure realm was created successfully. Check admin token is valid.

### Issue: "Client credentials invalid"
**Solution**: For confidential clients, ensure you're using the client secret. For public clients with PKCE, use code_challenge.

### Issue: "User can't login - TOTP required"
**Solution**: Expected behavior. User must configure TOTP on first login. Use Keycloak admin UI to remove TOTP requirement if needed for testing.

### Issue: "Token doesn't include custom claims"
**Solution**: Verify tenant-info scope is in client's defaultClientScopes and protocol mappers are configured correctly.

## Script-Based Setup

For automated setup, use the provided scripts in `src/scripts/`:

```bash
# Complete Keycloak initialization
cd src/scripts
bash keycloak-init.sh

# Or run individual scripts
bash keycloak-configure-clients.sh
bash keycloak-configure-attributes.sh
bash keycloak-create-test-users.sh
bash keycloak-configure-2fa.sh
```

## Testing the Setup

Run integration tests to verify everything is configured:

```bash
cd src/tests/integration
npm test 05-keycloak.test.js
```

Expected: All tests pass (realm, clients, users, TOTP, tokens with custom claims).

## Next Steps

1. Configure your backend to validate JWT tokens from Keycloak
2. Configure your frontend to use PKCE flow with Keycloak
3. Implement tenant isolation based on tenant_id claim
4. Set up refresh token rotation
5. Configure session management

## Resources

- [Keycloak Admin REST API](https://www.keycloak.org/docs-api/latest/rest-api/)
- [OpenID Connect Flows](https://openid.net/specs/openid-connect-core-1_0.html)
- [PKCE RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636)
