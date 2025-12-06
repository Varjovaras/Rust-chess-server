# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package.json bun.lockb* package-lock.json* yarn.lock* pnpm-lock.yaml* ./

# Install dependencies
RUN npm ci || npm install

# Copy source files
COPY . .

# Build the application
RUN npm run build

# Runtime stage
FROM node:20-alpine

WORKDIR /app

# Copy package files
COPY package.json bun.lockb* package-lock.json* yarn.lock* pnpm-lock.yaml* ./

# Install production dependencies only
RUN npm ci --omit=dev || npm install --omit=dev

# Copy built application from builder
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./

# Expose the port
EXPOSE 3000

# Set environment variables
ENV NODE_ENV=production
ENV PORT=3000

# Run the application
CMD ["node", "build"]
