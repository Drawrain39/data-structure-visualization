use crate::types::*;

pub fn graph_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry { algorithm: "bfs".into(),
            meta: meta_simple!("bfs","广度优先搜索","BFS",Graph,"从起点出发逐层访问相邻节点。",time="O(V+E)",space="O(V)",use=["最短路径","连通性"]),
            samples: CodeSamples {
                cpp: cpp!("void bfs(vector<vector<int>>& graph, int start) {","  vector<bool> visited(graph.size(), false);","  queue<int> q;","  visited[start] = true; q.push(start);","  while (!q.empty()) {","    int v = q.front(); q.pop();","    for (int u : graph[v])","      if (!visited[u]) { visited[u]=true;","        q.push(u); }","  }","}"),
                python: py!("def bfs(graph, start):","    from collections import deque","    visited = [False] * len(graph)","    q = deque([start])","    visited[start] = True","    while q:","        v = q.popleft()","        for u in graph[v]:","            if not visited[u]:","                visited[u] = True","                q.append(u)"),
                rust: rs!("fn bfs(graph: &[Vec<usize>], start: usize) {","    let mut visited = vec![false; graph.len()];","    let mut q = VecDeque::new();","    visited[start] = true; q.push_back(start);","    while let Some(v) = q.pop_front() {","        for &u in &graph[v] {","            if !visited[u] { visited[u]=true;","                q.push_back(u); }","        }","    }","}"),
            },
            line_map: lm!("bfs"=>[1],"visit"=>[6],"enqueue"=>[8],"done"=>[10]),
        },
        AlgorithmEntry { algorithm: "dfs".into(),
            meta: meta_simple!("dfs","深度优先搜索","DFS",Graph,"从起点出发沿一条路径深入访问。",time="O(V+E)",space="O(V)",use=["拓扑排序","连通分量"]),
            samples: CodeSamples {
                cpp: cpp!("void dfs(vector<vector<int>>& graph, int v,","         vector<bool>& visited) {","  visited[v] = true;","  for (int u : graph[v])","    if (!visited[u])","      dfs(graph, u, visited);","}"),
                python: py!("def dfs(graph, v, visited):","    visited[v] = True","    for u in graph[v]:","        if not visited[u]:","            dfs(graph, u, visited)"),
                rust: rs!("fn dfs(graph: &[Vec<usize>], v: usize, visited: &mut [bool]) {","    visited[v] = true;","    for &u in &graph[v] {","        if !visited[u] {","            dfs(graph, u, visited);","        }","    }","}"),
            },
            line_map: lm!("dfs"=>[1],"visit"=>[3],"done"=>[7]),
        },
        AlgorithmEntry { algorithm: "dijkstra".into(),
            meta: meta_simple!("dijkstra","Dijkstra 最短路径","Dijkstra",Graph,"贪心扩展，逐步确定每个节点的最短距离。",time="O((V+E)logV)",space="O(V)",use=["路由算法","地图导航"]),
            samples: CodeSamples {
                cpp: cpp!("void dijkstra(vector<vector<pair<int,int>>>& g,int s) {","  vector<int> dist(g.size(), INT_MAX);","  priority_queue<pair<int,int>> pq;","  dist[s]=0; pq.push({0,s});","  while(!pq.empty()) {","    auto [d,u]=pq.top(); pq.pop();","    if(d>dist[u]) continue;","    for(auto [v,w]:g[u])","      if(dist[v]>dist[u]+w) {","        dist[v]=dist[u]+w; pq.push({dist[v],v});","      }","  }","}"),
                python: py!("def dijkstra(g, s):","    import heapq","    dist = [float('inf')] * len(g)","    dist[s] = 0","    pq = [(0, s)]","    while pq:","        d, u = heapq.heappop(pq)","        if d > dist[u]: continue","        for v, w in g[u]:","            if dist[v] > dist[u] + w:","                dist[v] = dist[u] + w","                heapq.heappush(pq, (dist[v], v))","    return dist"),
                rust: rs!("fn dijkstra(g: &[Vec<(usize,i32)>], s: usize) -> Vec<i32> {","    let mut dist = vec![i32::MAX; g.len()];","    let mut pq = BinaryHeap::new();","    dist[s]=0; pq.push(Reverse((0,s)));","    while let Some(Reverse((d,u))) = pq.pop() {","        if d > dist[u] { continue; }","        for &(v,w) in &g[u] {","            if dist[v] > dist[u] + w {","                dist[v] = dist[u] + w;","                pq.push(Reverse((dist[v],v)));","            }","        }","    }","    dist","}"),
            },
            line_map: lm!("dijkstra"=>[1],"init"=>[3],"visit"=>[6],"relax"=>[9],"done"=>[14]),
        },
        AlgorithmEntry { algorithm: "topological-sort".into(),
            meta: meta_simple!("topological-sort","拓扑排序","Topological Sort",Graph,"Kahn 算法，按入度逐步输出 DAG 的有序序列。",time="O(V+E)",space="O(V)",use=["任务调度","依赖解析"]),
            samples: CodeSamples {
                cpp: cpp!("vector<int> topoSort(vector<vector<int>>& g) {","  int n=g.size(); vector<int> indeg(n,0), res;","  for(auto& es:g) for(int v:es) indeg[v]++;","  queue<int> q;","  for(int i=0;i<n;i++) if(!indeg[i]) q.push(i);","  while(!q.empty()) {","    int u=q.front(); q.pop(); res.push_back(u);","    for(int v:g[u]) if(--indeg[v]==0) q.push(v);","  }","  return res;","}"),
                python: py!("def topo_sort(g):","    n = len(g)","    indeg = [0] * n","    for es in g:","        for v in es: indeg[v] += 1","    q = deque([i for i in range(n) if indeg[i]==0])","    res = []","    while q:","        u = q.popleft(); res.append(u)","        for v in g[u]:","            indeg[v] -= 1","            if indeg[v] == 0: q.append(v)","    return res"),
                rust: rs!("fn topo_sort(g: &[Vec<usize>]) -> Vec<usize> {","    let n = g.len();","    let mut indeg = vec![0; n];","    for es in g { for &v in es { indeg[v]+=1; } }","    let mut q: VecDeque<_> = (0..n).filter(|&i| indeg[i]==0).collect();","    let mut res = vec![];","    while let Some(u) = q.pop_front() {","        res.push(u);","        for &v in &g[u] { indeg[v]-=1;","            if indeg[v]==0 { q.push_back(v); } }","    }","    res","}"),
            },
            line_map: lm!("topo-sort"=>[1],"init-queue"=>[5],"process"=>[7],"enqueue"=>[10],"done"=>[13]),
        },
        AlgorithmEntry { algorithm: "kruskal".into(),
            meta: meta_simple!("kruskal","Kruskal 最小生成树","Kruskal MST",Graph,"按边权升序，用并查集逐步加入不构成环的边。",time="O(E log E)",space="O(V)",use=["网络布线","聚类"]),
            samples: CodeSamples {
                cpp: cpp!("int kruskal(vector<tuple<int,int,int>>& edges,int n){","  sort(edges.begin(),edges.end());","  vector<int> parent(n); iota(parent.begin(),parent.end(),0);","  int cost=0;","  for(auto [w,u,v]:edges)","    if(find(parent,u)!=find(parent,v)){","      union_(parent,u,v); cost+=w;","    }","  return cost;","}"),
                python: py!("def kruskal(edges, n):","    edges.sort(key=lambda x:x[0])","    parent = list(range(n))","    def find(x):","        while parent[x]!=x: parent[x]=parent[parent[x]]; x=parent[x]","        return x","    cost = 0","    for w, u, v in edges:","        if find(u) != find(v):","            parent[find(u)] = find(v)","            cost += w","    return cost"),
                rust: rs!("fn kruskal(edges: &mut [(i32,usize,usize)], n: usize) -> i32 {","    edges.sort_by_key(|&(w,_,_)| w);","    let mut parent: Vec<usize> = (0..n).collect();","    fn find(p: &mut[usize], x: usize) -> usize {","        if p[x]!=x { p[x]=find(p,p[x]); } p[x]","    }","    let mut cost = 0;","    for &(w,u,v) in edges.iter() {","        if find(&mut parent,u)!=find(&mut parent,v) {","            parent[find(&mut parent,u)]=find(&mut parent,v);","            cost += w;","        }","    }","    cost","}"),
            },
            line_map: lm!("kruskal"=>[1],"consider"=>[6],"select"=>[8],"done"=>[14]),
        },
        AlgorithmEntry { algorithm: "prim".into(),
            meta: meta_simple!("prim","Prim 最小生成树","Prim MST",Graph,"从起点出发，每次都选择连接已选集合的最小边。",time="O((V+E)logV)",space="O(V)",use=["网络布线","连通图"]),
            samples: CodeSamples {
                cpp: cpp!("int prim(vector<vector<pair<int,int>>>& g) {","  int n=g.size(), cost=0;","  vector<bool> vis(n);","  priority_queue<pair<int,int>> pq;","  pq.push({0,0});","  while(!pq.empty()) {","    auto [w,u]=pq.top(); pq.pop();","    if(vis[u]) continue; vis[u]=true; cost+=w;","    for(auto [v,w2]:g[u])","      if(!vis[v]) pq.push({w2,v});","  }","  return cost;","}"),
                python: py!("def prim(g):","    import heapq","    n = len(g)","    vis = [False] * n","    pq = [(0, 0)]","    cost = 0","    while pq:","        w, u = heapq.heappop(pq)","        if vis[u]: continue","        vis[u] = True; cost += w","        for v, w2 in g[u]:","            if not vis[v]:","                heapq.heappush(pq, (w2, v))","    return cost"),
                rust: rs!("fn prim(g: &[Vec<(usize,i32)>]) -> i32 {","    let n = g.len();","    let mut vis = vec![false; n];","    let mut pq = BinaryHeap::new();","    pq.push(Reverse((0,0)));","    let mut cost = 0;","    while let Some(Reverse((w,u))) = pq.pop() {","        if vis[u] { continue; }","        vis[u] = true; cost += w;","        for &(v,w2) in &g[u] {","            if !vis[v] { pq.push(Reverse((w2,v))); }","        }","    }","    cost","}"),
            },
            line_map: lm!("prim"=>[1],"start"=>[5],"compare"=>[9],"add"=>[10],"done"=>[14]),
        },
    ]
}
