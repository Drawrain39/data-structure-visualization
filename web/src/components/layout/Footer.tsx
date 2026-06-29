export default function Footer() {
  return (
    <footer className="border-t border-slate-800 bg-slate-950 py-8">
      <div className="mx-auto max-w-7xl px-4 text-center text-sm text-slate-500">
        <p>数据结构可视化 — Rust Core + WebAssembly + React UI</p>
        <p className="mt-2">© {new Date().getFullYear()} 数据结构可视化</p>
      </div>
    </footer>
  );
}
