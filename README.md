<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/React-18.0+-61DAFB?style=for-the-badge&logo=react&logoColor=black" alt="React" />
  <img src="https://img.shields.io/badge/TypeScript-5.0+-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript" />
  <img src="https://img.shields.io/badge/Storybook-7.0+-FF4785?style=for-the-badge&logo=storybook&logoColor=white" alt="Storybook" />
  <img src="https://img.shields.io/badge/Docker-Ready-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker" />
  <img src="https://img.shields.io/badge/Monorepo-NPM_Workspaces-CB3837?style=for-the-badge&logo=npm&logoColor=white" alt="npm workspaces" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/badge/PRs-Welcome-brightgreen?style=for-the-badge" alt="PRs Welcome" />
</p>

<br />

<div align="center">
  <h1>⚡ Enzin</h1>
  <p><strong>Blazing fast full-text search engine.</strong> Built with Rust. Served with React.</p>
  <p><i>Fuzzy matching. Pagination. Rich indexing. All wrapped in a production-grade monorepo.</i></p>
</div>

---

## ✨ What is Enzin?

Enzin is a **high-performance search engine** designed for developers who need fast retrieval without the bloat of Elasticsearch.

It's a fully functional, end-to-end search solution that includes:

- 🦀 **A Rust core** – Memory-safe, thread-safe, and compiled to a lean binary.
- ⚛️ **A React component library** – Drop-in UI components (`<SearchBox />`, `<SearchResults />`, `<Paginator />`) with zero configuration.
- 🎨 **An isolated Storybook** – Test and prototype your search UI without spinning up the backend.
- 🐳 **Dockerized deployment** – One command and you're serving search results in seconds.
- 🧪 **Tested & structured** – Rust unit tests, Vitest component tests, and an npm workspaces monorepo that scales.

---

## 🧠 Tech Stack

| Layer | Technology |
|-------|------------|
| **Search Engine** | Rust (Tokio + Tantivy) |
| **API Layer** | Axum |
| **UI Framework** | React 18 + TypeScript |
| **Component Dev** | Storybook 7 |
| **Package Management** | npm workspaces |
| **Containerization** | Docker & Docker Compose |

---

## 🚀 Quick Start (Get it running in 2 minutes)

### Prerequisites
- Rust (1.70+)
- Node.js (18+) & npm
- Docker (optional)

### 1. Clone & Install
```bash
git clone https://github.com/emalinegayhart/enzin.git
cd enzin
npm install
```

### 2. Spin up the search engine
```bash
cd packages/api
cargo run --release
```

### 3. Fire up the UI playground
```bash
cd packages/react
npm run storybook
```

### 4. Or just use Docker
From the repo root:
```bash
npm run docker:up
```

That's it. The API runs on `http://localhost:7700` and Storybook on `http://localhost:6006`.

---

## 📁 Architecture Overview

```
enzin/
├── packages/
│   ├── api/                  # Rust search engine + HTTP server
│   │   ├── src/engine/       # Indexing, fuzzy matching, pagination
│   │   ├── src/api/          # Axum routes & handlers
│   │   ├── Dockerfile
│   │   └── docker-compose.yml
│   └── react/                # React component library
│       ├── src/hooks/        # useEnzinSearch, usePagination, useDebounce
│       ├── src/components/   # SearchBox, SearchResults, Paginator, Search
│       ├── src/types/        # Shared TypeScript types
│       └── .storybook/       # Isolated Storybook setup
└── package.json              # npm workspaces root
```

---

## 🔌 API Reference

<p align="center">
  <b>Base URL:</b> <code>http://localhost:7700</code> &nbsp;·&nbsp; <b>Env override:</b> <code>ENZIN_PORT</code>
</p>

### Endpoints

| Method | Endpoint | Description |
|:------:|----------|-------------|
| <kbd>POST</kbd> | `/indexes` | Create an index |
| <kbd>GET</kbd> | `/indexes` | List indexes |
| <kbd>DELETE</kbd> | `/indexes/:name` | Delete an index |
| <kbd>POST</kbd> | `/indexes/:name/documents` | Add documents (single or batch) |
| <kbd>GET</kbd> | `/indexes/:name/search` | Search documents |

> 💡 All examples below use `products` as the index name.

### Examples

**Create an index**

```bash
curl -X POST http://localhost:7700/indexes \
  -H "Content-Type: application/json" \
  -d '{"name":"products"}'
```

**List indexes**

```bash
curl http://localhost:7700/indexes
```

**Delete an index**

```bash
curl -X DELETE http://localhost:7700/indexes/products
```

**Add documents**

Single:

```bash
curl -X POST http://localhost:7700/indexes/products/documents \
  -H "Content-Type: application/json" \
  -d '{"id":"1","title":"red shoes","price":50}'
```

Batch:

```bash
curl -X POST http://localhost:7700/indexes/products/documents \
  -H "Content-Type: application/json" \
  -d '[{"id":"1","title":"red shoes"},{"id":"2","title":"blue shirt"}]'
```

**Search documents**

```bash
curl "http://localhost:7700/indexes/products/search?q=red+shoes"
```

Fuzzy matching:

```bash
curl "http://localhost:7700/indexes/products/search?q=red+shoes&fuzzy=true"
```

Pagination:

```bash
curl "http://localhost:7700/indexes/products/search?q=red&limit=10&offset=0"
```

---

## 🚀 Future Capabilities (Coming Soon)

Enzin is built for the **AI era**. The foundation is solid—now it's time to extend it:

- **🧠 Dense Vector Search** – Integrating semantic similarity via `candle` or `cuVS`. Move beyond keywords to understand *meaning*.
- **⚡ GPU Acceleration** – Currently CPU-bound. Exploring NVIDIA CUDA & Apple Metal backends (`hive-gpu`) for 10x faster indexing and querying.
- **📦 Embedding Pipeline** – Plug in OpenAI, Cohere, or local HuggingFace models to generate vectors on the fly.

---

## 🤝 Contributing

This is **open source with open arms**.

- Found a bug? Open an issue.
- Have a wild idea? Fork it and submit a PR.
- Want to help build the vector/GPU roadmap? Jump in—I'd love the collaboration.

**Guidelines:**
1. Fork the repo.
2. Create your feature branch (`git checkout -b feature/amazing-add`).
3. Commit your changes (`git commit -m 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-add`).
5. Open a Pull Request.

---

## 📄 License

Distributed under the **MIT License**. See `LICENSE` for more information.
