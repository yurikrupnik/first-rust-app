import { FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  console.log('🚀 Setting up e2e test environment');
  
  // Wait for server to be ready
  const baseURL = config.use?.baseURL || 'http://localhost:8080';
  
  // You could set up test data here if needed
  console.log(`✅ Server ready at ${baseURL}`);
}

export default globalSetup;