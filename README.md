# AI Commit CLI

[![Rust](https://github.com/yourusername/ai_commit_cli/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/ai_commit_cli/actions/workflows/rust.yml)

A Rust CLI tool that analyzes staged Git changes and automatically generates optimal commit messages using AI.

## Supported AI Platforms

- Claude (Anthropic)
- GPT-4 (OpenAI)
- Gemini (Google)

## Prerequisites

- Rust and Cargo installed
- Git installed
- API key for any of the supported AI platforms

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/ai_commit_cli.git
   cd ai_commit_cli
   ```

2. Set up your API key:

   - Using the interactive command:

     ```
     ai_commit_cli config --api
     ```

   - Or using environment variables:

     ```
     # For Claude
     export CLAUDE_API_KEY=your_anthropic_api_key_here

     # For OpenAI
     export OPENAI_API_KEY=your_openai_api_key_here

     # For Gemini
     export GEMINI_API_KEY=your_gemini_api_key_here
     ```

3. Build the tool:

   ```
   cargo build --release
   ```

4. Create a symbolic link to the executable (optional):
   ```
   ln -s $(pwd)/target/release/ai_commit_cli /usr/local/bin/
   ```

## Usage

### Configuration

```
# Set AI platform and API key (interactive)
ai_commit_cli config --api

# Set language for commit messages (interactive)
ai_commit_cli config --language

# Show current configuration
ai_commit_cli config --show
```

### Platform Selection

Using the config command, you can choose from the following AI platforms:

1. Claude (Anthropic)
2. GPT-4 (OpenAI)
3. Gemini (Google)

### Commit Message Language

You can set the language for commit messages with:

```
ai_commit_cli config --language
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
   ai_commit_cli
   ```

3. Automatically commit with the generated message:
   ```
   ai_commit_cli --commit
   ```
   or
   ```
   ai_commit_cli -c
   ```

## API Key Priority

API keys are loaded with the following priority:

1. Environment variables (depending on the platform: `CLAUDE_API_KEY`, `OPENAI_API_KEY`, or `GEMINI_API_KEY`)
2. API keys in the configuration file

## License

MIT
