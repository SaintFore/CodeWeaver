# CodeWeaver

```text
   ______          __     _       __
  / ____/___  ____/ /__  | |     / /__  ____ __   _____  _____
 / /   / __ \/ __  / _ \ | | /| / / _ \/ __ `/ | / / _ \/ ___/
/ /___/ /_/ / /_/ /  __/ | |/ |/ /  __/ /_/ /| |/ /  __/ /
\____/\____/\__,_/\___/  |__/|__/\___/\__,_/ |___/\___/_/
```

<div align="center">

[![Rust](https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey?style=for-the-badge&logo=linux)](https://www.kernel.org/)
[![AUR](https://img.shields.io/badge/AUR-code--weaver-1793d1?style=for-the-badge&logo=arch-linux)](https://aur.archlinux.org/packages/code-weaver)
[![Size](https://img.shields.io/github/repo-size/SaintFore/CodeWeaver?style=for-the-badge&color=green)](https://github.com/SaintFore/CodeWeaver)

**"Weave your entire codebase into a single AI-ready stream."**
将你的代码库编织成 AI 渴望的上下文流。

[Installation](#installation) • [Usage](#usage) • [Features](#features) • [License](#license)

</div>

---

## ⚡ What is CodeWeaver?

在 AI 辅助编程时代，我们经常需要将整个项目结构“喂”给 LLM (ChatGPT, Claude, Gemini)。手动复制粘贴不仅愚蠢，而且容易遗漏。

**CodeWeaver** 是一个用 Rust 编写的极速 CLI 工具，它能递归地读取指定目录，智能过滤掉无关文件（如 `target`, `.git`, 二进制文件），并将所有源代码整合成一个带有文件路径标注的单一文本流。

**一键生成 Context，让 AI 瞬间读懂你的项目。**

## 🚀 Features

- **🌪️ Blazing Fast**: 基于 Rust 构建，毫秒级遍历处理大型代码库。
- **🧠 Smart Filtering**: 自动识别并包含主流编程语言 (`rs`, `py`, `js`, `go`, `cpp` 等 17+ 种)，无情剔除 `node_modules`, `target`, `.git` 等噪声。
- **📝 Context Aware**: 输出的每个文件前自动添加 `// path/to/file` 注释，完美保留文件结构信息，让 LLM 知道代码在哪里。
- **🛡️ Recursive Power**: 深度递归目录，不论你的项目结构多复杂，都能一网打尽。

## 📦 Installation

### 🦀 Cargo (Recommended)

如果你已安装 Rust 工具链：

```bash
cargo install --path .
```

### 🏹 Arch Linux (AUR)

Arch 用户可以直接从 AUR 安装：

```bash
yay -S code-weaver
```

_(或者使用 `paru`, `pikaur` 等你喜欢的 AUR 助手)_

### 🏗️ Build from Source

```bash
git clone https://github.com/SaintFore/CodeWeaver
cd CodeWeaver
cargo build --release
sudo cp target/release/code-weaver /usr/local/bin/
```

## 💻 Usage

基本用法非常简单：给它一个路径，它给你代码流。

### 1. 基础编织

将当前目录下的所有代码打印到标准输出：

```bash
code-weaver .
```

### 2. 生成 Context 文件 (最常用)

将 `src` 目录下的所有代码保存为 `context.txt`，直接拖进 AI 聊天框：

```bash
code-weaver ./src > context.txt
```

### 3. 多路径混合

同时抓取后端代码和配置文件：

```bash
code-weaver ./backend/src ./config/Settings.toml > full_stack_context.txt
```

## 📄 License

Based on the MIT License. See [LICENSE](LICENSE) for details.

---

<div align="center">
Made with ❤️ & 🦀 by <a href="https://github.com/SaintFore">SaintFore</a>
</div>

