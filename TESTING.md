# Testing Guide

This document provides comprehensive information about testing the first-rust-app application.

## Test Structure

The application has a multi-layered testing approach:

- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test component interactions
- **End-to-End Tests**: Test complete user workflows
- **Performance Tests**: Test system performance under load

## Test Categories

### 1. Unit Tests (`src/` modules)

Located alongside the source code with `#[cfg(test)]` modules.

**Coverage:**
- **Authentication Module** (`src/auth/`)
  - JWT token generation and verification
  - Password hashing and verification
  - Edge cases and security scenarios
  
- **Handlers** (`src/handlers/`)
  - Health endpoint responses
  - Request/response validation
  - Error handling
  
- **Services** (`src/services/`)
  - User service database operations
  - Redis caching functionality
  - Data serialization/deserialization
  
- **Middleware** (`src/middleware/`)
  - Authentication middleware logic
  - Token parsing and validation
  - Path-based access control

### 2. Integration Tests (`tests/`)

Test interactions between components with mock services.

- **Handler Integration** (`tests/integration_handlers.rs`)
  - HTTP endpoint testing
  - Request/response cycles
  - Authentication flows

### 3. End-to-End Tests (`e2e-tests/`)

Complete workflow testing with Playwright.

- **Health API** (`e2e-tests/health.spec.ts`)
- **Authentication Flows** (`e2e-tests/auth.spec.ts`) 
- **User Management** (`e2e-tests/users.spec.ts`)

## Running Tests

### Prerequisites

1. **Database Services**: PostgreSQL, Redis, MongoDB
2. **Node.js**: For E2E tests (>=18.0.0)
3. **Playwright**: For browser automation

### Quick Setup

```bash
# Start databases with Kind cluster
nu setup.nu

# Or manually with Docker
docker run -d --name test-postgres -e POSTGRES_PASSWORD=test -e POSTGRES_USER=test -e POSTGRES_DB=test -p 5432:5432 postgres:15
docker run -d --name test-redis -p 6379:6379 redis:7-alpine
docker run -d --name test-mongo -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=password123 -p 27017:27017 mongo:7
```

### Unit Tests

```bash
# Run all unit tests
cargo test

# Run specific module tests
cargo test auth::
cargo test handlers::
cargo test services::

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html --output-dir coverage/

# Test specific functions
cargo test test_hash_password
cargo test test_health_check
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration_handlers

# With environment variables
DATABASE_URL=postgres://test:test@localhost:5432/test \
REDIS_URL=redis://localhost:6379 \
MONGODB_URL=mongodb://admin:password123@localhost:27017 \
cargo test --test integration_handlers
```

### End-to-End Tests

```bash
# Install dependencies
npm install

# Install Playwright browsers
npm run test:install

# Run all E2E tests
npm run test:e2e

# Run specific test files
npx playwright test health.spec.ts
npx playwright test auth.spec.ts
npx playwright test users.spec.ts

# Run with UI mode
npm run test:e2e:ui

# Run in headed mode (see browser)
npm run test:e2e:headed

# Debug tests
npm run test:e2e:debug

# Generate test report
npm run test:e2e:report
```

## Test Configuration

### Environment Variables

```bash
# Required for all tests
export DATABASE_URL="postgres://test:test@localhost:5432/test"
export REDIS_URL="redis://localhost:6379"
export MONGODB_URL="mongodb://admin:password123@localhost:27017"
export JWT_SECRET="test-secret-key"

# Optional
export JWT_EXPIRES_IN="3600"
export JWT_REFRESH_EXPIRES_IN="604800"
export PORT="8080"
```

### Configuration Files

- **`test-config.toml`**: Test-specific configuration
- **`playwright.config.ts`**: E2E test configuration
- **`.env.example`**: Environment variable examples

## Test Data Management

### Test Database Setup

```bash
# Run migrations
cargo install sqlx-cli
sqlx migrate run --database-url postgres://test:test@localhost:5432/test

# Reset test database
sqlx database reset --database-url postgres://test:test@localhost:5432/test
```

### Test Data Isolation

- Each E2E test uses unique email addresses with UUIDs
- Database operations use transactions where possible  
- Redis keys are prefixed with test identifiers
- MongoDB collections are cleaned between test runs

## Continuous Integration

Tests run automatically on:
- **Push** to main/master branch
- **Pull Requests** to main/master branch

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
- Unit Tests (with PostgreSQL, Redis, MongoDB services)
- Integration Tests  
- E2E Tests
- Security Audit
- Coverage Reporting
```

### Coverage Requirements

- **Minimum Coverage**: 80%
- **Fail Threshold**: 75%
- **Coverage Report**: Uploaded to Codecov

## Test Development Guidelines

### Writing Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_ok, assert_err};

    #[test]
    fn test_function_success_case() {
        let result = function_under_test("valid_input");
        assert_ok!(result);
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_function_error_case() {
        let result = function_under_test("invalid_input");
        assert_err!(result);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Writing E2E Tests

```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature Name', () => {
  test('should handle normal case', async ({ request }) => {
    const response = await request.get('/api/endpoint');
    expect(response.status()).toBe(200);
    
    const data = await response.json();
    expect(data).toHaveProperty('expected_field');
  });

  test('should handle error case', async ({ request }) => {
    const response = await request.post('/api/endpoint', {
      data: { invalid: 'data' }
    });
    expect(response.status()).toBe(400);
  });
});
```

### Test Naming Conventions

- **Unit Tests**: `test_function_name_scenario`
- **Integration Tests**: `test_integration_feature_scenario`  
- **E2E Tests**: `should describe expected behavior`

### Edge Cases to Test

1. **Input Validation**
   - Empty/null values
   - Very long strings
   - Special characters
   - Unicode characters
   - Malformed data

2. **Authentication**
   - Missing tokens
   - Invalid tokens
   - Expired tokens
   - Malformed headers

3. **Concurrency**
   - Multiple simultaneous requests
   - Race conditions
   - Resource contention

4. **Performance**
   - Response time limits
   - Memory usage
   - Database connection limits

5. **Security**
   - SQL injection attempts
   - XSS prevention
   - Authorization bypass attempts

## Troubleshooting

### Common Issues

1. **Database Connection Errors**
   ```bash
   # Check if databases are running
   docker ps
   # Verify connection strings
   psql $DATABASE_URL -c "SELECT 1"
   ```

2. **E2E Test Failures**
   ```bash
   # Check if application is running
   curl http://localhost:8080/api/health
   # Check browser installation
   npx playwright install --with-deps
   ```

3. **Test Timeouts**
   - Increase timeouts in `playwright.config.ts`
   - Check database performance
   - Verify resource availability

### Debug Commands

```bash
# Verbose test output
cargo test -- --nocapture

# Run single test with logging
RUST_LOG=debug cargo test test_name -- --nocapture

# E2E test debugging
npx playwright test --debug
npx playwright test --headed --slowMo=1000
```

## Performance Benchmarks

### Response Time Targets

- **Health Check**: < 50ms
- **Authentication**: < 200ms  
- **User Operations**: < 500ms
- **Complex Queries**: < 1000ms

### Load Testing

```bash
# Simple load test with curl
for i in {1..100}; do
  curl -s http://localhost:8080/api/health > /dev/null &
done
wait

# Or use proper load testing tools
# wrk -t12 -c400 -d30s http://localhost:8080/api/health
```

## Security Testing

### Automated Security Checks

```bash
# Dependency vulnerability scan
cargo audit

# Security advisory check
cargo deny check advisories

# Static analysis
cargo clippy -- -D warnings
```

### Manual Security Testing

1. **Authentication Bypass**
2. **Authorization Escalation**
3. **Input Sanitization**
4. **Rate Limiting**
5. **CORS Configuration**

## Contributing to Tests

1. **Add tests for new features**
2. **Maintain existing test coverage**
3. **Update tests when changing APIs**
4. **Follow testing conventions**
5. **Document complex test scenarios**

### Test Review Checklist

- [ ] Tests cover happy path scenarios
- [ ] Tests cover error conditions  
- [ ] Tests cover edge cases
- [ ] Tests are deterministic
- [ ] Tests clean up after themselves
- [ ] Tests have clear assertions
- [ ] Tests are well-documented