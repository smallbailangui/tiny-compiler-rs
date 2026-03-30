#![allow(non_camel_case_types)]

/// 词法类别枚举。
///
/// 设计目的：
/// 1. 避免使用自由字符串导致的拼写错误与分类不一致；
/// 2. 让 NFA/DFA 状态上的类别信息具备强类型约束；
/// 3. 便于后续扩展（例如新增 token 类型时，由编译器提示所有受影响分支）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexemeCategory {
    /// 整数常量（示例：`0`、`42`）
    INTEGER_CONST,
    /// 浮点常量（示例：`3.14`）
    FLOAT_CONST,
    /// 科学计数法常量（示例：`1.2e-3`）
    SCIENTIFIC_CONST,
    /// 数值运算符（`+ - * /`）
    NUMERIC_OPERATOR,
    /// 注释 token
    NOTE,
    /// 字符串常量（示例：`"hello"`）
    STRING_CONST,
    /// 空白符（当前主要用于过滤，不向上层输出）
    SPACE_CONST,
    /// 比较运算符（示例：`<`、`=`）
    COMPARE_OPERATOR,
    /// 标识符（变量名等）
    ID,
    /// 逻辑/结构符号（当前实现中包含部分界符）
    LOGIC_OPERATOR,
    /// 关键字（示例：`if`、`read`）
    KEYWORD,
    /// 赋值运算符（`:=`）
    ASSIGN_OPERATOR,
}
