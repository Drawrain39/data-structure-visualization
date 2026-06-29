# 数据结构可视化

一个使用 **Rust + WebAssembly + React** 构建的数据结构与算法可视化项目。

项目核心算法由 Rust 实现，通过 WebAssembly 暴露给前端；前端负责动画展示、代码同步高亮和交互控制。

访问地址：

```text
https://ds.drawrain.me
```

## 项目特点

* Rust 实现算法核心
* WebAssembly 提供浏览器端调用能力
* React + TypeScript 构建前端界面
* 支持算法执行过程动画演示
* 支持 C++ / Python / Rust 代码同步展示
* 支持单步执行、自动播放、暂停、重置和速度调节
* 支持自定义数组输入和随机数组生成

## 已实现算法

* 选择排序
* 冒泡排序
* 插入排序
* 快速排序
* 归并排序

## 技术栈

* Rust
* wasm-bindgen
* WebAssembly
* React
* TypeScript
* Vite
* Framer Motion
* Tailwind CSS

## 项目结构

```text
data-structure-visualization/
├── crates/
│   ├── visualizer-core/      # Rust 算法核心
│   └── visualizer-wasm/      # WebAssembly 绑定
├── web/                      # React 前端
├── Cargo.toml
├── package.json
└── README.md
```

## 本地运行

需要先安装 Rust、wasm-pack 和 Node.js。

```bash
npm install
cd web && npm install
cd ..

npm run dev
```

## 构建

```bash
npm run build
```

构建产物位于：

```text
web/dist
```

## 测试

```bash
cargo test
```

## 部署

项目部署在 Cloudflare Pages。

构建命令：

```bash
npm run build
```

构建输出目录：

```text
web/dist
```
