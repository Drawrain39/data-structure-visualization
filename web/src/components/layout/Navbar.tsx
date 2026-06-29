import { Link, useLocation } from 'react-router-dom';
import { BarChart3 } from 'lucide-react';

const navItems = [
  { path: '/', label: '首页' },
  { path: '/visualizer', label: '可视化' },
  { path: '/algorithms', label: '算法库' },
  { path: '/data-structures', label: '数据结构' },
  { path: '/roadmap', label: '学习路线' },
  { path: '/about', label: '关于' },
];

export default function Navbar() {
  const { pathname } = useLocation();

  return (
    <header className="sticky top-0 z-50 glass border-b border-slate-700/30">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 items-center justify-between">
          <Link to="/" className="flex items-center gap-2 text-accent">
            <BarChart3 className="h-7 w-7" />
            <span className="text-lg font-bold tracking-tight text-slate-50">
              数据结构可视化
            </span>
          </Link>

          <nav className="hidden md:flex items-center gap-1">
            {navItems.map((item) => {
              const active = pathname === item.path;
              return (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                    active
                      ? 'bg-accent/10 text-accent'
                      : 'text-slate-300 hover:bg-slate-800 hover:text-slate-50'
                  }`}
                >
                  {item.label}
                </Link>
              );
            })}
          </nav>

          <nav className="flex md:hidden items-center gap-1">
            {navItems.slice(0, 3).map((item) => {
              const active = pathname === item.path;
              return (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`rounded-md px-2 py-1.5 text-xs font-medium transition-colors ${
                    active
                      ? 'bg-accent/10 text-accent'
                      : 'text-slate-300 hover:bg-slate-800 hover:text-slate-50'
                  }`}
                >
                  {item.label}
                </Link>
              );
            })}
          </nav>
        </div>
      </div>
    </header>
  );
}
