# AI Commit CLI

A [Rust](https://www.rust-lang.org/) CLI tool that analyzes staged Git changes and automatically generates optimal commit messages using AI.

[![Changelog](https://img.shields.io/badge/changelog-v0.0.2-green.svg)](https://github.com/hiraishikentaro/ai_commit_cli/blob/master/CHANGELOG.md)

## Supported AI Platforms

- Claude (Anthropic)
- GPT-4 (OpenAI)
- Gemini (Google)

## Prerequisites

- Git installed
- API key for any of the supported AI platforms

## Installation

### Using Homebrew (macOS)

1. Add the tap and install:

   ```bash
   brew tap hiraishikentaro/ai_commit_cli
   brew install ai_commit_cli
   ```

### From Source

Requires Rust and Cargo installed.

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/ai_commit_cli.git
   cd ai_commit_cli
   ```

2. Install using Cargo:

   ```bash
   cargo install --path .
   ```

   This will install the `aic` command to your system.

3. Set up your API key:

   - Using the interactive command:

     ```bash
     aic config --api
     ```

   - Or using environment variables:

     ```bash
     # For Claude
     export CLAUDE_API_KEY=your_anthropic_api_key_here

     # For OpenAI
     export OPENAI_API_KEY=your_openai_api_key_here

     # For Gemini
     export GEMINI_API_KEY=your_gemini_api_key_here
     ```

## Usage

### Configuration

```
# Set AI platform and API key (interactive)
aic config --api

# Set language for commit messages (interactive)
aic config --language

# Show current configuration
aic config --show
```

### Platform Selection

Using the config command, you can choose from the following AI platforms:

1. Claude (Anthropic)
2. GPT-4 (OpenAI)
3. Gemini (Google)

### Commit Message Language

You can set the language for commit messages with:

```
aic config --language
```

Available language options:

- JA: Japanese (default)
- EN: English
- CN: Chinese

### Generating Commit Messages

1. Add your changes to the staging area:

   ```
   git add .
   ```

2. Generate a commit message:

   ```
   aic
   ```

3. Automatically commit with the generated message:
   ```
   aic --commit
   ```
   or
   ```
   aic -c
   ```

## API Key Priority

API keys are loaded with the following priority:

1. Environment variables (depending on the platform: `CLAUDE_API_KEY`, `OPENAI_API_KEY`, or `GEMINI_API_KEY`)
2. API keys in the configuration file

## License

MIT
