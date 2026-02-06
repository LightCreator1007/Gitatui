# Gitatui (gtu)
**A fast, elegant Git TUI for developers who prefer insight over incantations.**

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)](#)
[![License](https://img.shields.io/badge/license-MIT-blue)](#)

> Visualize your **branches** and **commit history** side-by-side in a beautiful terminal UI:
> no shelling out, no lag, no nonsense.

📸  **Preview** 
 
<img width="900" height="450" alt="Image" src="https://github.com/user-attachments/assets/9d6946be-026e-4514-aa9c-81aa59781f15" />

---

## ✨ Why Gitatui?

Stop spamming `git log --graph --oneline --decorate` 

**Gitatui (gtu)** is a **high-performance Git TUI** built in **Rust**, designed for developers who want:
- Instant feedback
- Clean visuals
- Keyboard first workflows

Powered by **Libgit2**.

---

## 🚀 Features

### ⚡ Fast. Like, Actually Fast.
- Direct Git access via **Libgit2**
- No process spawning
- Scales smoothly even on large repos

### 🎨 Tokyo Night Aesthetic
- Carefully tuned **Tokyo Night** color palette
- High contrast, easy on the eyes


### 🧭 Vim-Style Navigation
- `h / j / k / l` : because your fingers already know
- Intuitive, user friendly controls

### 👀 Live Preview (The Star Feature)
- Browse commits **and** see details update **live**
- Branches on one side, history on the other
- No context switching.

---

## 📦 Installation

All binaries are available on **GitHub Releases**.

👉 **Go to:** [Releases](https://github.com/LightCreator1007/Gitatui/releases/tag/v0.1.0)

### 🪟 Windows
- Download the `.zip`: [Download for Windows](https://github.com/LightCreator1007/Gitatui/releases/download/v0.1.0/gtu-x86_64-pc-windows-msvc.zip)
- Extract
- Add the binary to your `PATH`
- Done. No ceremony.

### 🍎 macOS (Apple Silicon: aarch64)

👉 **Download:**  
[Download for MacOs](https://github.com/LightCreator1007/Gitatui/releases/download/v0.1.0/gtu-aarch64-apple-darwin.tar.gz)

```bash
tar -xzf gtu-aarch64-apple-darwin.tar.gz
sudo mv gtu /usr/local/bin
```
### 🐧Linux

DownLoad the binary: [Download for linux](https://github.com/LightCreator1007/Gitatui/releases/download/v0.1.0/gtu-x86_64-unknown-linux-gnu.tar.gz)
```bash
tar -xzf gtu-x86_64-unknown-linux-gnu.tar.gz
sudo mv gtu /usr/local/bin
```
---

## Build from scratch:

- Clone the repo:
```bash
git clone https://github.com/LightCreator1007/Gitatui.git
cd gitatui

```
-Build (Dev Mode): `cargo run`
-Build(release): `cargo build --release`

---

## 🔮 Roadmap v0.2.0
We are actively working on the following features:

[ ] Action System: Press Enter to checkout branches.

[ ] Stash Management: Push and Pop stashes via keybinds.

[ ] Editor Integration: Open file diffs in VS Code/Neovim

## 🤝 Contributing
PRs are welcome! Please follow the "Fork & Pull" workflow.

Fork the repository.

Create a feature branch (git checkout -b feature/amazing-feature).

Commit your changes.

Open a Pull Request.




