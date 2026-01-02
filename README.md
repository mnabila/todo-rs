# Todo-rs

[![Build Status](https://github.com/mnabila/todo-rs/actions/workflows/build.yml/badge.svg)](https://github.com/mnabila/todo-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Todo application written in Rust**.  
This project is my **first project using the Rust programming language**, 
created to learn Rust fundamentals, async programming, and containerized development using Docker.

---

## Features

- REST API for managing todos
- Written in Rust
- Containerized using Docker
- Easy to run with Docker Compose

---

## Tech Stack

- Rust
- Docker & Docker Compose
- Postgresql
- Axum Framework
- Sqlx Library

---

## Prerequisites

Make sure you have installed:

- Docker
- Docker Compose

Check installation:

```bash
docker --version
docker compose version
```

## Running the Project with Docker Compose

1. Clone the Repository

```bash
git clone https://github.com/mnabila/todo-rs.git
cd todo-rs
```

2. Build and Run Containers

```bash
docker compose up --build
```

3. Access the Application
   Once running, the application will be available at:

```bash
http://localhost:3000
```

## Stopping the Application

```bash
docker compose down
```

## Purpose of This Project

This project was created to:
- Learn Rust language basics
- Understand ownership, borrowing, and lifetimes
- Practice building a backend service using rust
- Learn Docker & Docker Compose for Rust applications
