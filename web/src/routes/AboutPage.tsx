export default function AboutPage() {
  return (
    <div className="mx-auto max-w-3xl px-4 py-8 sm:px-6 lg:px-8">
      <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-8 shadow-xl">
        <h1 className="text-2xl font-bold text-slate-50">关于</h1>
        <p className="mt-4 leading-relaxed text-slate-300">
          数据结构可视化是一个用于学习和展示数据结构与算法运行过程的交互式工具。
          项目采用 Rust Core + WebAssembly + React UI 的架构，所有算法核心由
          Rust 实现并通过 WebAssembly 暴露给前端，前端只负责展示与交互。
        </p>
        <p className="mt-4 leading-relaxed text-slate-300">
          当前版本覆盖排序、查找、线性表、栈与队列、递归、树、图等多个主题，
          未来会持续补充更多算法与数据结构的可视化。
        </p>
        <div className="mt-6 space-y-2 text-sm text-slate-400">
          <p><strong className="text-slate-200">技术栈：</strong> Rust, wasm-bindgen, WebAssembly, React, TypeScript, Vite, Framer Motion, Tailwind CSS</p>
          <p><strong className="text-slate-200">部署目标：</strong> Cloudflare Pages</p>
          <p><strong className="text-slate-200">域名：</strong> ds.drawrain.me</p>
        </div>
      </div>
    </div>
  );
}
