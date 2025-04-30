# AI Commit CLI

A [Rust](https://www.rust-lang.org/) CLI tool that analyzes staged Git changes and automatically generates optimal commit messages using AI.

[![Actions Status](https://github.com/dalance/procs/workflows/Regression/badge.svg)](https://github.com/hiraishikentaro/ai_commit_cli/actions)
[![Changelog](https://img.shields.io/badge/changelog-v0.0.3-green.svg)](https://github.com/hiraishikentaro/ai_commit_cli/blob/master/CHANGELOG.md)

## Supported AI Platforms

- Claude (Anthropic)
- GPT-4 (OpenAI)
- Gemini (Google)

## Available Models

Each AI platform offers multiple models with different capabilities and costs:

### Claude (Anthropic)

- Claude 3.7 Sonnet
- Claude 3.5 Sonnet
- Claude 3.5 Haiku

### OpenAI

- o4-mini
- GPT-4.1-mini
- o3-mini
- GPT-4o-mini

### Gemini

- Gemini 2.0 Flash Lite
- Gemini 2.0 Flash
- Gemini 1.5 Pro

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

# Set custom prompt for commit messages (interactive)
aic config --prompt

# Show current configuration
aic config --show
```

### Platform Selection

Using the config command, you can choose from the following AI platforms:

1. Claude (Anthropic)
2. GPT-4 (OpenAI)
3. Gemini (Google)

When selecting a platform, you'll also be prompted to choose a specific model from that platform. Each platform offers different models with varying capabilities and performance.

### Model Selection

During configuration (`aic config --api`), after selecting the AI platform, you'll be prompted to choose a specific model for that platform.

Different models offer trade-offs between:

- Quality of generated commit messages
- Speed of response
- Cost (API usage)

You can change your model selection at any time by running the configuration again:

```
aic config --api
```

Your model selection will be saved in the configuration file and used for all future commit message generations.

### Commit Message Language

You can set the language for commit messages with:

```
aic config --language
```

Available language options:

- JA: Japanese (default)
- EN: English
- CN: Chinese

### Custom Prompts

You can set your own custom system prompt to control how AI generates commit messages:

```
aic config --prompt
```

This will open a simple vim-like editor where you can write and edit your custom prompt with multi-line support. Key controls:

- Use arrow keys to navigate
- Type to insert text
- Press `Ctrl+S` to save changes
- Press `Esc` to exit without saving

This allows you to specify custom instructions for the AI model. For example:

- Enforce specific commit message conventions
- Adjust the style or format of commit messages
- Add project-specific context or requirements

When a custom prompt is set, it will be used instead of the default system prompt for the selected language. To return to using the default prompt, set an empty custom prompt.

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
