import { Routes, Route } from 'react-router-dom';
import PageShell from './components/layout/PageShell';
import HomePage from './routes/HomePage';
import VisualizerPage from './routes/VisualizerPage';
import AlgorithmsPage from './routes/AlgorithmsPage';
import DataStructuresPage from './routes/DataStructuresPage';
import RoadmapPage from './routes/RoadmapPage';
import AboutPage from './routes/AboutPage';

function App() {
  return (
    <PageShell>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/visualizer" element={<VisualizerPage />} />
        <Route path="/algorithms" element={<AlgorithmsPage />} />
        <Route path="/data-structures" element={<DataStructuresPage />} />
        <Route path="/roadmap" element={<RoadmapPage />} />
        <Route path="/about" element={<AboutPage />} />
      </Routes>
    </PageShell>
  );
}

export default App;
