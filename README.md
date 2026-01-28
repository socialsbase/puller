# Puller

A Rust CLI tool for pulling/archiving posts from social media platforms.

## Overview

Puller is a multi-platform content archiving CLI designed to pull and archive your existing posts from social platforms. It stores content as Markdown files with YAML frontmatter, compatible with the [publisher](https://github.com/socialsbase/publisher) tool for round-trip content management.

## Features

- Pull posts from social platforms into Markdown with YAML frontmatter
- Supported platforms: Dev.to
- State tracking to avoid re-pulling already archived content
- Dry-run mode for previewing without writing files
- Date filtering to pull only recent posts
- Force mode to re-pull existing articles

## Installation

### Prerequisites

- Rust 1.70+ and Cargo

### Build from source

```bash
cargo build --release
```

The binary will be available at `target/release/puller`.

## Usage

### List articles

Preview what articles are available without downloading:

```bash
puller list --platform devto
```

### Pull articles

Pull all articles to a directory:

```bash
puller pull --platform devto ./output
```

### Dry-run mode

Preview what would be pulled without writing files:

```bash
puller pull --platform devto ./output --dry-run
```

### Pull with date filter

Only pull articles published since a specific date:

```bash
puller pull --platform devto ./output --since 2024-01-01
```

### Force re-pull

Re-pull articles even if already archived:

```bash
puller pull --platform devto ./output --force
```

### Include drafts

Include unpublished draft articles:

```bash
puller pull --platform devto ./output --include-drafts
```

## Output Format

Pulled articles are saved as Markdown files with YAML frontmatter:

**Filename:** `{date}-{slug}.md` (e.g., `2024-03-15-building-cli-tools-in-rust.md`)

```markdown
---
title: "Building CLI Tools in Rust"
scheduled_at: 2024-03-15T10:00:00Z
status: publish
tags:
  - rust
  - cli
series: "Rust CLI Series"
canonical_url: https://mysite.com/original
# Pulled from: https://dev.to/user/article-abc123
# Platform ID: devto:12345
---

Article body in markdown...
```

## State Tracking

Puller maintains a `.puller-state.json` file in the output directory to track which articles have been pulled:

```json
{
  "pulled": {
    "devto:12345": {
      "local_path": "2024-03-15-building-cli-tools.md",
      "pulled_at": "2024-03-20T10:00:00Z"
    }
  }
}
```

This prevents re-downloading articles on subsequent runs unless `--force` is used.

## Configuration

### Environment Variables

Create a `.env` file or set environment variables:

#### Dev.to

Get your API key from https://dev.to/settings/extensions

```bash
DEVTO_API_KEY=your_api_key
```

## Project Structure

```
src/
├── main.rs           # CLI entry point and orchestration
├── error.rs          # Custom error types
├── config.rs         # Platform API configuration
├── platform.rs       # Platform enum definitions
├── article.rs        # Article struct and frontmatter generation
├── state.rs          # Pull state tracking
├── writer.rs         # Write articles to Markdown files
└── adapters/
    ├── mod.rs        # Puller trait definition
    └── devto.rs      # Dev.to API implementation
```

## License

MIT
