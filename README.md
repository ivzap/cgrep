# cgrep
cgrep or "code grep" quickly finds code of interest within a project.

## Usage
cgrep (code grep) — a command-line tool similar to grep found on Unix-like systems, but optimized for fast and efficient source code search.
The backend of cgrep utilizes tree-sitter, a library that helps create syntax like trees from source code. While **cgrep** excels at source code search,
it is limited when searching for text fragments that do not conform to valid syntax in the target programming language.  

For example:
- Searching for `apples` alone in a C++ codebase will not match, since it does not form a complete syntax node.
- Searching for `int apples;` works because it is valid syntax and will be tokenized into nodes by Tree-sitter.

```r
cgrep <search-directory> <code>
```

## System Design ⚙️

<img width="1456" height="1020" alt="image" src="https://github.com/user-attachments/assets/64a66920-b6b4-4070-8903-e1a219bfb8a8" />

