# Sentinai
Autonomous CI/CD & Self-Healing DevOps Agent Backend

## Overview
Sentinai is a production-grade backend for an autonomous DevOps automation platform. It is designed to connect GitHub repositories, store project metadata, dynamically generate CI configurations, and track security findings and CI builds.

## Setup Instructions

### Prerequisites
- Rust (latest stable)
- PostgreSQL (or a Neon Serverless Postgres instance)
- Next.js (Typescript) + Tailwind

## API Documentation:
- Check out [API Documentation](./API_REFERENCE.md).

## Deployment Guide:
1. Create a new **Web Service** on Render connected to your GitHub repository.
2. **Build Command**: `cargo build --release`
3. **Start Command**: `cargo run --release` (or `./target/release/sentinai-backend`)
4. **Environment Variables**:
   - Add `DATABASE_URL` (your Neon connection string).
   - Add `JWT_SECRET` (generate a strong random string).
   - Add `PORT` (Render uses automatically, but you can specify `10000`).