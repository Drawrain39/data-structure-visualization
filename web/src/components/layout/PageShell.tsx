import Navbar from './Navbar';
import Footer from './Footer';

interface Props {
  children: React.ReactNode;
}

export default function PageShell({ children }: Props) {
  return (
    <div className="flex min-h-screen flex-col bg-slate-950">
      <Navbar />
      <main className="flex-1">{children}</main>
      <Footer />
    </div>
  );
}
