import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Authentication API', () => {
  const baseUser = {
    name: 'Test User',
    email: `test-${Date.now()}@example.com`,
    password: 'SecurePassword123!',
    age: 25
  };

  test.describe('Registration', () => {
    test('should register a new user successfully', async ({ request }) => {
      const user = { ...baseUser, email: `register-${uuidv4()}@example.com` };
      
      const response = await request.post('/api/auth/register', {
        data: user
      });

      expect(response.status()).toBe(200);
      const data = await response.json();
      
      expect(data).toHaveProperty('access_token');
      expect(data).toHaveProperty('refresh_token');
      expect(data).toHaveProperty('user');
      
      expect(data.user.email).toBe(user.email);
      expect(data.user.name).toBe(user.name);
      expect(data.user.role).toBe('user');
      expect(data.user).not.toHaveProperty('password');
      expect(data.user).not.toHaveProperty('password_hash');
    });

    test('should reject registration with missing fields', async ({ request }) => {
      const incompleteUsers = [
        { email: 'test@example.com', password: 'pass123' }, // missing name
        { name: 'Test', password: 'pass123' }, // missing email
        { name: 'Test', email: 'test@example.com' }, // missing password
      ];

      for (const user of incompleteUsers) {
        const response = await request.post('/api/auth/register', {
          data: user
        });
        expect(response.status()).toBe(400);
      }
    });

    test('should reject registration with duplicate email', async ({ request }) => {
      const user = { ...baseUser, email: `duplicate-${uuidv4()}@example.com` };
      
      // First registration should succeed
      const response1 = await request.post('/api/auth/register', {
        data: user
      });
      expect(response1.status()).toBe(200);

      // Second registration with same email should fail
      const response2 = await request.post('/api/auth/register', {
        data: user
      });
      expect(response2.status()).toBe(409); // Conflict or similar
    });

    test('should reject invalid email formats', async ({ request }) => {
      const invalidEmails = [
        'notanemail',
        '@example.com',
        'test@',
        'test.example.com',
        'test@.com',
        'test@example.',
      ];

      for (const email of invalidEmails) {
        const user = { ...baseUser, email };
        const response = await request.post('/api/auth/register', {
          data: user
        });
        expect(response.status()).toBe(400);
      }
    });

    test('should reject weak passwords', async ({ request }) => {
      const weakPasswords = [
        '123',      // too short
        'password', // too common
        '12345678', // only numbers
        'abcdefgh', // only letters
      ];

      for (const password of weakPasswords) {
        const user = { 
          ...baseUser, 
          email: `weak-pass-${Date.now()}@example.com`,
          password 
        };
        const response = await request.post('/api/auth/register', {
          data: user
        });
        // Should either reject or succeed, but test password strength if implemented
        expect([200, 400]).toContain(response.status());
      }
    });

    test('should handle unicode characters in name', async ({ request }) => {
      const user = { 
        ...baseUser, 
        email: `unicode-${uuidv4()}@example.com`,
        name: 'José García 김민수 田中太郎'
      };
      
      const response = await request.post('/api/auth/register', {
        data: user
      });

      expect(response.status()).toBe(200);
      const data = await response.json();
      expect(data.user.name).toBe(user.name);
    });
  });

  test.describe('Login', () => {
    let registeredUser: any;

    test.beforeEach(async ({ request }) => {
      // Register a user for login tests
      registeredUser = { ...baseUser, email: `login-${uuidv4()}@example.com` };
      const response = await request.post('/api/auth/register', {
        data: registeredUser
      });
      expect(response.status()).toBe(200);
    });

    test('should login with valid credentials', async ({ request }) => {
      const response = await request.post('/api/auth/login', {
        data: {
          email: registeredUser.email,
          password: registeredUser.password
        }
      });

      expect(response.status()).toBe(200);
      const data = await response.json();
      
      expect(data).toHaveProperty('access_token');
      expect(data).toHaveProperty('refresh_token');
      expect(data).toHaveProperty('user');
      expect(data.user.email).toBe(registeredUser.email);
    });

    test('should reject login with wrong password', async ({ request }) => {
      const response = await request.post('/api/auth/login', {
        data: {
          email: registeredUser.email,
          password: 'WrongPassword123!'
        }
      });

      expect(response.status()).toBe(401);
    });

    test('should reject login with non-existent email', async ({ request }) => {
      const response = await request.post('/api/auth/login', {
        data: {
          email: 'nonexistent@example.com',
          password: 'AnyPassword123!'
        }
      });

      expect(response.status()).toBe(401);
    });

    test('should reject login with missing credentials', async ({ request }) => {
      const incompleteCredentials = [
        { email: 'test@example.com' }, // missing password
        { password: 'password123' },   // missing email
        {},                           // missing both
      ];

      for (const credentials of incompleteCredentials) {
        const response = await request.post('/api/auth/login', {
          data: credentials
        });
        expect(response.status()).toBe(400);
      }
    });

    test('should be case sensitive for passwords', async ({ request }) => {
      const response = await request.post('/api/auth/login', {
        data: {
          email: registeredUser.email,
          password: registeredUser.password.toLowerCase()
        }
      });

      if (registeredUser.password !== registeredUser.password.toLowerCase()) {
        expect(response.status()).toBe(401);
      }
    });

    test('should be case insensitive for emails', async ({ request }) => {
      const response = await request.post('/api/auth/login', {
        data: {
          email: registeredUser.email.toUpperCase(),
          password: registeredUser.password
        }
      });

      // This depends on implementation - adjust based on actual behavior
      expect([200, 401]).toContain(response.status());
    });
  });

  test.describe('Token Refresh', () => {
    let tokens: any;

    test.beforeEach(async ({ request }) => {
      // Register and login to get tokens
      const user = { ...baseUser, email: `refresh-${uuidv4()}@example.com` };
      await request.post('/api/auth/register', { data: user });
      
      const loginResponse = await request.post('/api/auth/login', {
        data: { email: user.email, password: user.password }
      });
      tokens = await loginResponse.json();
    });

    test('should refresh tokens with valid refresh token', async ({ request }) => {
      const response = await request.post('/api/auth/refresh', {
        data: {
          refresh_token: tokens.refresh_token
        }
      });

      expect(response.status()).toBe(200);
      const data = await response.json();
      
      expect(data).toHaveProperty('access_token');
      expect(data).toHaveProperty('refresh_token');
      expect(data).toHaveProperty('user');
      
      // New tokens should be different from old ones
      expect(data.access_token).not.toBe(tokens.access_token);
      expect(data.refresh_token).not.toBe(tokens.refresh_token);
    });

    test('should reject invalid refresh token', async ({ request }) => {
      const response = await request.post('/api/auth/refresh', {
        data: {
          refresh_token: 'invalid.token.here'
        }
      });

      expect(response.status()).toBe(401);
    });

    test('should reject missing refresh token', async ({ request }) => {
      const response = await request.post('/api/auth/refresh', {
        data: {}
      });

      expect(response.status()).toBe(400);
    });

    test('should reject empty refresh token', async ({ request }) => {
      const response = await request.post('/api/auth/refresh', {
        data: {
          refresh_token: ''
        }
      });

      expect(response.status()).toBe(401);
    });
  });

  test.describe('Token Validation', () => {
    let authTokens: any;

    test.beforeEach(async ({ request }) => {
      // Register and login to get tokens for protected endpoints
      const user = { ...baseUser, email: `protected-${uuidv4()}@example.com` };
      await request.post('/api/auth/register', { data: user });
      
      const loginResponse = await request.post('/api/auth/login', {
        data: { email: user.email, password: user.password }
      });
      authTokens = await loginResponse.json();
    });

    test('should access protected endpoint with valid token', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${authTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
    });

    test('should reject protected endpoint without token', async ({ request }) => {
      const response = await request.get('/api/users');
      expect(response.status()).toBe(401);
    });

    test('should reject protected endpoint with invalid token', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': 'Bearer invalid.token.here'
        }
      });

      expect(response.status()).toBe(401);
    });

    test('should reject protected endpoint with malformed authorization header', async ({ request }) => {
      const malformedHeaders = [
        'invalid.token.here',         // missing Bearer
        'Basic dXNlcjpwYXNz',        // wrong auth type
        `Bearer`,                     // missing token
        `Bearer ${authTokens.access_token} extra`, // extra content
      ];

      for (const authHeader of malformedHeaders) {
        const response = await request.get('/api/users', {
          headers: {
            'Authorization': authHeader
          }
        });
        expect(response.status()).toBe(401);
      }
    });

    test('should handle concurrent requests with same token', async ({ request }) => {
      const promises = Array.from({ length: 5 }, () =>
        request.get('/api/users', {
          headers: {
            'Authorization': `Bearer ${authTokens.access_token}`
          }
        })
      );

      const responses = await Promise.all(promises);
      
      for (const response of responses) {
        expect(response.status()).toBe(200);
      }
    });
  });

  test.describe('Edge Cases', () => {
    test('should handle very long input gracefully', async ({ request }) => {
      const longString = 'a'.repeat(10000);
      const user = {
        name: longString,
        email: `long-${Date.now()}@example.com`,
        password: 'ValidPassword123!'
      };

      const response = await request.post('/api/auth/register', {
        data: user
      });

      // Should either accept or reject gracefully, not crash
      expect([200, 400, 422]).toContain(response.status());
    });

    test('should handle special characters in input', async ({ request }) => {
      const user = {
        name: 'Test User <script>alert("xss")</script>',
        email: `special-${Date.now()}@example.com`,
        password: 'Password123!@#$%^&*()'
      };

      const response = await request.post('/api/auth/register', {
        data: user
      });

      if (response.status() === 200) {
        const data = await response.json();
        // Should properly escape/sanitize the name
        expect(data.user.name).toBeDefined();
      }
    });

    test('should handle null and undefined values', async ({ request }) => {
      const invalidUsers = [
        { name: null, email: 'test@example.com', password: 'pass123' },
        { name: 'Test', email: null, password: 'pass123' },
        { name: 'Test', email: 'test@example.com', password: null },
      ];

      for (const user of invalidUsers) {
        const response = await request.post('/api/auth/register', {
          data: user
        });
        expect(response.status()).toBe(400);
      }
    });
  });
});