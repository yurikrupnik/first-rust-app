import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up e2e test environment');
  
  // Clean up any test data or resources
  console.log('✅ Cleanup completed');
}

export default globalTeardown;