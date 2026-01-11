# HyperSwap

SaaS sécurisé de partage de secrets avec chiffrement zero-knowledge.

## Architecture

- **Backend**: Rust avec Axum + SQLx
- **Frontend**: React + TypeScript + Vite
- **Base de données**: PostgreSQL
- **Stockage**: S3 compatible (MinIO)

## Fonctionnalités

- Partage de secrets texte avec chiffrement côté client
- Upload de fichiers sensibles
- Collecte de secrets (reverse sharing)
- Authentification OAuth2 (Google, Microsoft, GitHub)
- Gestion d'organisations
- Expiration automatique et limites de vues
- Destruction après lecture

## Développement

### Prérequis

- Rust (latest stable)
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 15+
- MinIO (ou S3 compatible)

### Setup Backend

```bash
cd backend
cp .env.example .env
# Éditer .env avec vos configurations

# Démarrer PostgreSQL et MinIO
docker-compose up -d

# Installer les dépendances et compiler
cargo build

# Lancer les migrations
cargo sqlx migrate run

# Démarrer le serveur
cargo run
```

### Setup Frontend

```bash
cd frontend
npm install
npm run dev
```

## Configuration

Voir `backend/.env.example` pour la liste complète des variables d'environnement.

## Sécurité

- Chiffrement AES-GCM-256 côté client
- Clés de chiffrement jamais transmises au serveur
- Validation du format chiffré côté serveur
- Rate limiting et protection anti-brute force
- Audit logging

## Roadmap

Voir le plan d'architecture pour la roadmap complète de développement.
