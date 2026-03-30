#![allow(non_snake_case)]

use once_cell::sync::Lazy;
use std::collections::{HashSet, VecDeque};
use std::sync::Mutex;

use super::char_set::{has_contain_char, is_subset, segments_of};
use super::category::LexemeCategory;
use super::edge::Edge;
use super::state::State;
use super::token::Token;

/// 图结构，既可表示 NFA 也可表示 DFA
#[derive(Clone, Debug)]
pub struct Graph {
    pub graphId: i32,
    pub numOfStates: i32,
    pub pEdgeTable: Vec<Edge>,
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
    pub fn move_by_driver(&self, curr: &HashSet<i32>, driver_id: i32) -> HashSet<i32> {
        let mut result = HashSet::new();
        for edge in &self.pEdgeTable {
            if curr.contains(&edge.fromState) && edge.driverId == driver_id {
                result.insert(edge.nextState);
            }
        }
        result
    }

    /// 用实际字符触发 move，匹配 CHARSET 边
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
    pub fn DTran_driver(&self, curr: &HashSet<i32>, driver_id: i32) -> HashSet<i32> {
        let moved = self.move_by_driver(curr, driver_id);
        self.epsilon_closure(&moved)
    }

    /// DTran 封装：用字符驱动 move + epsilon
    pub fn DTran_char(&self, curr: &HashSet<i32>, c: char) -> HashSet<i32> {
        let moved = self.move_by_char(curr, c);
        self.epsilon_closure(&moved)
    }

    /// 经典 subset construction 将 NFA 转为 DFA
    pub fn NFA_to_DFA(&self) -> Graph {
        let mut node_list = Vec::new();
        let mut edge_list = Vec::new();
        let mut dfa_state_sets: Vec<HashSet<i32>> = Vec::new();
        let mut queue: VecDeque<HashSet<i32>> = VecDeque::new();

        // 先将 NFA 起点做 epsilon 闭包
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

            // 收集所有可用驱动，区分单字符和字符集
            for edge in &self.pEdgeTable {
                if curr_set.contains(&edge.fromState) && edge.driverId != -1 {
                    if edge.DriverType == "CHARSET" {
                        diff_char_set.insert(edge.driverId);
                    } else if edge.DriverType == "CHAR" {
                        just_char_set.insert(edge.driverId);
                    }
                }
            }

            // 单字符驱动转移
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

            // 字符集驱动转移
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

        // 根据状态集合生成 DFA 状态信息
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
            let mut category: Option<LexemeCategory> = None;

            for state in state_set {
                let st = &self.pStateTable[*state as usize];
                if let Some(state_category) = &st.LexemeCategory {
                    if *state_category != LexemeCategory::ID {
                        category = Some(state_category.clone());
                        break;
                    } else if category.is_none() {
                        category = Some(LexemeCategory::ID);
                    }
                }
            }

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
    pub fn long_text_search(&self, text: &str) -> Vec<Token> {
        let mut result = Vec::new();
        let mut next_state = 0;
        let mut buffer = String::new();

        for c in text.chars() {
            let mut has_next = false;
            // 检查是否存在可走的边
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
                // 从初态重新尝试匹配
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

    /// 获取某个词素的类别
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

        if category == LexemeCategory::ID {
            token.identify = Some(buffer.to_string());
        } else if category == LexemeCategory::INTEGER_CONST {
            token.value = buffer.parse::<i64>().ok();
        } else if category == LexemeCategory::KEYWORD {
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

/// 为组合操作准备：整体平移图结构
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
pub fn generateBasicNFA(
    driverType: &str,
    driverId: i32,
    category: Option<LexemeCategory>,
) -> Graph {
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

/// 构造两个图的并集，增加新起止
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

/// 图的串联：将 g1 的终态指向 g2 的始态
pub fn product(mut g1: Graph, mut g2: Graph) -> Graph {
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

/// 正闭包：至少一次
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

/// Kleene 闭包：零次或多次
pub fn closure(mut g: Graph) -> Graph {
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
            nextState: 1,
            driverId: -1,
            DriverType: "NULL".to_string(),
        });
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

/// 可选：零次或一次
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
