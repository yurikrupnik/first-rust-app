import { test, expect } from '@playwright/test';

test.describe('Health API', () => {
  test('should return healthy status', async ({ request }) => {
    const response = await request.get('/api/health');
    
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('application/json');
    
    const data = await response.json();
    expect(data).toHaveProperty('status', 'ok');
    expect(data).toHaveProperty('service', 'first-rust-app');
    expect(data).toHaveProperty('timestamp');
    
    // Timestamp should be a valid ISO string
    expect(new Date(data.timestamp).toISOString()).toBe(data.timestamp);
  });

  test('should return consistent structure across calls', async ({ request }) => {
    const responses = await Promise.all([
      request.get('/api/health'),
      request.get('/api/health'),
      request.get('/api/health'),
    ]);

    for (const response of responses) {
      expect(response.status()).toBe(200);
      const data = await response.json();
      expect(data.status).toBe('ok');
      expect(data.service).toBe('first-rust-app');
      expect(data).toHaveProperty('timestamp');
    }
  });

  test('should handle concurrent requests', async ({ request }) => {
    const concurrentRequests = 10;
    const promises = Array.from({ length: concurrentRequests }, () =>
      request.get('/api/health')
    );

    const responses = await Promise.all(promises);
    
    for (const response of responses) {
      expect(response.status()).toBe(200);
      const data = await response.json();
      expect(data.status).toBe('ok');
      expect(data.service).toBe('first-rust-app');
    }
  });

  test('should return different timestamps for sequential calls', async ({ request }) => {
    const response1 = await request.get('/api/health');
    await new Promise(resolve => setTimeout(resolve, 10)); // Small delay
    const response2 = await request.get('/api/health');
    
    const data1 = await response1.json();
    const data2 = await response2.json();
    
    expect(data1.timestamp).not.toBe(data2.timestamp);
  });

  test('should include CORS headers', async ({ request }) => {
    const response = await request.get('/api/health', {
      headers: {
        'Origin': 'https://example.com'
      }
    });
    
    expect(response.status()).toBe(200);
    // Should include CORS headers due to CorsLayer::permissive()
    expect(response.headers()).toHaveProperty('access-control-allow-origin');
  });

  test('should reject non-GET methods', async ({ request }) => {
    const methods = ['POST', 'PUT', 'DELETE', 'PATCH'];
    
    for (const method of methods) {
      const response = await request.fetch('/api/health', { method });
      expect(response.status()).toBe(405); // Method Not Allowed
    }
  });

  test('should handle malformed paths gracefully', async ({ request }) => {
    const paths = [
      '/api/health/',     // trailing slash
      '/api//health',     // double slash
      '/API/HEALTH',      // wrong case
    ];
    
    for (const path of paths) {
      const response = await request.get(path);
      // Should either work (200) or return 404, but not crash
      expect([200, 404]).toContain(response.status());
    }
  });

  test('should respond quickly', async ({ request }) => {
    const start = Date.now();
    const response = await request.get('/api/health');
    const duration = Date.now() - start;
    
    expect(response.status()).toBe(200);
    expect(duration).toBeLessThan(1000); // Should respond within 1 second
  });
});