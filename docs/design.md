# Design Document for Media Duplicate Scanner

## Overview

This document outlines the design for a desktop tool to scan for duplicate files
across a large media collection. The tool will be developed using Rust,
leveraging the Tauri framework for desktop application capabilities and the
Leptos framework for the front-end. The application will be modular, with each
module running in parallel on its own thread to optimize performance.

## Architecture

### Modules Overview

- Orchestrator Module: Manages the workflow, coordinating between modules.
- Config Module: Manages configuration settings such as file patterns to exclude
  from scanning.
- Scanning Module: Iterates over files, collects metadata, and filters files
  considered for ingestion based on patterns from the Config Module.
- Ingest Module: Reads files, in chunks, into a shared in-memory buffer.
- Fingerprint Module: Calculates hashes for files, using perceptual hashing for
  images and binary hashing for all media.
- Persist Module: Journals metadata to a SQLite database.
- Front-End Module: Provides the user interface, real-time updates and instructs
  the orchestrator of user interactions

### Concurrency Model

- Each module does work via its own thread pool.
- Use Rust's concurrency primitives (e.g., channels, mutexes) to manage
  communication and data sharing between threads.

## Module Details

### Orchestrator Module

Responsibilities:
- Coordinate operations between modules.
- Manage the workflow and data flow.

Implementation:
- Use Rust's channels for communication between modules.
- Implement a task queue for managing fingerprinting tasks.

### Config Module

Responsibilities:
- Store and manage configuration settings.
- Provide file patterns for exclusion.

Implementation:
- Use a configuration file (e.g., JSON, TOML) for settings.
- Provide an API for other modules to access configuration data.

### Scanning Module

Responsibilities:
- Traverse external hard drives.
- Ignore files based on patterns from the Config Module.
- Collect data about files on the drive, such as file paths and sizes.

Implementation:
- Spawn a new thread for each external drive.
- Use Rust's std::fs for file system operations.
- Implement filtering logic using patterns from the Config Module.

### Ingest Module

Responsibilities:
- Spawn a thread for each CPU core.
- Read files in configurable chunk sizes.
- Populate a shared in-memory buffer.

Implementation:
- Use Rust's std::io for file reading.
- Implement a buffer using `Vec<u8>` or similar.

### Fingerprint Module

Responsibilities:
- Calculate fast xxh3 hashes for all files.
- Use appropriate perceptual hashing for rellevant file types such as images.

Implementation:
- Use libraries like image for perceptual hashing.
- Use xxhash-rust for binary hashing.

### Persist Module

Responsibilities:
- Store metadata in a SQLite database.
- Ensure data integrity and consistency.

Implementation:
- Use rusqlite for database operations.
- Define a schema for storing file path, binary hash, perceptual hash, and file
  size.

### Front-End Module

Framework: Leptos

- UI Components:
- Dashboard for displaying scan progress and results.
- Configuration panel for setting file patterns and other settings.
- Results view for displaying duplicate files.

## Error Handling and Logging

Implement robust error handling in each module.

- Use logging to track application events and errors.

## Testing

- Unit tests for each module.
- Integration tests for module interactions.
- Performance tests to ensure scalability with large media collections.

## Deployment

- Package the application using Tauri's bundling capabilities.
- Ensure cross-platform compatibility (Windows, macOS, Linux).
