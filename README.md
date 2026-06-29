# data-structure-visualization

数据结构可视化 — Rust Core + WebAssembly + React UI

## 项目简介

本项目是一个用于学习和展示数据结构与算法运行过程的**纯前端**可视化工具。算法核心由 **Rust** 实现，通过 **WebAssembly** 暴露给前端，前端使用 **React + TypeScript + Vite** 负责页面、动画和交互。

未来计划部署到 **Cloudflare Pages**，通过 `ds.drawrain.me` 访问。

> 注意：`drawrain.me` 是我的个人博客，本项目与博客完全独立。

## 技术栈

- Rust + wasm-bindgen
- WebAssembly (wasm-pack)
- React 18 + TypeScript
- Vite
- React Router
- Framer Motion
- Tailwind CSS

## 目录结构

```
data-structure-visualization/
├── Cargo.toml
├── package.json
├── README.md
├── crates/
│   ├── visualizer-core/      # 纯 Rust 算法核心
│   └── visualizer-wasm/      # wasm-bindgen 导出
└── web/                      # React + TypeScript 前端
    ├── package.json
    ├── vite.config.ts
    └── src/
```

## 本地开发

### 环境要求

1. 安装 [Rust](https://www.rust-lang.org/tools/install)
2. 安装 [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3. 安装 Node.js（建议 18+）

### 安装依赖

```bash
# 安装前端依赖
cd web
npm install

# 或从根目录
cd web && npm install
```

### 常用命令

```bash
# 启动开发服务器（会自动先编译 WASM）
npm run dev

# 构建生产版本（含 WASM 编译）
npm run build

# 单独编译 WASM
npm run build:wasm

# 运行 Rust 单元测试
cargo test
# 或
npm run test:rust
```

## 部署到 Cloudflare Pages

### 方案一：直接在 Cloudflare Pages 构建

Cloudflare Pages 的构建环境默认可能不带 Rust 和 wasm-pack，需要在构建设置中配置环境变量和安装命令：

- **构建命令**：
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && npm install && cd web && npm install && cd .. && npm run build
  ```
- **构建输出目录**：`web/dist`

> 注意：不同平台可能需要调整 wasm-pack 的安装方式。

### 方案二：GitHub Actions 构建后部署

推荐做法：使用 GitHub Actions 安装 Rust 和 wasm-pack，执行 `npm run build` 生成 `web/dist`，再通过 [cloudflare/pages-action](https://github.com/cloudflare/pages-action) 或 wrangler 部署到 Cloudflare Pages。

### 绑定子域名

在 Cloudflare Pages 的项目设置中，将自定义域名绑定为 `ds.drawrain.me`。`drawrain.me` 主站不需要任何改动。

## 算法实现

所有排序算法都在 `crates/visualizer-core/src/sorting/` 中实现，并生成 `TraceStep` 序列。前端仅消费 Rust 返回的 trace 数据，不自己实现排序逻辑。

已实现的算法：

- 选择排序（Selection Sort）
- 冒泡排序（Bubble Sort）
- 插入排序（Insertion Sort）
- 快速排序（Quick Sort）
- 归并排序（Merge Sort）

## 开源协议

保留原仓库 LICENSE 文件。