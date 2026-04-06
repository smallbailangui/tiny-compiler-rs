#![allow(non_snake_case)]
#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::char;
use std::cmp::{max, min};
use std::sync::Mutex;

/// 字符集分段结构。
/// 同一个 `indexId` 可能由多个区间段（`segementId`）组成。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharSet {
    pub indexId: i32,
    pub segementId: i32,
    pub fromChar: char,
    pub toChar: char,
}

/// 全局字符集表：保存所有字符集的分段数据。
static P_CHAR_SET_TABLE: Lazy<Mutex<Vec<CharSet>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// 字符集 id 自增游标。
static CURR_CHAR_SET_INDEX: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(1));

/// 生成新的字符集 id。
fn next_char_set_index() -> i32 {
    let mut guard = CURR_CHAR_SET_INDEX.lock().unwrap();
    let next = *guard;
    *guard += 1;
    next
}

/// 向全局表追加一个区间段。
fn push_segment(index: i32, segement_id: i32, from_char: char, to_char: char) {
    let mut table = P_CHAR_SET_TABLE.lock().unwrap();
    table.push(CharSet {
        indexId: index,
        segementId: segement_id,
        fromChar: from_char,
        toChar: to_char,
    });
}

/// 读取某个字符集 id 的全部区间段。
pub fn segments_of(char_set_id: i32) -> Vec<CharSet> {
    let table = P_CHAR_SET_TABLE.lock().unwrap();
    table
        .iter()
        .filter(|c| c.indexId == char_set_id)
        .cloned()
        .collect()
}

/// 判断字符 `c` 是否属于 `driver_id` 对应的字符集。
pub fn is_subset(c: char, driver_id: i32) -> bool {
    segments_of(driver_id)
        .into_iter()
        .any(|seg| c >= seg.fromChar && c <= seg.toChar)
}

/// 与 `is_subset` 等价，保留旧命名用于兼容调用方。
pub fn has_contain_char(char_set_id: i32, c: char) -> bool {
    is_subset(c, char_set_id)
}

/// 获取字符的整数索引（Unicode code point）。
pub fn get_index(c: char) -> i32 {
    c as i32
}

/// 创建闭区间字符集 `[from_char, to_char]`。
pub fn range(from_char: char, to_char: char) -> i32 {
    let index = next_char_set_index();
    push_segment(index, 1, from_char, to_char);
    index
}

/// 两个单字符求并，返回新字符集 id。
pub fn union_chars(c1: char, c2: char) -> i32 {
    let mut a = c1 as u32;
    let mut b = c2 as u32;
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    let index = next_char_set_index();
    if a == b {
        push_segment(
            index,
            1,
            char::from_u32(a).unwrap(),
            char::from_u32(a).unwrap(),
        );
    } else if b == a + 1 {
        push_segment(
            index,
            1,
            char::from_u32(a).unwrap(),
            char::from_u32(b).unwrap(),
        );
    } else {
        push_segment(
            index,
            1,
            char::from_u32(a).unwrap(),
            char::from_u32(a).unwrap(),
        );
        push_segment(
            index,
            2,
            char::from_u32(b).unwrap(),
            char::from_u32(b).unwrap(),
        );
    }

    index
}

/// 字符集与单字符求并。
pub fn union_charset_char(char_set_id: i32, c: char) -> i32 {
    let mut segments = Vec::new();
    let mut is_contain = false;
    let mut left_next = false;
    let mut right_next = false;

    // 扫描原字符集，记录包含/相邻信息。
    for seg in segments_of(char_set_id) {
        if seg.fromChar <= c && seg.toChar >= c {
            is_contain = true;
        }
        if (seg.fromChar as u32).wrapping_sub(1) == c as u32 {
            right_next = true;
        }
        if (seg.toChar as u32) + 1 == c as u32 {
            left_next = true;
        }
        segments.push(seg);
    }

    let new_index = next_char_set_index();
    let mut segement_id = 1;

    // 已包含：直接复制原分段。
    if is_contain {
        for seg in segments {
            push_segment(new_index, segement_id, seg.fromChar, seg.toChar);
            segement_id += 1;
        }
        return new_index;
    }

    if left_next && right_next {
        // 左右都相邻：尝试桥接并合并。
        segments.sort_by(|a, b| a.fromChar.cmp(&b.fromChar));
        let mut idx = 0;
        while idx < segments.len() {
            if (segments[idx].toChar as u32) + 1 == c as u32
                && idx + 1 < segments.len()
                && (segments[idx + 1].fromChar as u32) == c as u32 + 1
            {
                push_segment(
                    new_index,
                    segement_id,
                    segments[idx].fromChar,
                    segments[idx + 1].toChar,
                );
                segement_id += 1;
                idx += 2;
            } else {
                push_segment(
                    new_index,
                    segement_id,
                    segments[idx].fromChar,
                    segments[idx].toChar,
                );
                segement_id += 1;
                idx += 1;
            }
        }
    } else if left_next {
        // 只与左侧相邻：扩展左侧段右边界。
        for mut seg in segments {
            if (seg.toChar as u32) + 1 == c as u32 {
                seg.toChar = c;
            }
            push_segment(new_index, segement_id, seg.fromChar, seg.toChar);
            segement_id += 1;
        }
    } else if right_next {
        // 只与右侧相邻：扩展右侧段左边界。
        for mut seg in segments {
            if (seg.fromChar as u32).wrapping_sub(1) == c as u32 {
                seg.fromChar = c;
            }
            push_segment(new_index, segement_id, seg.fromChar, seg.toChar);
            segement_id += 1;
        }
    } else {
        // 与任何段都不相邻：新增独立单点段。
        for seg in segments {
            push_segment(new_index, segement_id, seg.fromChar, seg.toChar);
            segement_id += 1;
        }
        push_segment(new_index, segement_id, c, c);
    }

    new_index
}

/// 字符集并运算：`char_set_id1 ∪ char_set_id2`。
pub fn union_charsets(char_set_id1: i32, char_set_id2: i32) -> i32 {
    let mut merged = segments_of(char_set_id1);
    merged.extend(segments_of(char_set_id2));

    // 先按区间起点排序，再线性合并。
    merged.sort_by(|a, b| match a.fromChar.cmp(&b.fromChar) {
        std::cmp::Ordering::Equal => a.toChar.cmp(&b.toChar),
        other => other,
    });

    let new_index = next_char_set_index();
    let mut segement_id = 1;
    if merged.is_empty() {
        return new_index;
    }

    let mut current = merged[0].clone();
    for seg in merged.into_iter().skip(1) {
        if seg.fromChar <= char::from_u32(current.toChar as u32 + 1).unwrap_or(current.toChar) {
            current.toChar = max(current.toChar, seg.toChar);
        } else {
            push_segment(new_index, segement_id, current.fromChar, current.toChar);
            segement_id += 1;
            current = seg;
        }
    }
    push_segment(new_index, segement_id, current.fromChar, current.toChar);

    new_index
}

/// 字符集差运算：`char_set_id1 - char_set_id2`。
pub fn difference_charsets(char_set_id1: i32, char_set_id2: i32) -> i32 {
    let list1 = segments_of(char_set_id1);
    let list2 = segments_of(char_set_id2);
    let mut idx2 = 0;
    let mut segement_id = 1;
    let new_index = next_char_set_index();

    for seg1 in list1 {
        let mut start1 = seg1.fromChar as u32;
        let end1 = seg1.toChar as u32;

        // 跳过所有在 seg1 左侧且不相交的 seg2。
        while idx2 < list2.len() && (list2[idx2].toChar as u32) < start1 {
            idx2 += 1;
        }

        let mut j = idx2;
        let mut covered = false;
        while j < list2.len() && (list2[j].fromChar as u32) <= end1 {
            let start2 = list2[j].fromChar as u32;
            let end2 = list2[j].toChar as u32;
            let overlap_start = max(start1, start2);
            let overlap_end = min(end1, end2);

            // 先写入重叠前的未覆盖部分。
            if overlap_start > start1 {
                push_segment(
                    new_index,
                    segement_id,
                    char::from_u32(start1).unwrap(),
                    char::from_u32(overlap_start - 1).unwrap(),
                );
                segement_id += 1;
            }

            start1 = overlap_end + 1;
            if start1 > end1 {
                covered = true;
                break;
            }
            j += 1;
        }

        // 尾部剩余区间。
        if !covered && start1 <= end1 {
            push_segment(
                new_index,
                segement_id,
                char::from_u32(start1).unwrap(),
                char::from_u32(end1).unwrap(),
            );
            segement_id += 1;
        }
    }

    new_index
}

/// 字符集与单字符的差：`char_set_id - {c}`。
pub fn difference_charset_char(char_set_id: i32, c: char) -> i32 {
    let mut segement_id = 1;
    let new_index = next_char_set_index();

    for seg in segments_of(char_set_id) {
        if seg.toChar < c || seg.fromChar > c {
            // 不相交，直接复制。
            push_segment(new_index, segement_id, seg.fromChar, seg.toChar);
            segement_id += 1;
            continue;
        }

        if seg.fromChar == seg.toChar {
            // 单点段且命中 c，删除后为空，不写入。
            continue;
        }

        if seg.toChar == c {
            push_segment(
                new_index,
                segement_id,
                seg.fromChar,
                char::from_u32(seg.toChar as u32 - 1).unwrap(),
            );
            segement_id += 1;
        } else if seg.fromChar == c {
            push_segment(
                new_index,
                segement_id,
                char::from_u32(seg.fromChar as u32 + 1).unwrap(),
                seg.toChar,
            );
            segement_id += 1;
        } else {
            // c 在中间：拆分成左右两段。
            push_segment(
                new_index,
                segement_id,
                seg.fromChar,
                char::from_u32(c as u32 - 1).unwrap(),
            );
            segement_id += 1;
            push_segment(
                new_index,
                segement_id,
                char::from_u32(c as u32 + 1).unwrap(),
                seg.toChar,
            );
            segement_id += 1;
        }
    }

    new_index
}

/// 打印单个字符集分段。
pub fn show_char_set(char_set: &CharSet) {
    println!(
        "indexId: {} segementId: {} fromChar: {} toChar: {}",
        char_set.indexId, char_set.segementId, char_set.fromChar, char_set.toChar
    );
}

/// 打印全局字符集表。
pub fn show_char_set_table() {
    let table = P_CHAR_SET_TABLE.lock().unwrap();
    println!("\n字符集表内容 (total: {})", table.len());
    for set in table.iter() {
        show_char_set(set);
    }
}

/// 清空字符集表并重置 id 游标。
pub fn reset_char_sets() {
    P_CHAR_SET_TABLE.lock().unwrap().clear();
    *CURR_CHAR_SET_INDEX.lock().unwrap() = 1;
}

