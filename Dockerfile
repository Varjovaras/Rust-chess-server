# Build backend stage
FROM rust:1.83-slim AS backend-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY chess ./chess
COPY backend ./backend

# Build the application in release mode
RUN cargo build --release --bin backend

# Build frontend stage
FROM oven/bun:1 AS frontend-builder

WORKDIR /app

# Copy package files
COPY frontend/package.json frontend/bun.lockb ./

# Install dependencies
RUN bun install --frozen-lockfile

# Copy source files
COPY frontend/ .

# Build the application
RUN bun run build

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    unzip \
    nginx \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

WORKDIR /app

# Copy backend binary from builder
COPY --from=backend-builder /app/target/release/backend /app/backend

# Copy frontend build and dependencies from builder
COPY --from=frontend-builder /app/build /app/frontend/build
COPY --from=frontend-builder /app/package.json /app/frontend/package.json
COPY --from=frontend-builder /app/bun.lockb /app/frontend/bun.lockb
COPY --from=frontend-builder /app/node_modules /app/frontend/node_modules

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Create supervisord configuration
RUN echo '[supervisord]\n\
    nodaemon=true\n\
    user=root\n\
    \n\
    [program:backend]\n\
    command=/app/backend\n\
    environment=PORT=8000\n\
    stdout_logfile=/dev/stdout\n\
    stdout_logfile_maxbytes=0\n\
    stderr_logfile=/dev/stderr\n\
    stderr_logfile_maxbytes=0\n\
    autorestart=true\n\
    \n\
    [program:frontend]\n\
    command=bun run build/index.js\n\
    directory=/app/frontend\n\
    environment=NODE_ENV=production,PORT=3001\n\
    stdout_logfile=/dev/stdout\n\
    stdout_logfile_maxbytes=0\n\
    stderr_logfile=/dev/stderr\n\
    stderr_logfile_maxbytes=0\n\
    autorestart=true\n\
    \n\
    [program:nginx]\n\
    command=/usr/sbin/nginx -g "daemon off;"\n\
    stdout_logfile=/dev/stdout\n\
    stdout_logfile_maxbytes=0\n\
    stderr_logfile=/dev/stderr\n\
    stderr_logfile_maxbytes=0\n\
    autorestart=true\n\
    ' > /etc/supervisor/conf.d/supervisord.conf

# Expose only the nginx port
EXPOSE 3000

# Set environment variables
ENV NODE_ENV=production

# Run supervisord to manage all services
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/supervisord.conf"]
