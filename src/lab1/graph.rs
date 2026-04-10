#![allow(non_snake_case)]

#![allow(dead_code)]



use once_cell::sync::Lazy;

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use std::sync::Mutex;



use super::category::LexemeCategory;

use super::char_set::{has_contain_char, is_subset, segments_of};

use super::edge::Edge;

use super::state::State;

use super::token::Token;



/// 图结构，既可表示 NFA（非确定有限状态自动机），也可表示 DFA（确定有限状态自动机）

#[derive(Clone, Debug)]

pub struct Graph {

    /// 自动机图的唯一标识 ID

    pub graphId: i32,

    /// 自动机状态的数量

    pub numOfStates: i32,

    /// 边的表（保存整个图中所有的状态转移）

    pub pEdgeTable: Vec<Edge>,

    /// 状态的表（保存图中的所有状态信息，包括是否为结束状态等）

    pub pStateTable: Vec<State>,

}



static CURR_GRAPH_NUM: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));



fn next_graph_id() -> i32 {

    let mut guard = CURR_GRAPH_NUM.lock().unwrap();

    *guard += 1;

    *guard

}



pub fn reset_graph_counter() {

    *CURR_GRAPH_NUM.lock().unwrap() = 0;

}



#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]

struct DriverKey {

    driver_id: i32,

    driver_type: String,

}



fn is_epsilon_edge(edge: &Edge) -> bool {

    edge.driverId == -1 && edge.DriverType == "NULL"

}



fn pick_preferred_category_from_ids<I>(

    state_ids: I,

    state_table: &[State],

) -> Option<LexemeCategory>

where

    I: IntoIterator<Item = i32>,

{

    let mut ids: Vec<i32> = state_ids.into_iter().collect();

    ids.sort_unstable();

    ids.dedup();



    let mut fallback_id = None;

    for sid in ids {

        let Some(state) = state_table.get(sid as usize) else {

            continue;

        };

        if let Some(category) = &state.LexemeCategory {

            if *category != LexemeCategory::ID {

                return Some(category.clone());

            }

            if fallback_id.is_none() {

                fallback_id = Some(LexemeCategory::ID);

            }

        }

    }

    fallback_id

}



fn category_code(category: &Option<LexemeCategory>) -> i32 {

    match category {

        None => -1,

        Some(LexemeCategory::KEYWORD) => 0,

        Some(LexemeCategory::ASSIGN_OPERATOR) => 1,

        Some(LexemeCategory::COMPARE_OPERATOR) => 2,

        Some(LexemeCategory::NUMERIC_OPERATOR) => 3,

        Some(LexemeCategory::LOGIC_OPERATOR) => 4,

        Some(LexemeCategory::INTEGER_CONST) => 5,

        Some(LexemeCategory::FLOAT_CONST) => 6,

        Some(LexemeCategory::SCIENTIFIC_CONST) => 7,

        Some(LexemeCategory::STRING_CONST) => 8,

        Some(LexemeCategory::NOTE) => 9,

        Some(LexemeCategory::SPACE_CONST) => 10,

        Some(LexemeCategory::ID) => 11,

    }

}



fn build_state_signature(graph: &Graph, state_id: i32, state_to_block: &[usize]) -> String {

    let mut grouped: BTreeMap<DriverKey, Vec<usize>> = BTreeMap::new();



    for edge in &graph.pEdgeTable {

        if edge.fromState != state_id {

            continue;

        }

        let Some(&target_block) = state_to_block.get(edge.nextState as usize) else {

            continue;

        };

        let key = DriverKey {

            driver_id: edge.driverId,

            driver_type: edge.DriverType.clone(),

        };

        grouped.entry(key).or_default().push(target_block);

    }



    let mut chunks = Vec::new();

    for (key, mut blocks) in grouped {

        blocks.sort_unstable();

        blocks.dedup();

        chunks.push(format!(

            "{}#{}->{:?}",

            key.driver_type, key.driver_id, blocks

        ));

    }



    let state = &graph.pStateTable[state_id as usize];

    format!(

        "A:{}|C:{}|T:{}",

        state.StateType == "MATCH",

        category_code(&state.LexemeCategory),

        chunks.join("|")

    )

}



impl Graph {

    /// 构造给定状态数量的空图

    pub fn new(num_states: i32) -> Self {

        Self {

            graphId: next_graph_id(),

            numOfStates: num_states,

            pEdgeTable: Vec::new(),

            pStateTable: Vec::new(),

        }

    }



    /// 深拷贝当前图

    pub fn copyGraph(&self) -> Graph {

        self.clone()

    }



    /// 在 driverId 边集上进行 move 操作

    /// 经过精简和优化的 NFA，去除不必要的冗余状态和空跳边

    /// 调度对 NFA 的全面最小化优化，包括消除 ε-边、修剪无用状态及合并等价状态。
///
/// Returns:
///   最小化后的 NFA 图。
/// 调度对 NFA 的全面最小化优化，包括消除 ε-边、修剪无用状态及合并等价状态。
///
/// Returns:
///   最小化后的 NFA 图。
pub fn minimize_nfa(&self) -> Graph {

        let epsilon_free = self.build_epsilon_free_nfa();

        let pruned = epsilon_free.prune_useless_states();

        let merged = pruned.merge_equivalent_states();

        merged.prune_useless_states()

    }



    /// 将带有 ε-边的 NFA 转换为不带 ε-边的 NFA，消除空跳状态。
///
/// Returns:
///   消除 ε-边后的图。
/// 将带有 ε-边的 NFA 转换为不带 ε-边的 NFA，消除空跳状态。
///
/// Returns:
///   消除 ε-边后的图。
fn build_epsilon_free_nfa(&self) -> Graph {

        let state_count = self.pStateTable.len();

        if state_count == 0 {

            return Graph {

                graphId: next_graph_id(),

                numOfStates: 0,

                pEdgeTable: Vec::new(),

                pStateTable: Vec::new(),

            };

        }



        let mut closures: Vec<HashSet<i32>> = Vec::with_capacity(state_count);

        for sid in 0..state_count {

            let mut seed = HashSet::new();

            seed.insert(sid as i32);

            closures.push(self.epsilon_closure(&seed));

        }



        let mut states = Vec::with_capacity(state_count);

        for sid in 0..state_count {

            let closure = &closures[sid];

            let is_match = closure.iter().any(|state_id| {

                self.pStateTable

                    .get(*state_id as usize)

                    .map(|s| s.StateType == "MATCH")

                    .unwrap_or(false)

            });

            let category = if is_match {

                pick_preferred_category_from_ids(closure.iter().copied(), &self.pStateTable)

            } else {

                None

            };



            states.push(State {

                stateId: sid as i32,

                StateType: if is_match {

                    "MATCH".to_string()

                } else {

                    "UNMATCH".to_string()

                },

                LexemeCategory: category,

            });

        }



        let mut edges = Vec::new();

        let mut seen: HashSet<(i32, i32, i32, String)> = HashSet::new();



        for sid in 0..state_count {

            let mut grouped_targets: HashMap<DriverKey, HashSet<i32>> = HashMap::new();

            for inner_state in &closures[sid] {

                for edge in &self.pEdgeTable {

                    if edge.fromState != *inner_state || is_epsilon_edge(edge) {

                        continue;

                    }

                    let Some(target_closure) = closures.get(edge.nextState as usize) else {

                        continue;

                    };

                    let key = DriverKey {

                        driver_id: edge.driverId,

                        driver_type: edge.DriverType.clone(),

                    };

                    let target_bucket = grouped_targets.entry(key).or_default();

                    for target in target_closure {

                        target_bucket.insert(*target);

                    }

                }

            }



            for (key, targets) in grouped_targets {

                let driver_id = key.driver_id;

                let driver_type = key.driver_type;

                for target in targets {

                    let dedup_key = (sid as i32, target, driver_id, driver_type.clone());

                    if seen.insert(dedup_key) {

                        edges.push(Edge {

                            fromState: sid as i32,

                            nextState: target,

                            driverId: driver_id,

                            DriverType: driver_type.clone(),

                        });

                    }

                }

            }

        }



        Graph {

            graphId: next_graph_id(),

            numOfStates: states.len() as i32,

            pEdgeTable: edges,

            pStateTable: states,

        }

    }



    fn rebuild_with_state_subset(&self, keep_ids: &[i32]) -> Graph {

        if keep_ids.is_empty() {

            return Graph {

                graphId: next_graph_id(),

                numOfStates: 0,

                pEdgeTable: Vec::new(),

                pStateTable: Vec::new(),

            };

        }



        let mut ordered = keep_ids.to_vec();

        ordered.sort_unstable();

        ordered.dedup();

        ordered.retain(|sid| *sid >= 0 && (*sid as usize) < self.pStateTable.len());

        if let Some(pos) = ordered.iter().position(|sid| *sid == 0) {

            ordered.swap(0, pos);

        }



        let mut old_to_new: HashMap<i32, i32> = HashMap::new();

        for (new_id, old_id) in ordered.iter().enumerate() {

            old_to_new.insert(*old_id, new_id as i32);

        }



        let mut states = Vec::with_capacity(ordered.len());

        for (new_id, old_id) in ordered.iter().enumerate() {

            let old_state = &self.pStateTable[*old_id as usize];

            states.push(State {

                stateId: new_id as i32,

                StateType: old_state.StateType.clone(),

                LexemeCategory: old_state.LexemeCategory.clone(),

            });

        }



        let mut edges = Vec::new();

        let mut seen: HashSet<(i32, i32, i32, String)> = HashSet::new();

        for edge in &self.pEdgeTable {

            let (Some(&from), Some(&to)) = (

                old_to_new.get(&edge.fromState),

                old_to_new.get(&edge.nextState),

            ) else {

                continue;

            };



            let dedup_key = (from, to, edge.driverId, edge.DriverType.clone());

            if seen.insert(dedup_key) {

                edges.push(Edge {

                    fromState: from,

                    nextState: to,

                    driverId: edge.driverId,

                    DriverType: edge.DriverType.clone(),

                });

            }

        }



        Graph {

            graphId: next_graph_id(),

            numOfStates: states.len() as i32,

            pEdgeTable: edges,

            pStateTable: states,

        }

    }



    /// 裁剪自动机中的不可达状态和死状态，提升效率。
///
/// Returns:
///   裁剪无用状态后的图。
/// 裁剪自动机中的不可达状态和死状态，提升效率。
///
/// Returns:
///   裁剪无用状态后的图。
fn prune_useless_states(&self) -> Graph {

        if self.pStateTable.is_empty() {

            return self.clone();

        }



        let mut reachable = HashSet::new();

        let mut queue = VecDeque::new();

        reachable.insert(0);

        queue.push_back(0);



        while let Some(curr) = queue.pop_front() {

            for edge in &self.pEdgeTable {

                if edge.fromState == curr && reachable.insert(edge.nextState) {

                    queue.push_back(edge.nextState);

                }

            }

        }



        let mut reverse_adj: Vec<Vec<i32>> = vec![Vec::new(); self.pStateTable.len()];

        let mut accept_states = Vec::new();

        for state in &self.pStateTable {

            if state.StateType == "MATCH" {

                accept_states.push(state.stateId);

            }

        }

        for edge in &self.pEdgeTable {

            if edge.nextState >= 0

                && edge.fromState >= 0

                && (edge.nextState as usize) < reverse_adj.len()

                && (edge.fromState as usize) < reverse_adj.len()

            {

                reverse_adj[edge.nextState as usize].push(edge.fromState);

            }

        }



        let mut productive = HashSet::new();

        let mut reverse_queue = VecDeque::new();

        for accept in accept_states {

            if productive.insert(accept) {

                reverse_queue.push_back(accept);

            }

        }

        while let Some(curr) = reverse_queue.pop_front() {

            if curr < 0 || curr as usize >= reverse_adj.len() {

                continue;

            }

            for prev in &reverse_adj[curr as usize] {

                if productive.insert(*prev) {

                    reverse_queue.push_back(*prev);

                }

            }

        }



        let mut keep: Vec<i32> = reachable.intersection(&productive).copied().collect();

        if keep.is_empty() {

            keep.push(0);

        }

        self.rebuild_with_state_subset(&keep)

    }



    /// 查找并合并图中行为等价的状态（相同输入、相同输出），最小化状态数。
///
/// Returns:
///   合并等价状态后的图。
/// 查找并合并图中行为等价的状态（相同输入、相同输出），最小化状态数。
///
/// Returns:
///   合并等价状态后的图。
fn merge_equivalent_states(&self) -> Graph {

        if self.pStateTable.len() <= 1 {

            return self.clone();

        }



        let mut init_groups: BTreeMap<String, Vec<i32>> = BTreeMap::new();

        for state in &self.pStateTable {

            let key = format!(

                "{}#{}",

                state.StateType == "MATCH",

                category_code(&state.LexemeCategory)

            );

            init_groups.entry(key).or_default().push(state.stateId);

        }

        let mut partitions: Vec<Vec<i32>> = init_groups.into_values().collect();

        for block in &mut partitions {

            block.sort_unstable();

        }



        loop {

            let mut state_to_block = vec![0usize; self.pStateTable.len()];

            for (block_id, block) in partitions.iter().enumerate() {

                for sid in block {

                    if *sid >= 0 && (*sid as usize) < state_to_block.len() {

                        state_to_block[*sid as usize] = block_id;

                    }

                }

            }



            let mut changed = false;

            let mut next_partitions: Vec<Vec<i32>> = Vec::new();



            for block in &partitions {

                let mut buckets: BTreeMap<String, Vec<i32>> = BTreeMap::new();

                for sid in block {

                    let signature = build_state_signature(self, *sid, &state_to_block);

                    buckets.entry(signature).or_default().push(*sid);

                }

                if buckets.len() > 1 {

                    changed = true;

                }

                for (_, mut group) in buckets {

                    group.sort_unstable();

                    next_partitions.push(group);

                }

            }



            partitions = next_partitions;

            if !changed {

                break;

            }

        }



        if let Some(pos) = partitions.iter().position(|block| block.contains(&0)) {

            partitions.swap(0, pos);

        }



        let mut old_to_new: HashMap<i32, i32> = HashMap::new();

        for (new_id, block) in partitions.iter().enumerate() {

            for sid in block {

                old_to_new.insert(*sid, new_id as i32);

            }

        }



        let mut states = Vec::with_capacity(partitions.len());

        for (new_id, block) in partitions.iter().enumerate() {

            let is_match = block.iter().any(|sid| {

                self.pStateTable

                    .get(*sid as usize)

                    .map(|s| s.StateType == "MATCH")

                    .unwrap_or(false)

            });

            let category = if is_match {

                pick_preferred_category_from_ids(block.iter().copied(), &self.pStateTable)

            } else {

                None

            };



            states.push(State {

                stateId: new_id as i32,

                StateType: if is_match {

                    "MATCH".to_string()

                } else {

                    "UNMATCH".to_string()

                },

                LexemeCategory: category,

            });

        }



        let mut edges = Vec::new();

        let mut seen: HashSet<(i32, i32, i32, String)> = HashSet::new();

        for edge in &self.pEdgeTable {

            let (Some(&from), Some(&to)) = (

                old_to_new.get(&edge.fromState),

                old_to_new.get(&edge.nextState),

            ) else {

                continue;

            };

            let dedup_key = (from, to, edge.driverId, edge.DriverType.clone());

            if seen.insert(dedup_key) {

                edges.push(Edge {

                    fromState: from,

                    nextState: to,

                    driverId: edge.driverId,

                    DriverType: edge.DriverType.clone(),

                });

            }

        }



        Graph {

            graphId: next_graph_id(),

            numOfStates: states.len() as i32,

            pEdgeTable: edges,

            pStateTable: states,

        }

    }



    /// 根据指定的 driverId 在当前状态集合上进行状态转移（move）。
///
/// Args:
///   curr: 当前状态集合。
///   driver_id: 驱动 ID。
///
/// Returns:
///   转移后的状态集合。
/// 根据指定的 driverId 在当前状态集合上进行状态转移（move）。
///
/// Args:
///   curr: 当前状态集合。
///   driver_id: 驱动 ID。
///
/// Returns:
///   转移后的状态集合。
pub fn move_by_driver(&self, curr: &HashSet<i32>, driver_id: i32) -> HashSet<i32> {

        let mut result = HashSet::new();

        for edge in &self.pEdgeTable {

            if curr.contains(&edge.fromState) && edge.driverId == driver_id {

                result.insert(edge.nextState);

            }

        }

        result

    }

    /// Move by concrete character on CHAR/CHARSET edges.

    /// 根据指定的字符在当前状态集合上进行状态转移（move）。
///
/// Args:
///   curr: 当前状态集合。
///   c: 输入的字符。
///
/// Returns:
///   转移后的状态集合。
/// 根据指定的字符在当前状态集合上进行状态转移（move）。
///
/// Args:
///   curr: 当前状态集合。
///   c: 输入的字符。
///
/// Returns:
///   转移后的状态集合。
pub fn move_by_char(&self, curr: &HashSet<i32>, c: char) -> HashSet<i32> {

        let mut result = HashSet::new();

        for edge in &self.pEdgeTable {

            if curr.contains(&edge.fromState) && edge.driverId != -1 && is_subset(c, edge.driverId)

            {

                result.insert(edge.nextState);

            }

        }

        result

    }



    /// 计算 epsilon 闭包

    /// 计算指定状态集合的 ε-闭包（所有通过 ε-边可达的状态）。
///
/// Args:
///   curr: 初始状态集合。
///
/// Returns:
///   包含 ε-闭包的状态集合。
/// 计算指定状态集合的 ε-闭包（所有通过 ε-边可达的状态）。
///
/// Args:
///   curr: 初始状态集合。
///
/// Returns:
///   包含 ε-闭包的状态集合。
pub fn epsilon_closure(&self, curr: &HashSet<i32>) -> HashSet<i32> {

        let mut result = curr.clone();

        let mut queue: VecDeque<i32> = curr.iter().cloned().collect();

        while let Some(state) = queue.pop_front() {

            for edge in &self.pEdgeTable {

                if edge.fromState == state && edge.driverId == -1 && edge.DriverType == "NULL" {

                    if result.insert(edge.nextState) {

                        queue.push_back(edge.nextState);

                    }

                }

            }

        }

        result

    }



    /// DTran 封装：在指定 driver 上 move + epsilon

    /// 封装 DTran 逻辑：组合 move_by_driver 和 epsilon_closure。
///
/// Args:
///   curr: 当前状态集合。
///   driver_id: 驱动 ID。
///
/// Returns:
///   转移后并应用 ε-闭包的状态集合。
/// 封装 DTran 逻辑：组合 move_by_driver 和 epsilon_closure。
///
/// Args:
///   curr: 当前状态集合。
///   driver_id: 驱动 ID。
///
/// Returns:
///   转移后并应用 ε-闭包的状态集合。
pub fn DTran_driver(&self, curr: &HashSet<i32>, driver_id: i32) -> HashSet<i32> {

        let moved = self.move_by_driver(curr, driver_id);

        self.epsilon_closure(&moved)

    }



    /// DTran 封装：用字符驱动 move + epsilon

    /// 封装 DTran 逻辑：组合 move_by_char 和 epsilon_closure。
///
/// Args:
///   curr: 当前状态集合。
///   c: 输入的字符。
///
/// Returns:
///   转移后并应用 ε-闭包的状态集合。
/// 封装 DTran 逻辑：组合 move_by_char 和 epsilon_closure。
///
/// Args:
///   curr: 当前状态集合。
///   c: 输入的字符。
///
/// Returns:
///   转移后并应用 ε-闭包的状态集合。
pub fn DTran_char(&self, curr: &HashSet<i32>, c: char) -> HashSet<i32> {

        let moved = self.move_by_char(curr, c);

        self.epsilon_closure(&moved)

    }



    /// 经典 subset construction 将 NFA 转为 DFA

    /// 将当前的 NFA (非确定性有限自动机) 转化为 DFA (确定性有限自动机)

    /// 使用的是经典的子集构造法 (Subset Construction Algorithm)

    /// 使用子集构造法将当前的 NFA 转换为 DFA。
///
/// Returns:
///   转换后的 DFA 图。
/// 使用子集构造法将当前的 NFA 转换为 DFA。
///
/// Returns:
///   转换后的 DFA 图。
pub fn NFA_to_DFA(&self) -> Graph {

        let mut node_list = Vec::new();

        let mut edge_list = Vec::new();

        let mut dfa_state_sets: Vec<HashSet<i32>> = Vec::new();

        let mut queue: VecDeque<HashSet<i32>> = VecDeque::new();

        let mut start_set = HashSet::new();

        start_set.insert(0);

        start_set = self.epsilon_closure(&start_set);

        dfa_state_sets.push(start_set.clone());

        queue.push_back(start_set.clone());



        while let Some(curr_set) = queue.pop_front() {

            let curr_state_id = dfa_state_sets

                .iter()

                .position(|set| *set == curr_set)

                .unwrap();



            let mut just_char_set = HashSet::new();

            let mut diff_char_set = HashSet::new();



            for edge in &self.pEdgeTable {

                if curr_set.contains(&edge.fromState) && edge.driverId != -1 {

                    if edge.DriverType == "CHARSET" {

                        diff_char_set.insert(edge.driverId);

                    } else if edge.DriverType == "CHAR" {

                        just_char_set.insert(edge.driverId);

                    }

                }

            }



            // 单字符驱动转移：

            for driver_id in just_char_set {

                let mut c = '?';

                for segment in segments_of(driver_id) {

                    c = segment.fromChar;

                    break;

                }

                let next_set = self.DTran_char(&curr_set, c);

                handle_state_transition(

                    &mut dfa_state_sets,

                    &mut queue,

                    &mut edge_list,

                    curr_state_id as i32,

                    driver_id,

                    "CHAR",

                    next_set,

                );

            }



            for driver_id in diff_char_set {

                let next_set = self.DTran_driver(&curr_set, driver_id);

                handle_state_transition(

                    &mut dfa_state_sets,

                    &mut queue,

                    &mut edge_list,

                    curr_state_id as i32,

                    driver_id,

                    "CHARSET",

                    next_set,

                );

            }

        }



        // 根据状态集合生成 DFA 状态信息：

        // - 只要集合内存在 MATCH 态，DFA 态初步判定为 MATCH；        // - 类别冲突时优先选择“非 ID 类别”；

        for (idx, state_set) in dfa_state_sets.iter().enumerate() {

            let contains_match = state_set.iter().any(|state_id| {

                self.pStateTable

                    .get(*state_id as usize)

                    .map(|s| s.StateType == "MATCH")

                    .unwrap_or(false)

            });



            let mut match_type = if contains_match {

                "MATCH".to_string()

            } else {

                "UNMATCH".to_string()

            };

            let category = if contains_match {

                pick_preferred_category_from_ids(state_set.iter().copied(), &self.pStateTable)

            } else {

                None

            };



            if category.is_none() && match_type == "MATCH" {

                match_type = "UNMATCH".to_string();

            }



            node_list.push(State {

                stateId: idx as i32,

                StateType: match_type,

                LexemeCategory: category,

            });

        }



        Graph {

            graphId: next_graph_id(),

            numOfStates: node_list.len() as i32,

            pEdgeTable: edge_list,

            pStateTable: node_list,

        }

    }



    /// 对长文本进行 DFA 扫描

    /// 将输入的长字符串通过当前 DFA 识别并解析出一系列的词法单元（Tokens）

    pub fn long_text_search(&self, text: &str) -> Vec<Token> {

        let mut result = Vec::new();

        let mut next_state = 0;

        let mut buffer = String::new();



        for c in text.chars() {

            let mut has_next = false;

            for edge in &self.pEdgeTable {

                if edge.fromState == next_state

                    && edge.driverId != -1

                    && has_contain_char(edge.driverId, c)

                {

                    next_state = edge.nextState;

                    has_next = true;

                    break;

                }

            }



            if !has_next {

                if let Some(token) = self.build_token(next_state, &buffer) {

                    result.push(token);

                }

                buffer.clear();

                buffer.push(c);

                next_state = 0;

                for edge in &self.pEdgeTable {

                    if edge.fromState == next_state

                        && edge.driverId != -1

                        && has_contain_char(edge.driverId, c)

                    {

                        next_state = edge.nextState;

                        break;

                    }

                }

            } else {

                buffer.push(c);

            }

        }



        if let Some(token) = self.build_token(next_state, &buffer) {

            result.push(token);

        }



        result

    }

    /// Get lexeme category for a given input.

    pub fn get_lexeme_category(&self, text: &str) -> Option<LexemeCategory> {

        let mut curr_state = 0;

        for c in text.chars() {

            let mut jumped = false;

            for edge in &self.pEdgeTable {

                if edge.fromState == curr_state

                    && edge.driverId != -1

                    && has_contain_char(edge.driverId, c)

                {

                    curr_state = edge.nextState;

                    jumped = true;

                    break;

                }

            }

            if !jumped {

                return None;

            }

        }

        self.pStateTable[curr_state as usize].LexemeCategory.clone()

    }



    /// 根据当前状态生成 token

    fn build_token(&self, state_idx: i32, buffer: &str) -> Option<Token> {

        if state_idx < 0 || state_idx as usize >= self.pStateTable.len() {

            return None;

        }

        let state = &self.pStateTable[state_idx as usize];

        let category = state.LexemeCategory.clone()?;

        if category == LexemeCategory::SPACE_CONST || category == LexemeCategory::NOTE {

            return None;

        }



        let mut token = Token {

            lexeme_category: category.clone(),

            symbol_type: "TERMINAL".to_string(),

            identify: None,

            value: None,

        };



        // 依据类别填充 token 的附加字段：

        if category == LexemeCategory::ID {

            token.identify = Some(buffer.to_string());

        } else if category == LexemeCategory::INTEGER_CONST {

            token.value = buffer.parse::<i64>().ok();

        } else if category == LexemeCategory::KEYWORD {

            token.identify = Some(buffer.to_string());

        } else if category == LexemeCategory::COMPARE_OPERATOR 
               || category == LexemeCategory::LOGIC_OPERATOR 
               || category == LexemeCategory::NUMERIC_OPERATOR 
               || category == LexemeCategory::ASSIGN_OPERATOR {
            token.identify = Some(buffer.to_string());
        }



        Some(token)

    }

}



/// 将 NFA 状态集合转化为 DFA 边与节点

fn handle_state_transition(

    dfa_state_sets: &mut Vec<HashSet<i32>>,

    queue: &mut VecDeque<HashSet<i32>>,

    edge_list: &mut Vec<Edge>,

    curr_state_id: i32,

    driver_id: i32,

    driver_type: &str,

    next_state_set: HashSet<i32>,

) {

    if next_state_set.is_empty() {

        return;

    }

    if let Some(pos) = dfa_state_sets.iter().position(|set| *set == next_state_set) {

        edge_list.push(Edge {

            fromState: curr_state_id,

            nextState: pos as i32,

            driverId: driver_id,

            DriverType: driver_type.to_string(),

        });

    } else {

        dfa_state_sets.push(next_state_set.clone());

        queue.push_back(next_state_set.clone());

        let new_id = (dfa_state_sets.len() - 1) as i32;

        edge_list.push(Edge {

            fromState: curr_state_id,

            nextState: new_id,

            driverId: driver_id,

            DriverType: driver_type.to_string(),

        });

    }

}

/// Shift state/edge ids by offset.

fn shift_graph(graph: &mut Graph, offset: i32) {

    for state in &mut graph.pStateTable {

        state.stateId += offset;

    }

    for edge in &mut graph.pEdgeTable {

        edge.fromState += offset;

        edge.nextState += offset;

    }

}



/// 创建一个只有起止两状态的基础 NFA

/// 构建一个基础 NFA: 例如单个字符或单个词汇的识别

/// 创建一个基础的 NFA（通常用于匹配单个字符或字符集）。
///
/// Args:
///   driverType: 驱动类型（如 CHAR 或 CHARSET）。
///   driverId: 驱动 ID（字符集 ID 或字符 Unicode 值）。
///   category: 可选的词法类别。
///
/// Returns:
///   构建的基础 NFA 图。
/// 创建一个基础的 NFA（通常用于匹配单个字符或字符集）。
///
/// Args:
///   driverType: 驱动类型（如 CHAR 或 CHARSET）。
///   driverId: 驱动 ID（字符集 ID 或字符 Unicode 值）。
///   category: 可选的词法类别。
///
/// Returns:
///   构建的基础 NFA 图。
pub fn generateBasicNFA(

    driverType: &str,

    driverId: i32,

    category: Option<LexemeCategory>,

) -> Graph {

    // 结构固定：    //   state 0(UNMATCH) --driver--> state 1(MATCH, category)

    let mut graph = Graph::new(2);

    graph.pStateTable.push(State {

        stateId: 0,

        StateType: "UNMATCH".to_string(),

        LexemeCategory: None,

    });

    graph.pStateTable.push(State {

        stateId: 1,

        StateType: "MATCH".to_string(),

        LexemeCategory: category,

    });

    graph.pEdgeTable.push(Edge {

        fromState: 0,

        nextState: 1,

        driverId,

        DriverType: driverType.to_string(),

    });

    graph

}

/// Union of two NFAs.

/// 对两个 NFA 进行并集（|）运算。
///
/// Args:
///   g1: 第一个 NFA。
///   g2: 第二个 NFA。
///
/// Returns:
///   进行并集运算后的新 NFA。
/// 对两个 NFA 进行并集（|）运算。
///
/// Args:
///   g1: 第一个 NFA。
///   g2: 第二个 NFA。
///
/// Returns:
///   进行并集运算后的新 NFA。
pub fn union(mut g1: Graph, mut g2: Graph) -> Graph {

    let g1_count = g1.pStateTable.len() as i32;

    shift_graph(&mut g1, 1);

    shift_graph(&mut g2, 1 + g1_count);

    let g2_start_id = 1 + g1_count;



    let mut states = Vec::new();

    states.push(State {

        stateId: 0,

        StateType: "UNMATCH".to_string(),

        LexemeCategory: None,

    });

    states.extend(g1.pStateTable.clone());

    states.extend(g2.pStateTable.clone());

    let accept_id = states.len() as i32;

    states.push(State {

        stateId: accept_id,

        StateType: "MATCH".to_string(),

        LexemeCategory: None,

    });



    let mut edges = Vec::new();

    edges.extend(g1.pEdgeTable.clone());

    edges.extend(g2.pEdgeTable.clone());

    edges.push(Edge {

        fromState: 0,

        nextState: 1,

        driverId: -1,

        DriverType: "NULL".to_string(),

    });

    edges.push(Edge {

        fromState: 0,

        nextState: g2_start_id,

        driverId: -1,

        DriverType: "NULL".to_string(),

    });



    let mut match_ids = Vec::new();

    for state in states.iter() {

        if state.stateId != 0 && state.stateId != accept_id && state.StateType == "MATCH" {

            match_ids.push(state.stateId);

        }

    }

    for id in match_ids.iter() {

        edges.push(Edge {

            fromState: *id,

            nextState: accept_id,

            driverId: -1,

            DriverType: "NULL".to_string(),

        });

    }

    for state in states.iter_mut() {

        if match_ids.contains(&state.stateId) {

            state.StateType = "UNMATCH".to_string();

        }

    }



    Graph {

        graphId: next_graph_id(),

        numOfStates: states.len() as i32,

        pEdgeTable: edges,

        pStateTable: states,

    }

}

/// Concatenate two NFAs.

/// 对两个 NFA 进行连接（串联）运算。
///
/// Args:
///   g1: 第一个 NFA。
///   g2: 第二个 NFA。
///
/// Returns:
///   进行连接运算后的新 NFA。
/// 对两个 NFA 进行连接（串联）运算。
///
/// Args:
///   g1: 第一个 NFA。
///   g2: 第二个 NFA。
///
/// Returns:
///   进行连接运算后的新 NFA。
pub fn product(g1: Graph, mut g2: Graph) -> Graph {

    let g1_count = g1.pStateTable.len() as i32;

    shift_graph(&mut g2, g1_count);

    let g2_start_id = g1_count;



    let mut states = g1.pStateTable.clone();

    states.extend(g2.pStateTable.clone());



    let mut edges = g1.pEdgeTable.clone();

    edges.extend(g2.pEdgeTable.clone());



    for state in states.iter_mut() {

        if state.stateId < g2_start_id && state.StateType == "MATCH" {

            state.StateType = "UNMATCH".to_string();

            state.LexemeCategory = None;

            edges.push(Edge {

                fromState: state.stateId,

                nextState: g2_start_id,

                driverId: -1,

                DriverType: "NULL".to_string(),

            });

        }

    }



    Graph {

        graphId: next_graph_id(),

        numOfStates: states.len() as i32,

        pEdgeTable: edges,

        pStateTable: states,

    }

}

/// One-or-more closure.

/// 对 NFA 进行正闭包（+，匹配一次或多次）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行正闭包运算后的新 NFA。
/// 对 NFA 进行正闭包（+，匹配一次或多次）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行正闭包运算后的新 NFA。
pub fn plusClosure(mut g: Graph) -> Graph {

    let start_id = g.pStateTable.first().map(|s| s.stateId).unwrap_or(0);

    let match_ids: Vec<i32> = g

        .pStateTable

        .iter()

        .filter(|s| s.StateType == "MATCH")

        .map(|s| s.stateId)

        .collect();

    for id in match_ids {

        g.pEdgeTable.push(Edge {

            fromState: id,

            nextState: start_id,

            driverId: -1,

            DriverType: "NULL".to_string(),

        });

    }

    g.graphId = next_graph_id();

    g.numOfStates = g.pStateTable.len() as i32;

    g

}



fn add_epsilon_edge(edges: &mut Vec<Edge>, from_state: i32, to_state: i32) {

    edges.push(Edge {

        fromState: from_state,

        nextState: to_state,

        driverId: -1,

        DriverType: "NULL".to_string(),

    });

}

/// 检查指定的节点是否存在入边。
///
/// Args:
///   edges: 图的所有边集合。
///   state_id: 目标状态 ID。
///
/// Returns:
///   如果存在入边返回 true，否则返回 false。
/// 检查指定的节点是否存在入边。
///
/// Args:
///   edges: 图的所有边集合。
///   state_id: 目标状态 ID。
///
/// Returns:
///   如果存在入边返回 true，否则返回 false。
fn has_incoming_edge(edges: &[Edge], state_id: i32) -> bool {

    edges

        .iter()

        .any(|edge| edge.nextState == state_id && edge.fromState != state_id)

}

/// 检查指定的节点是否存在出边。
///
/// Args:
///   edges: 图的所有边集合。
///   state_id: 目标状态 ID。
///
/// Returns:
///   如果存在出边返回 true，否则返回 false。
/// 检查指定的节点是否存在出边。
///
/// Args:
///   edges: 图的所有边集合。
///   state_id: 目标状态 ID。
///
/// Returns:
///   如果存在出边返回 true，否则返回 false。
fn has_outgoing_edge(edges: &[Edge], state_id: i32) -> bool {

    edges.iter().any(|edge| edge.fromState == state_id)

}

/// 使用保守的 Thompson 构造法计算 Kleene 闭包（*），避免图结构的冗余合并。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行保守闭包运算后的新 NFA。
/// 使用保守的 Thompson 构造法计算 Kleene 闭包（*），避免图结构的冗余合并。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行保守闭包运算后的新 NFA。
fn closure_conservative(mut g: Graph) -> Graph {

    shift_graph(&mut g, 1);

    let mut states = Vec::new();

    states.push(State {

        stateId: 0,

        StateType: "UNMATCH".to_string(),

        LexemeCategory: None,

    });

    states.extend(g.pStateTable.clone());

    let accept_id = states.len() as i32;

    states.push(State {

        stateId: accept_id,

        StateType: "MATCH".to_string(),

        LexemeCategory: None,

    });

    let mut edges = g.pEdgeTable.clone();

    add_epsilon_edge(&mut edges, 0, 1);

    add_epsilon_edge(&mut edges, 0, accept_id);

    let match_ids: Vec<i32> = states

        .iter()

        .filter(|s| s.stateId != 0 && s.stateId != accept_id && s.StateType == "MATCH")

        .map(|s| s.stateId)

        .collect();

    for id in &match_ids {

        add_epsilon_edge(&mut edges, *id, 1);

        add_epsilon_edge(&mut edges, *id, accept_id);

    }

    for state in states.iter_mut() {

        if match_ids.contains(&state.stateId) {

            state.StateType = "UNMATCH".to_string();

        }

    }

    Graph {

        graphId: next_graph_id(),

        numOfStates: states.len() as i32,

        pEdgeTable: edges,

        pStateTable: states,

    }

}



/// Kleene 闭包：零次或多次

/// 对 NFA 进行 Kleene 闭包（*，匹配零次或多次）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行闭包运算后的新 NFA。
/// 对 NFA 进行 Kleene 闭包（*，匹配零次或多次）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   进行闭包运算后的新 NFA。
pub fn closure(g: Graph) -> Graph {

    if g.pStateTable.is_empty() {

        return g;

    }

    let start_id = g.pStateTable.first().map(|s| s.stateId).unwrap_or(0);

    let match_ids: Vec<i32> = g

        .pStateTable

        .iter()

        .filter(|s| s.StateType == "MATCH")

        .map(|s| s.stateId)

        .collect();

    // 图 2.16：判断复用起始状态

    if match_ids.len() != 1 {

        return closure_conservative(g);

    }

    let old_accept = match_ids[0];

    let start_has_in = has_incoming_edge(&g.pEdgeTable, start_id);

    let accept_has_out = has_outgoing_edge(&g.pEdgeTable, old_accept);

    // 情况 1：入度 + 出度 -> 保守处理

    if start_has_in && accept_has_out {

        return closure_conservative(g);

    }

    // 情况 2：入度无出度 -> 平移处理

    if start_has_in && !accept_has_out {

        let mut shifted = g;

        shift_graph(&mut shifted, 1);

        let shifted_start = start_id + 1;

        let shifted_accept = old_accept + 1;

        let mut states = Vec::new();

        states.push(State {

            stateId: 0,

            StateType: "UNMATCH".to_string(),

            LexemeCategory: None,

        });

        states.extend(shifted.pStateTable.clone());

        let mut edges = shifted.pEdgeTable.clone();

        add_epsilon_edge(&mut edges, 0, shifted_start);

        add_epsilon_edge(&mut edges, 0, shifted_accept);

        add_epsilon_edge(&mut edges, shifted_accept, shifted_start);

        return Graph {

            graphId: next_graph_id(),

            numOfStates: states.len() as i32,

            pEdgeTable: edges,

            pStateTable: states,

        };

    }

    // 情况 3：出度无入度 -> 新建处理

    if !start_has_in && accept_has_out {

        let mut states = g.pStateTable.clone();

        for state in states.iter_mut() {

            if state.stateId == old_accept {

                state.StateType = "UNMATCH".to_string();

            }

        }

        let new_accept = states.iter().map(|state| state.stateId).max().unwrap_or(-1) + 1;

        states.push(State {

            stateId: new_accept,

            StateType: "MATCH".to_string(),

            LexemeCategory: None,

        });

        let mut edges = g.pEdgeTable.clone();

        add_epsilon_edge(&mut edges, start_id, new_accept);

        add_epsilon_edge(&mut edges, old_accept, start_id);

        add_epsilon_edge(&mut edges, old_accept, new_accept);

        return Graph {

            graphId: next_graph_id(),

            numOfStates: states.len() as i32,

            pEdgeTable: edges,

            pStateTable: states,

        };

    }

    // 情况 4：无出度无入度 -> 直接连接处理

    let states = g.pStateTable.clone();

    let mut edges = g.pEdgeTable.clone();

    add_epsilon_edge(&mut edges, start_id, old_accept);

    add_epsilon_edge(&mut edges, old_accept, start_id);

    Graph {

        graphId: next_graph_id(),

        numOfStates: states.len() as i32,

        pEdgeTable: edges,

        pStateTable: states,

    }

}

/// Zero-or-one operator.

/// 对 NFA 进行零次或一次（?）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   运算后的新 NFA。
/// 对 NFA 进行零次或一次（?）运算。
///
/// Args:
///   g: 目标 NFA。
///
/// Returns:
///   运算后的新 NFA。
pub fn zeroOrOne(mut g: Graph) -> Graph {

    shift_graph(&mut g, 1);

    let mut states = Vec::new();

    states.push(State {

        stateId: 0,

        StateType: "UNMATCH".to_string(),

        LexemeCategory: None,

    });

    states.extend(g.pStateTable.clone());

    let accept_id = states.len() as i32;

    states.push(State {

        stateId: accept_id,

        StateType: "MATCH".to_string(),

        LexemeCategory: None,

    });



    let mut edges = g.pEdgeTable.clone();

    edges.push(Edge {

        fromState: 0,

        nextState: 1,

        driverId: -1,

        DriverType: "NULL".to_string(),

    });

    edges.push(Edge {

        fromState: 0,

        nextState: accept_id,

        driverId: -1,

        DriverType: "NULL".to_string(),

    });



    let match_ids: Vec<i32> = states

        .iter()

        .filter(|s| s.stateId != 0 && s.stateId != accept_id && s.StateType == "MATCH")

        .map(|s| s.stateId)

        .collect();

    for id in &match_ids {

        edges.push(Edge {

            fromState: *id,

            nextState: accept_id,

            driverId: -1,

            DriverType: "NULL".to_string(),

        });

    }

    for state in states.iter_mut() {

        if match_ids.contains(&state.stateId) {

            state.StateType = "UNMATCH".to_string();

        }

    }



    Graph {

        graphId: next_graph_id(),

        numOfStates: states.len() as i32,

        pEdgeTable: edges,

        pStateTable: states,

    }

}

