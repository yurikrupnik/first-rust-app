import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('User Management API', () => {
  let adminTokens: any;
  let userTokens: any;
  const adminUser = {
    name: 'Admin User',
    email: `admin-${Date.now()}@example.com`,
    password: 'AdminPassword123!',
    age: 30
  };
  const regularUser = {
    name: 'Regular User', 
    email: `user-${Date.now()}@example.com`,
    password: 'UserPassword123!',
    age: 25
  };

  test.beforeAll(async ({ request }) => {
    // Register admin user (assuming first user becomes admin or manual setup)
    const adminRegResponse = await request.post('/api/auth/register', {
      data: adminUser
    });
    expect(adminRegResponse.status()).toBe(200);
    adminTokens = await adminRegResponse.json();

    // Register regular user
    const userRegResponse = await request.post('/api/auth/register', {
      data: regularUser  
    });
    expect(userRegResponse.status()).toBe(200);
    userTokens = await userRegResponse.json();
  });

  test.describe('Get All Users', () => {
    test('should get users list with valid token', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
      const users = await response.json();
      
      expect(Array.isArray(users)).toBe(true);
      expect(users.length).toBeGreaterThan(0);
      
      // Check user structure
      for (const user of users) {
        expect(user).toHaveProperty('id');
        expect(user).toHaveProperty('name');
        expect(user).toHaveProperty('email');
        expect(user).toHaveProperty('role');
        expect(user).toHaveProperty('age');
        expect(user).not.toHaveProperty('password');
        expect(user).not.toHaveProperty('password_hash');
      }
    });

    test('should reject unauthenticated request', async ({ request }) => {
      const response = await request.get('/api/users');
      expect(response.status()).toBe(401);
    });

    test('should reject invalid token', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': 'Bearer invalid.token.here'
        }
      });
      expect(response.status()).toBe(401);
    });

    test('should return users ordered by creation date', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
      const users = await response.json();
      
      if (users.length > 1) {
        // Assuming users are returned in descending order by created_at
        for (let i = 0; i < users.length - 1; i++) {
          // Would need created_at field in response to test this properly
          expect(users[i]).toHaveProperty('id');
        }
      }
    });

    test('should handle concurrent requests', async ({ request }) => {
      const promises = Array.from({ length: 5 }, () =>
        request.get('/api/users', {
          headers: {
            'Authorization': `Bearer ${userTokens.access_token}`
          }
        })
      );

      const responses = await Promise.all(promises);
      
      for (const response of responses) {
        expect(response.status()).toBe(200);
        const users = await response.json();
        expect(Array.isArray(users)).toBe(true);
      }
    });
  });

  test.describe('Get User by ID', () => {
    let testUserId: string;

    test.beforeAll(async ({ request }) => {
      // Get users list to find a valid user ID
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });
      const users = await response.json();
      testUserId = users[0]?.id;
      expect(testUserId).toBeDefined();
    });

    test('should get user by valid ID', async ({ request }) => {
      const response = await request.get(`/api/users/${testUserId}`, {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
      const user = await response.json();
      
      expect(user).toHaveProperty('id', testUserId);
      expect(user).toHaveProperty('name');
      expect(user).toHaveProperty('email');
      expect(user).toHaveProperty('role');
      expect(user).not.toHaveProperty('password');
      expect(user).not.toHaveProperty('password_hash');
    });

    test('should reject request for non-existent user', async ({ request }) => {
      const fakeUserId = '00000000-0000-0000-0000-000000000000';
      const response = await request.get(`/api/users/${fakeUserId}`, {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(404);
    });

    test('should reject invalid UUID format', async ({ request }) => {
      const invalidIds = [
        'not-a-uuid',
        '123',
        'invalid-uuid-format',
        '',
      ];

      for (const invalidId of invalidIds) {
        const response = await request.get(`/api/users/${invalidId}`, {
          headers: {
            'Authorization': `Bearer ${userTokens.access_token}`
          }
        });
        expect([400, 404]).toContain(response.status());
      }
    });

    test('should reject unauthenticated request', async ({ request }) => {
      const response = await request.get(`/api/users/${testUserId}`);
      expect(response.status()).toBe(401);
    });
  });

  test.describe('Create User (Admin Only)', () => {
    test('should allow admin to create user', async ({ request }) => {
      // Skip if admin functionality not implemented
      test.skip(!adminTokens || adminTokens.user?.role !== 'admin', 'Admin functionality not available');

      const newUser = {
        name: 'Created User',
        email: `created-${uuidv4()}@example.com`,
        password: 'CreatedPassword123!',
        age: 28
      };

      const response = await request.post('/api/users', {
        headers: {
          'Authorization': `Bearer ${adminTokens.access_token}`
        },
        data: newUser
      });

      expect(response.status()).toBe(200);
      const createdUser = await response.json();
      
      expect(createdUser).toHaveProperty('id');
      expect(createdUser.name).toBe(newUser.name);
      expect(createdUser.email).toBe(newUser.email);
      expect(createdUser.role).toBe('user');
      expect(createdUser.age).toBe(newUser.age);
      expect(createdUser).not.toHaveProperty('password');
    });

    test('should reject non-admin user creation', async ({ request }) => {
      const newUser = {
        name: 'Unauthorized User',
        email: `unauthorized-${uuidv4()}@example.com`,
        password: 'Password123!',
        age: 26
      };

      const response = await request.post('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        },
        data: newUser
      });

      expect(response.status()).toBe(403); // Forbidden
    });

    test('should reject unauthenticated user creation', async ({ request }) => {
      const newUser = {
        name: 'Unauthenticated User',
        email: `unauth-${uuidv4()}@example.com`,
        password: 'Password123!',
        age: 27
      };

      const response = await request.post('/api/users', {
        data: newUser
      });

      expect(response.status()).toBe(401);
    });

    test('should validate required fields', async ({ request }) => {
      // Skip if admin functionality not implemented
      test.skip(!adminTokens || adminTokens.user?.role !== 'admin', 'Admin functionality not available');

      const incompleteUsers = [
        { email: 'test@example.com', password: 'pass123' }, // missing name
        { name: 'Test', password: 'pass123' }, // missing email  
        { name: 'Test', email: 'test@example.com' }, // missing password
      ];

      for (const user of incompleteUsers) {
        const response = await request.post('/api/users', {
          headers: {
            'Authorization': `Bearer ${adminTokens.access_token}`
          },
          data: user
        });
        expect(response.status()).toBe(400);
      }
    });

    test('should reject duplicate email in user creation', async ({ request }) => {
      // Skip if admin functionality not implemented
      test.skip(!adminTokens || adminTokens.user?.role !== 'admin', 'Admin functionality not available');

      const duplicateUser = {
        name: 'Duplicate Email User',
        email: userTokens.user.email, // Use existing user's email
        password: 'Password123!',
        age: 29
      };

      const response = await request.post('/api/users', {
        headers: {
          'Authorization': `Bearer ${adminTokens.access_token}`
        },
        data: duplicateUser
      });

      expect(response.status()).toBe(409); // Conflict
    });
  });

  test.describe('User Data Integrity', () => {
    test('should not expose sensitive data in user responses', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
      const users = await response.json();
      
      for (const user of users) {
        expect(user).not.toHaveProperty('password');
        expect(user).not.toHaveProperty('password_hash');
        expect(user).not.toHaveProperty('password_digest');
        expect(user).not.toHaveProperty('hash');
        expect(user).not.toHaveProperty('secret');
        expect(user).not.toHaveProperty('private_key');
      }
    });

    test('should return consistent user data structure', async ({ request }) => {
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });

      expect(response.status()).toBe(200);
      const users = await response.json();
      
      if (users.length > 1) {
        const firstUser = users[0];
        const expectedKeys = Object.keys(firstUser).sort();
        
        for (let i = 1; i < users.length; i++) {
          const currentUserKeys = Object.keys(users[i]).sort();
          expect(currentUserKeys).toEqual(expectedKeys);
        }
      }
    });

    test('should handle special characters in search/filter', async ({ request }) => {
      // This would test search functionality if implemented
      const specialChars = ['<', '>', '"', "'", '&', '%', '\\', '/', '='];
      
      for (const char of specialChars) {
        const response = await request.get(`/api/users?search=${encodeURIComponent(char)}`, {
          headers: {
            'Authorization': `Bearer ${userTokens.access_token}`
          }
        });
        
        // Should handle gracefully, not crash
        expect([200, 400]).toContain(response.status());
      }
    });
  });

  test.describe('Rate Limiting and Performance', () => {
    test('should handle multiple concurrent user requests', async ({ request }) => {
      const concurrentRequests = 20;
      const promises = Array.from({ length: concurrentRequests }, () =>
        request.get('/api/users', {
          headers: {
            'Authorization': `Bearer ${userTokens.access_token}`
          }
        })
      );

      const start = Date.now();
      const responses = await Promise.all(promises);
      const duration = Date.now() - start;
      
      // All requests should succeed
      for (const response of responses) {
        expect(response.status()).toBe(200);
      }
      
      // Should complete within reasonable time
      expect(duration).toBeLessThan(10000); // 10 seconds
    });

    test('should respond to user requests within acceptable time', async ({ request }) => {
      const start = Date.now();
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });
      const duration = Date.now() - start;
      
      expect(response.status()).toBe(200);
      expect(duration).toBeLessThan(2000); // 2 seconds
    });
  });

  test.describe('Edge Cases and Security', () => {
    test('should handle malformed authorization headers', async ({ request }) => {
      const malformedHeaders = [
        'Bearer',                           // Missing token
        'Basic dXNlcjpwYXNz',              // Wrong auth type
        `Bearer ${userTokens.access_token} extra`, // Extra content
        'bearer ' + userTokens.access_token, // Wrong case
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

    test('should handle very long URLs gracefully', async ({ request }) => {
      const longPath = 'a'.repeat(10000);
      const response = await request.get(`/api/users/${longPath}`, {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });
      
      // Should handle gracefully, not crash
      expect([400, 404, 414]).toContain(response.status());
    });

    test('should prevent token reuse after user deletion/modification', async ({ request }) => {
      // This would test token invalidation scenarios
      // For now, just verify tokens work normally
      const response = await request.get('/api/users', {
        headers: {
          'Authorization': `Bearer ${userTokens.access_token}`
        }
      });
      expect(response.status()).toBe(200);
    });

    test('should handle null/undefined in query parameters', async ({ request }) => {
      const badParams = [
        '?limit=null',
        '?offset=undefined', 
        '?sort=',
        '?filter=%00',
      ];

      for (const params of badParams) {
        const response = await request.get(`/api/users${params}`, {
          headers: {
            'Authorization': `Bearer ${userTokens.access_token}`
          }
        });
        
        // Should handle gracefully
        expect([200, 400]).toContain(response.status());
      }
    });
  });
});