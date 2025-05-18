# Commit Message Suggester CLI

A simple Rust CLI tool that suggests helpful, AI-generated commit messages based on your current `git diff`. Built with `clap`, `git2`, and OpenAI.

---

## Features

- Analyze staged changes in your Git repo
- Generate **clear, concise commit messages**
- Fully local CLI usage

---

## How it works

This tool reads the `git diff` from your working directory and sends it to the OpenAI API. The AI then generates commit message suggestions based on the changes.

---

## Installation

### 1. Clone the Repo

```bash
git clone https://github.com/IbrahimBagalwa/commit_msg_generator.git
cd commit_msg_generator
```

### 2. Set Up Environment

Create a `.env` file with your OpenAI API key:

```env
OPENAI_API_KEY=<your-openai-key>
OPENAI_API_ENDPOINT=<your-openai-endpoint>
```

### 3. Build the Binary

```bash
cargo build --release
```

### 4. Use it Globally (optional)

```bash
sudo cp target/release/commit_msg_generator /usr/local/bin/msg-suggest
```

Now you can use it anywhere with:

```bash
msg-suggest --path .
```

---

## Usage

1. Make changes in your project.
2. Stage your files:

```bash
git add .
```

3. Run the CLI:

```bash
cargo run -- --path .
```

Or if installed globally:

```bash
msg-suggest --path .
```

### Example Output:

```
Suggested Commit Messages:
1. Add user authentication check in login flow
2. Refactor login handler to improve error handling
3. Update user model with new validation rules
```

Pick your favorite and commit:

```bash
git commit -m "Add user authentication check in login flow"
```

## Contributing

1. Fork the repo
2. Create a new branch (`feature/my-feature`)
3. Commit your changes
4. Push to the branch
5. Open a Pull Request
