# 数据结构可视化

一个使用 **Rust + WebAssembly + React** 构建的数据结构与算法可视化项目。

算法核心由 Rust 实现，通过 WebAssembly 暴露给前端；前端负责动画展示、代码同步高亮和交互控制。

访问地址：https://ds.drawrain.me

> `drawrain.me` 为个人博客，本项目与其独立。

## 项目特点

- Rust 实现算法核心，前端不实现排序逻辑
- WebAssembly 提供浏览器端调用能力
- React + TypeScript 构建前端界面
- 算法执行过程动画演示
- C++ / Python / Rust 代码同步展示与高亮
- 单步执行、自动播放、暂停、重置、速度调节
- 自定义数组输入与随机数组生成

## 已覆盖主题

- **排序**：选择、冒泡、插入、希尔、归并、快速、堆、计数、桶、基数排序
- **查找**：顺序查找、二分查找、插值查找、哈希查找
- **线性表**：数组插入/删除、链表遍历
- **栈与队列**：栈 push/pop、队列 enqueue/dequeue
- **递归**：阶乘、斐波那契、汉诺塔
- **树**：BST 插入/查找、堆插入、AVL 插入、前/中/后/层序遍历
- **图**：BFS、DFS、Dijkstra、拓扑排序、Kruskal、Prim
- **动态规划**：斐波那契 DP、0/1 背包、LCS、LIS

## 技术栈

- Rust
- wasm-bindgen
- WebAssembly
- React
- TypeScript
- Vite
- Framer Motion
- Tailwind CSS

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

需要 Rust、wasm-pack 和 Node.js。

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

构建产物位于 `web/dist`。

## 测试

```bash
cargo test
```

## 部署

部署到 Cloudflare Pages。

- 构建命令：`npm run build`
- 构建输出目录：`web/dist`

Cloudflare Pages 构建环境默认不带 Rust 和 wasm-pack，可在构建设置中先安装：

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && npm install && npm run build
```

Windows 环境可改用 `cargo install wasm-pack`。

## 开源协议

保留原仓库 LICENSE 文件。