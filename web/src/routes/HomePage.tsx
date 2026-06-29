import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { BarChart3, ArrowRight, Code2, Layers, Route } from 'lucide-react';

const features = [
  {
    icon: Code2,
    title: 'Rust 核心',
    desc: '排序算法由 Rust 实现，编译为 WebAssembly 运行，保证性能与正确性。',
  },
  {
    icon: BarChart3,
    title: '流畅动画',
    desc: '使用 Framer Motion 实现元素真实移动、比较上浮、交换强调等动画效果。',
  },
  {
    icon: Layers,
    title: '多语言代码',
    desc: '同步展示 C++ / Python / Rust 代码，并随算法步骤高亮对应行。',
  },
  {
    icon: Route,
    title: '学习路线',
    desc: '从基础排序到高级数据结构，规划清晰的学习路径。',
  },
];

export default function HomePage() {
  return (
    <div className="relative overflow-hidden">
      <div className="absolute inset-0 -z-10 bg-gradient-to-b from-accent/5 via-transparent to-transparent" />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <motion.div
          initial={{ opacity: 0, y: 24 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="text-center"
        >
          <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-slate-800 bg-slate-900/60 px-4 py-1.5 text-sm text-slate-300">
            <span className="h-2 w-2 rounded-full bg-accent" />
            Rust + WebAssembly + React
          </div>
          <h1 className="text-4xl font-extrabold tracking-tight text-slate-50 sm:text-6xl">
            数据结构可视化
          </h1>
          <p className="mx-auto mt-6 max-w-2xl text-lg text-slate-400">
            让算法动起来。通过交互式动画和代码同步高亮，深入理解排序算法与数据结构的运行过程。
          </p>
          <div className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row">
            <Link
              to="/visualizer"
              className="inline-flex items-center gap-2 rounded-xl bg-accent px-6 py-3 text-base font-bold text-slate-950 shadow-lg shadow-accent/20 transition-transform hover:scale-105"
            >
              开始可视化
              <ArrowRight className="h-5 w-5" />
            </Link>
            <Link
              to="/algorithms"
              className="inline-flex items-center gap-2 rounded-xl border border-slate-700 bg-slate-900/60 px-6 py-3 text-base font-medium text-slate-200 hover:bg-slate-800"
            >
              浏览算法库
            </Link>
          </div>
        </motion.div>
      </section>

      <section className="mx-auto max-w-7xl px-4 pb-24 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
          {features.map((f, i) => (
            <motion.div
              key={f.title}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 * i, duration: 0.5 }}
              className="rounded-2xl border border-slate-800 bg-slate-900/60 p-6 shadow-xl"
            >
              <div className="mb-4 inline-flex rounded-xl bg-accent/10 p-3 text-accent">
                <f.icon className="h-6 w-6" />
              </div>
              <h3 className="mb-2 text-lg font-bold text-slate-50">{f.title}</h3>
              <p className="text-sm leading-relaxed text-slate-400">{f.desc}</p>
            </motion.div>
          ))}
        </div>
      </section>
    </div>
  );
}
