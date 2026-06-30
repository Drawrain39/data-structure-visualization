use crate::types::*;

pub fn searching_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry { algorithm: "linear-search".into(),
            meta: meta_no_stable!("linear-search","顺序查找","Linear Search",Searching,"从头到尾逐个比较查找目标值。",time="O(n)",best="O(1)",avg="O(n)",worst="O(n)",space="O(1)",use=["无序数据","小数据集"]),
            samples: CodeSamples {
                cpp: cpp!("int linearSearch(vector<int>& arr, int target) {","  for (int i = 0; i < arr.size(); i++) {","    if (arr[i] == target) return i;","  }","  return -1;","}"),
                python: py!("def linear_search(arr, target):","    for i, v in enumerate(arr):","        if v == target:","            return i","    return -1"),
                rust: rs!("fn linear_search(arr: &[i32], target: i32) -> Option<usize> {","    for (i, &v) in arr.iter().enumerate() {","        if v == target {","            return Some(i);","        }","    }","    None","}"),
            },
            line_map: lm!("linear-search"=>[1],"compare"=>[3],"found"=>[3],"done"=>[5]),
        },
        AlgorithmEntry { algorithm: "binary-search".into(),
            meta: meta_no_stable!("binary-search","二分查找","Binary Search",Searching,"在有序数组中通过折半快速定位目标。",time="O(log n)",best="O(1)",avg="O(log n)",worst="O(log n)",space="O(1)",use=["有序数组","频繁查询"]),
            samples: CodeSamples {
                cpp: cpp!("int binarySearch(vector<int>& arr, int target) {","  int l = 0, r = arr.size() - 1;","  while (l <= r) {","    int m = l + (r - l) / 2;","    if (arr[m] == target) return m;","    if (arr[m] < target) l = m + 1;","    else r = m - 1;","  }","  return -1;","}"),
                python: py!("def binary_search(arr, target):","    l, r = 0, len(arr) - 1","    while l <= r:","        m = (l + r) // 2","        if arr[m] == target:","            return m","        if arr[m] < target:","            l = m + 1","        else:","            r = m - 1","    return -1"),
                rust: rs!("fn binary_search(arr: &[i32], target: i32) -> Option<usize> {","    let mut l = 0;","    let mut r = arr.len() - 1;","    while l <= r {","        let m = l + (r - l) / 2;","        if arr[m] == target { return Some(m); }","        if arr[m] < target { l = m + 1; }","        else { r = m - 1; }","    }","    None","}"),
            },
            line_map: lm!("binary-search"=>[1],"mid"=>[4],"compare"=>[5],"found"=>[5],"done"=>[10]),
        },
        AlgorithmEntry { algorithm: "interpolation-search".into(),
            meta: meta_no_stable!("interpolation-search","插值查找","Interpolation Search",Searching,"根据目标值估算其在有序数组中的位置。",time="O(log log n)",best="O(1)",avg="?",worst="O(n)",space="O(1)",use=["均匀分布有序数据"]),
            samples: CodeSamples {
                cpp: cpp!("int interpolationSearch(vector<int>& arr,int t){","  int lo=0, hi=arr.size()-1;","  while(lo<=hi&&t>=arr[lo]&&t<=arr[hi]){","    int pos=lo+(t-arr[lo])*(hi-lo)/(arr[hi]-arr[lo]);","    if(arr[pos]==t) return pos;","    if(arr[pos]<t) lo=pos+1; else hi=pos-1;","  }","  return -1;","}"),
                python: py!("def interpolation_search(arr, t):","    lo, hi = 0, len(arr)-1","    while lo <= hi and arr[lo] <= t <= arr[hi]:","        pos = lo + (t-arr[lo])*(hi-lo)//(arr[hi]-arr[lo])","        if arr[pos] == t: return pos","        if arr[pos] < t: lo = pos+1","        else: hi = pos-1","    return -1"),
                rust: rs!("fn interpolation_search(arr: &[i32], t: i32) -> Option<usize> {","    let mut lo:i32 = 0;","    let mut hi = (arr.len() as i32)-1;","    while lo<=hi && t>=arr[lo as usize] && t<=arr[hi as usize] {","        let pos = lo+(t-arr[lo as usize])*(hi-lo)/(arr[hi as usize]-arr[lo as usize]).max(1);","        if arr[pos as usize]==t{return Some(pos as usize);}","        if arr[pos as usize]<t{lo=pos+1;}else{hi=pos-1;}","    }","    None","}"),
            },
            line_map: lm!("interpolation-search"=>[1],"probe"=>[4],"found"=>[5],"done"=>[9]),
        },
        AlgorithmEntry { algorithm: "hash-search".into(),
            meta: meta_simple!("hash-search","哈希查找","Hash Search",Searching,"通过哈希函数直接映射到存储位置，O(1) 查询。",time="O(1)",space="O(n)",use=["大量数据快速查找","去重"]),
            samples: CodeSamples {
                cpp: cpp!("int hashSearch(vector<int>& arr, int t) {","  unordered_map<int,int> map;","  for(int i=0;i<arr.size();i++) map[arr[i]]=i;","  if(map.count(t)) return map[t];","  return -1;","}"),
                python: py!("def hash_search(arr, t):","    table = {v: i for i, v in enumerate(arr)}","    return table.get(t, -1)"),
                rust: rs!("fn hash_search(arr: &[i32], t: i32) -> Option<usize> {","    use std::collections::HashMap;","    let map: HashMap<i32,usize> = arr.iter().enumerate().map(|(i,&v)|(v,i)).collect();","    map.get(&t).copied()","}"),
            },
            line_map: lm!("hash-search"=>[1],"build"=>[3],"query"=>[3],"found"=>[4],"done"=>[5]),
        },
    ]
}
