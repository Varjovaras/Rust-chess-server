# Chess Game - Full Stack Application

A real-time multiplayer chess game built with Rust, SvelteKit, and TailwindCSS. Features a custom chess engine, WebSocket-based multiplayer, and a modern responsive UI.

🎮 **Live Demo**: [https://chess-in-rust.vercel.app](https://chess-in-rust.vercel.app)

## 🎯 Features

- **Real-time Multiplayer**: Play chess with others through WebSocket connections
- **Custom Chess Engine**: Built from scratch in Rust with complete move validation
- **Modern UI**: Responsive design with drag-and-drop piece movement
- **Touch Support**: Mobile-friendly interface with touch controls
- **Animated Moves**: Smooth piece transitions and visual feedback
- **Legal Move Validation**: All chess rules enforced, including castling, en passant, and pawn promotion
- **Game State Management**: Persistent game state across all connected clients

## 🏗️ Architecture

This project is organized as a monorepo with three main components:

### Backend (`/backend`)
- **Framework**: Axum web framework
- **WebSocket Server**: Real-time bidirectional communication
- **Chess Engine Integration**: Handles move validation and game state
- **CORS Support**: Configured for cross-origin requests
- **Port Configuration**: Reads from `PORT` environment variable (default: 8000)

### Chess Engine (`/chess`)
- **Pure Rust**: Custom chess logic implementation
- **Complete Rules**: All standard chess rules including special moves
- **Move Validation**: Ensures only legal moves are allowed
- **State Serialization**: JSON serialization for client communication
- **Type Safety**: Leverages Rust's type system for correctness

### Frontend (`/frontend`)
- **Framework**: SvelteKit with TypeScript
- **Styling**: TailwindCSS for responsive design
- **Skeleton UI**: Component library for consistent design
- **Drag & Drop**: Intuitive piece movement interface
- **State Management**: Reactive updates via WebSocket messages

### Infrastructure (`/nginx`)
- **Reverse Proxy**: Nginx configuration for routing
- **Rate Limiting**: API and general request throttling
- **WebSocket Support**: Proper upgrade headers for WebSocket connections

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70+ ([Install Rust](https://rustup.rs/))
- **Bun**: Latest version ([Install Bun](https://bun.sh/))
- **Node.js**: 18+ (if not using Bun)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/chess.git
   cd chess
   ```

2. **Install frontend dependencies**
   ```bash
   cd frontend
   bun install
   ```

### Development

#### Run Backend Server
```bash
# From project root
bun run backend
# or
cargo run
```

The backend will start on `http://localhost:8000`

#### Run Frontend Development Server
```bash
# From project root
bun run dev
```

The frontend will start on `http://localhost:5173`

### Available Scripts

From the project root:

- `bun run backend` - Start Rust backend server
- `bun run dev` - Start frontend development server
- `bun run build` - Build frontend for production
- `bun run check` - Type-check frontend code
- `bun run check:watch` - Type-check in watch mode
- `bun run lint` - Lint frontend code with Biome
- `bun run biome` - Run Biome checks

## 📦 Project Structure

```
chess/
├── backend/           # Rust Axum backend
│   ├── src/
│   │   └── main.rs   # WebSocket server & API endpoints
│   └── Cargo.toml
├── chess/            # Chess engine library
│   ├── src/
│   │   ├── lib.rs
│   │   └── chessboard/
│   └── Cargo.toml
├── frontend/         # SvelteKit frontend
│   ├── src/
│   │   ├── routes/
│   │   ├── lib/
│   │   └── app.html
│   └── package.json
├── nginx/           # Nginx configuration
│   └── nginx.conf
├── Cargo.toml       # Workspace configuration
└── package.json     # Root package scripts
```

## 🔧 Configuration

### Environment Variables

**Backend:**
- `PORT` - Server port (default: 8000)

**Frontend:**
- Set WebSocket endpoint in your SvelteKit configuration

### CORS Configuration

The backend is configured to accept requests from any origin for development. For production, update the CORS settings in `backend/src/main.rs`:

```rust
let cors = CorsLayer::new()
    .allow_origin(Any)  // Change this for production
    .allow_methods(Any)
    .allow_headers(Any);
```

## 🎮 How to Play

1. Visit the application URL
2. The chess board will load with the starting position
3. Drag and drop pieces to make moves
4. All connected players see moves in real-time
5. Use the reset button to start a new game

## 🛠️ Technology Stack

### Backend
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Tower](https://github.com/tower-rs/tower)** - Middleware
- **[Serde](https://serde.rs/)** - Serialization

### Frontend
- **[SvelteKit](https://kit.svelte.dev/)** - Web framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type safety
- **[TailwindCSS](https://tailwindcss.com/)** - Styling
- **[Skeleton UI](https://www.skeleton.dev/)** - Component library
- **[Biome](https://biomejs.dev/)** - Linting and formatting

## 📡 WebSocket API

### Client → Server Messages

**Make a Move:**
```json
{
  "list_of_moves": [
    [["e", 2], ["e", 4], [0, 0]]
  ],
  "new_move": ["e", "e4", [0, 0]]
}
```

**Reset Game:**
```json
{
  "action": "reset"
}
```

### Server → Client Messages

**Initial State:**
```json
{
  "type": "initial_state",
  "chess": { /* game state */ }
}
```

**Game Update:**
```json
{
  "type": "update",
  "chess": { /* game state */ }
}
```

**Game Reset:**
```json
{
  "type": "reset",
  "chess": { /* game state */ }
}
```

## 🚢 Deployment

### Backend
The backend is a standard Rust application that can be deployed to any platform supporting Rust:
- Docker containers
- Cloud providers (AWS, GCP, Azure)
- Platform-as-a-Service (Render, Railway, Fly.io)

### Frontend
The frontend is deployed on **[Vercel](https://vercel.com/)** with automatic deployments from the main branch.

To deploy your own:
```bash
cd frontend
vercel deploy
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

- Chess rules reference: [Chess Programming Wiki](https://www.chessprogramming.org/)
- Inspiration from various open-source chess implementations
- Community feedback and contributions

## 📧 Contact

For questions or feedback, please open an issue on GitHub.

---

Built with ❤️ using Rust, SvelteKit, and TailwindCSS