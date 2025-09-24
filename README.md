Minigrep
​​Minigrep​​ is a lightweight command-line tool written in Rust, inspired by the classic grepcommand. It allows you to search for a text pattern within files efficiently.

Features
​​Basic Text Search:​​ Search for a query string within a specified file.

​​Case-Insensitive Search:​​ Use environment variables to perform case-insensitive searches.

​​Error Handling:​​ Provides user-friendly error messages for common issues like file not found.

Installation
​​Prerequisites:​​ Ensure you have Rust and Cargoinstalled on your system.

​​Clone the Repository:​​

git clone https://github.com/Concurro/minigrep.git
cd minigrep
​​Build and Run:​​ You can run it directly using Cargo.

cargo run -- <query> <filepath>
​​Build for Production:​​ To build an optimized binary.

cargo build --release
The executable will be located at ./target/release/minigrep.

Usage
The basic syntax for using minigrepis:

minigrep <query> <filepath>
<query>: The string you want to search for.

<filepath>: The path to the file you want to search in.

Example
Search for the word "important" in notes.txt:

minigrep important notes.txt
​​Case-Insensitive Search:​​ Set the CASE_INSENSITIVEenvironment variable to 1to perform a search that ignores case.

# On Linux/macOS
export CASE_INSENSITIVE=1
minigrep Rust Cargo.toml

# On Windows Command Prompt
set CASE_INSENSITIVE=1
minigrep Rust Cargo.toml

# On Windows PowerShell
$env:CASE_INSENSITIVE=1
minigrep Rust Cargo.toml
This will find lines containing "Rust", "RUST", "rust", etc.

Project Structure
src/
  lib.rs  # Contains the core library logic (e.g., `search` function)
  main.rs # Contains the command-line argument parsing and error handling
Cargo.toml # Rust project manifest file
Cargo.lock # Dependency lock file
Contributing
Contributions are welcome! Please feel free to submit issues or pull requests on GitHub.

License
This project is licensed under the standard Rust project license (typically MIT or Apache-2.0). Please see the LICENSEfile in the repository for details.

Minigrep 中文说明
​​Minigrep​​ 是一个使用 Rust 编写的轻量级命令行工具，其灵感来源于经典的 grep命令。它可以让你高效地在文件中搜索文本模式。

功能特点
​​基础文本搜索:​​ 在指定文件中搜索查询字符串。

​​不区分大小写搜索:​​ 使用环境变量来执行不区分大小写的搜索。

​​错误处理:​​ 针对文件未找到等常见问题提供用户友好的错误信息。

安装
​​ prerequisites (前置要求):​​ 确保您的系统已安装 Rust 和 Cargo。

​​克隆代码库:​​

git clone https://github.com/Concurro/minigrep.git
cd minigrep
​​构建并运行:​​ 你可以直接使用 Cargo 运行它。

cargo run -- <查询词> <文件路径>
​​生产环境构建:​​ 要构建优化后的二进制可执行文件。

cargo build --release
生成的二进制文件将位于 ./target/release/minigrep。

使用方法
使用 minigrep的基本语法是：

minigrep <查询词> <文件路径>
<查询词>: 你想要搜索的字符串。

<文件路径>: 你想要搜索的文件的路径。

示例
在 notes.txt中搜索单词 "important"：

minigrep important notes.txt
​​不区分大小写搜索:​​ 将环境变量 CASE_INSENSITIVE设置为 1来执行不区分大小写的搜索。

# 在 Linux/macOS 上
export CASE_INSENSITIVE=1
minigrep Rust Cargo.toml

# 在 Windows Command Prompt 上
set CASE_INSENSITIVE=1
minigrep Rust Cargo.toml

# 在 Windows PowerShell 上
$env:CASE_INSENSITIVE=1
minigrep Rust Cargo.toml
这将找到包含 "Rust"、"RUST"、"rust" 等内容的行。

项目结构
src/
  lib.rs  # 包含核心库逻辑（例如 `search` 函数）
  main.rs # 包含命令行参数解析和错误处理
Cargo.toml # Rust 项目清单文件
Cargo.lock # 依赖锁文件
贡献代码
欢迎贡献！您可以在 GitHub 上提交 Issue 或 Pull Request。

许可证
此项目采用 Rust 项目的标准许可证（通常为 MIT 或 Apache-2.0）。请查看代码库中的 LICENSE文件以了解详情。
