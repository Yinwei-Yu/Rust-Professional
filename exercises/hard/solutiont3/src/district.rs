// src/district.rs
use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;
use std::fs;

type Batch = BTreeMap<String, Vec<String>>; // 改用有序Map

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        // 固定合并顺序：总是将小索引合并到大索引
        if x_root < y_root {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
        }
    }
}

pub fn count_provinces() -> String {
    let input = fs::read_to_string("district.json").expect("读取文件失败");
    let batches: HashMap<String, Batch> = serde_json::from_str(&input).expect("JSON解析失败");

    let mut results = vec![];
    
    for (_, batch) in batches {
        // 使用BTreeSet保证城市顺序稳定
        let mut cities:BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for (city, neighbors) in &batch {
            cities.entry(city.as_str()).or_insert(vec![]);
            for n in neighbors {
                cities.entry(n.as_str()).or_insert(vec![]);
            }
        }

        // 生成稳定索引映射
        let city_list: Vec<&str> = cities.keys().copied().collect();
        let city_index: HashMap<_, _> = city_list
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        let mut uf = UnionFind::new(city_list.len());
        
        // 按字母顺序处理城市
        for (city, neighbors) in batch.iter() {
            let city_idx = city_index[city.as_str()];
            for neighbor in neighbors {
                let neighbor_idx = city_index[neighbor.as_str()];
                uf.union(city_idx, neighbor_idx);
            }
        }

        // 计算连通分量
        let mut roots = vec![];
        for i in 0..city_list.len() {
            roots.push(uf.find(i));
        }
        roots.sort_unstable();
        roots.dedup();
        
        results.push(roots.len().to_string());
    }

    results.join(",")
}