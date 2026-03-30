# 实验一完成度核对（DFA 构建）

结论：当前代码**部分完成**实验一，但还未完全满足给定规格。

## 已完成

- 已定义并实现 `CharSet` 及集合运算（并、差、区间等）。
- 已定义 `Graph`/`Edge`/`State`，并实现 `generateBasicNFA`、`union`、`product`、`plusClosure`、`closure`、`zeroOrOne`。
- 已实现 `epsilon_closure`、`move`/`DTran` 相关逻辑以及 `NFA_to_DFA` 子集构造。

## 未完全满足（关键）

- `category` 目前使用自由字符串，未按实验文档中的枚举值体系统一。
- 目前词法类别值包括：`KEYWORD`、`identifier`、`Number`、`NOTE`、`BLANK`、以及运算符字面值（如 `+`、`;`、`:=`）。
- 与文档示例的类别集合（如 `INTEGER_CONST`、`FLOAT_CONST`、`SCIENTIFIC_CONST`、`NUMERIC_OPERATOR`、`STRING_CONST`、`SPACE_CONST`、`COMPARE_OPERATOR`、`LOGIC_OPERATOR`、`ID` 等）并不一致。
- 关键字优先级存在问题：如 `read` 会被识别为 `identifier`，说明“关键字优先于标识符”策略还不稳健。

## 建议

1. 用 `enum LexemeCategory` 统一词类，替代散落的字符串。
2. 在 DFA 终态合并/冲突决策中加入显式优先级（例如 KEYWORD > ID）。
3. 扩充数字自动机，支持浮点与科学计数法以覆盖 `FLOAT_CONST` / `SCIENTIFIC_CONST`。
4. 将运算符映射到文档要求的统一类目（如 `NUMERIC_OPERATOR`、`COMPARE_OPERATOR`、`LOGIC_OPERATOR`）。
