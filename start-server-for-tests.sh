#!/bin/bash
set -e

# Kill any existing server on port 8080
lsof -ti:8080 | xargs kill -9 2>/dev/null || true
sleep 2

# Start databases if not running
docker compose -f docker-compose.test.yml up -d

# Wait for databases to be ready
echo "⏳ Waiting for databases to be ready..."
sleep 10

# Check database health
max_attempts=30
attempt=1
while [ $attempt -le $max_attempts ]; do
    if docker compose -f docker-compose.test.yml ps | grep -q "healthy"; then
        echo "✅ All databases are healthy"
        break
    fi
    echo "⏳ Attempt $attempt/$max_attempts - waiting for databases..."
    sleep 2
    attempt=$((attempt + 1))
done

if [ $attempt -gt $max_attempts ]; then
    echo "❌ Databases failed to become healthy"
    exit 1
fi

# Run migrations
DATABASE_URL=postgres://app:secure-password-123@localhost:5432/app cargo sqlx migrate run

# Start the server
export DATABASE_URL=postgres://app:secure-password-123@localhost:5432/app
export REDIS_URL=redis://localhost:6379
export MONGODB_URL=mongodb://admin:password123@localhost:27017
export JWT_SECRET=test-secret-key

exec cargo run