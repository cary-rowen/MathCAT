//! Core Simplified Chinese speech and navigation regression tests.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn fractions_use_denominator_first_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;

    let complex =
        "<math><mfrac><mi>a</mi><mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        complex,
        "分数, b 加 1, 分之 a, 结束分数",
    )?;
    test("zh", "ClearSpeak", complex, "分数，分子为 a; 分母为 b 加 1")
}

#[test]
fn zh_cn_locale_falls_back_to_simplified_chinese_rules() -> Result<()> {
    // Regional Chinese locales without overrides must use the base zh rules.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )
}

#[test]
fn roots_and_powers_use_chinese_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>",
        "根号 x 加 y 结束根号",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>",
        "x 的 n 次方根",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Terse")],
        "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>",
        "x 的 立方根",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "x 加 y 的 n 次方根, 结束根号",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><msup><mi>x</mi><mi>n</mi></msup></math>",
        "x 的 n 次方",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "根指数 n 根号, x 加 y, 结束根号",
    )
}

#[test]
fn clearspeak_ordinal_exponents_keep_the_chinese_power_noun() -> Result<()> {
    // Chinese exponent readings use "次方" rather than an inflected ordinal form.
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Exponents", "Ordinal")],
        "<math><msup><mi>x</mi><mn>4</mn></msup></math>",
        "x 的 4 次方",
    )
}

#[test]
fn confirmed_repairs_have_regression_coverage() -> Result<()> {
    // Keep the repaired readings consistent across both Chinese speech styles.
    let cases = [
        (
            "floor-and-ceiling",
            "<math><mrow><mo>&#x230a;</mo><mi>x</mi><mo>&#x230b;</mo></mrow><mo>+</mo><mrow><mo>&#x2308;</mo><mi>y</mi><mo>&#x2309;</mo></mrow></math>",
            "x 向下取整 加 y 向上取整",
        ),
        (
            "second-partial-derivative",
            "<math><mfrac><mrow><msup><mo>&#x2202;</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>&#x2202;</mo><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "f 对 x 的二阶偏导数",
        ),
        (
            "square-meter",
            "<math><mn>3</mn><msup><mi intent=':unit'>m</mi><mn>2</mn></msup></math>",
            "3 平方米",
        ),
        (
            "cubic-meter",
            "<math><mn>2</mn><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></math>",
            "2 立方米",
        ),
        (
            "general-unit-power",
            "<math><msup><mi intent=':unit'>m</mi><mn>4</mn></msup></math>",
            "米的 4 次方",
        ),
        (
            "tilde-modified-variable",
            "<math><mover><mi>z</mi><mo>&#x303;</mo></mover></math>",
            "z 上加波浪线",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn absolute_values_use_argument_first_order() -> Result<()> {
    let expr =
        "<math><mrow><mo>|</mo><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mo>|</mo></mrow></math>";
    test("zh", "SimpleSpeak", expr, "x 加 1 的绝对值")?;
    test("zh", "ClearSpeak", expr, "x 加 1 的绝对值")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "AbsEnd")],
        expr,
        "x 加 1 的绝对值, 结束绝对值",
    )?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "Cardinality")],
        "<math><mrow><mo>|</mo><mi>S</mi><mo>|</mo></mrow></math>",
        "大写 s 的基数",
    )
}

#[test]
fn literal_speech_matches_documented_examples() -> Result<()> {
    test(
        "zh",
        "LiteralSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><msqrt><mi>x</mi></msqrt></math>",
        "根号 x, 结束根号",
    )
}

#[test]
fn units_increment_and_calculus_operators_use_standard_terms() -> Result<()> {
    // Differential operators put their operand before the operator name in Chinese.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>kat</mi></math>",
        "1 开特",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2206;</mo><mi>x</mi></math>",
        "增量 x",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='laplacian($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的拉普拉斯算子",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='divergence($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的散度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='curl($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的旋度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='gradient($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的梯度",
    )
}

#[test]
fn limit_arrows_and_variable_bars_use_mathematical_context() -> Result<()> {
    // Diagonal arrows denote one-sided limits only in a limit, and a bar over a variable is not a phonetic macron.
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从下方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从上方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x2197;</mo><mi>y</mi><mo>,</mo><mi>x</mi><mo>&#x2198;</mo><mi>y</mi></math>",
        "x 右上箭头 y, 逗号; x 右下箭头 y",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mover><mi>x</mi><mo>&#xaf;</mo></mover></math>",
        "x 上横线",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xaf;</mo></math>",
        "长音符",
    )
}

#[test]
fn permutation_cycles_and_repeating_decimals_use_concise_terms() -> Result<()> {
    // Use the standard nouns "轮换" and "循环节", without literal English-style expansion.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='permutation-cycle($x)'><mi arg='x'>x</mi></mrow></math>",
        "轮换 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='repeating-decimal($a,$b)'><mn arg='a'>0.1</mn><mn arg='b'>6</mn></mrow></math>",
        "0.1 循环节为 6",
    )
}

#[test]
fn permutation_counts_use_mainland_textbook_word_order() -> Result<()> {
    // All supported P-notation layouts mean the number of permutations obtained by taking k from n.
    let cases = [
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>",
        "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>",
    ];

    for mathml in cases {
        test("zh", "SimpleSpeak", mathml, "n 取 k 的排列数")?;
    }
    Ok(())
}

#[test]
fn ellipses_use_the_standard_symbol_name() -> Result<()> {
    // U+2026 is the general ellipsis; U+22EF keeps its distinct midline name.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mo>&#x2026;</mo><mn>3</mn><mo>,</mo><mo>&#x22ef;</mo></math>",
        "1 省略号 3, 逗号, 中线水平省略号",
    )
}

#[test]
fn chemical_equilibrium_arrows_use_standard_reaction_terms() -> Result<()> {
    // Distinguish a reversible reaction from equilibria biased to either side.
    let equation = |arrow: &str| {
        format!(
            "<math><mrow data-chem-equation='3'><mi mathvariant='normal' data-chem-element='1'>H</mi><mo data-chem-equation-op='1'>{arrow}</mo><mi mathvariant='normal' data-chem-element='1'>I</mi></mrow></math>"
        )
    };
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x21cc;"),
        "大写 h, 可逆反应 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d1;"),
        "大写 h, 可逆反应 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d3;"),
        "大写 h, 平衡偏左 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d2;"),
        "大写 h, 平衡偏右 大写 i",
    )
}

#[test]
fn chemical_quadruple_bond_uses_the_standard_bond_order_term() -> Result<()> {
    // A bond formed by four shared electron pairs is a 四重键, parallel to 单键、双键、三键.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>C</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>&#x2263;</mo><mi mathvariant='normal' data-chem-element='1'>C</mi></mrow></math>",
        "大写 c, 四重键 大写 c",
    )
}

#[test]
fn function_intents_use_natural_chinese_argument_order() -> Result<()> {
    // Chinese property names follow their argument; binary relationships keep their semantic order.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='domain($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的定义域",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='complex-conjugate($x)'><mi arg='x'>z</mi></mrow></math>",
        "z 的共轭复数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='fractional-part($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 的小数部分",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='floor($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 向下取整",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='round($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 四舍五入后的值",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最大公约数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最小公倍数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='conditional-probability($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "在 b 条件下 a 的概率",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-above($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从上方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-below($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从下方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set-difference($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 与 b 的差集",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-denominator($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
        "x, y 与 z 的最小公分母",
    )
}

#[test]
fn argument_owned_function_variants_keep_their_standard_terms() -> Result<()> {
    // Exercise every remaining branch of the shared argument-first function rule.
    let cases = [
        ("inverse", "f", "f 的逆"),
        ("codomain", "f", "f 的陪域"),
        ("image", "f", "f 的像"),
        ("max", "s", "s 的最大值"),
        ("min", "s", "s 的最小值"),
        ("complex-arg", "z", "z 的辐角"),
        ("real-part", "z", "z 的实部"),
        ("imaginary-part", "z", "z 的虚部"),
        ("complement", "a", "a 的补集"),
        ("cardinality", "s", "s 的基数"),
        ("probability", "a", "a 的概率"),
        ("volume", "v", "v 的体积"),
        ("chemistry-concentration", "c", "c 的浓度"),
        ("ceiling", "x", "x 向上取整"),
    ];

    for (intent, argument, expected) in cases {
        let mathml = format!(
            "<math><mrow intent='{intent}($x)'><mi arg='x'>{argument}</mi></mrow></math>"
        );
        test("zh", "ClearSpeak", &mathml, expected)
            .map_err(|error| anyhow::anyhow!("{intent}: {error}"))?;
    }
    Ok(())
}

#[test]
fn linear_algebra_intents_use_standard_noun_phrases() -> Result<()> {
    // These intents denote a property of an object or a map between spaces.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='determinant($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的行列式",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='adjugate($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的伴随矩阵",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='magnitude($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的模",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='transpose($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的转置",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='trace($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的迹",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='dimension($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的维数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='kernel($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的核",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u,$v)'><mi arg='u'>u</mi><mi arg='v'>v</mi></mrow></math>",
        "由 u 与 v 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u)'><mi arg='u'>u</mi></mrow></math>",
        "由 u 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m)'><mi arg='m'>m</mi></mrow></math>",
        "m 的同态",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m,$n)'><mi arg='m'>m</mi><mi arg='n'>n</mi></mrow></math>",
        "m 到 n 的同态",
    )
}

#[test]
fn non_divisibility_uses_the_textbook_relation() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='does-not-divide($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 不整除 b",
    )
}

#[test]
fn long_multiscripts_state_the_remaining_order_naturally() -> Result<()> {
    // Once explicit pairs are exhausted, state that the remaining lower and upper scripts alternate.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mprescripts/><mi>k</mi><mi>l</mi><mi>m</mi><mi>n</mi><mi>o</mi><mi>p</mi></mmultiscripts></math>",
        "大写 t 有 3 组前置上下标, 前下标 k 且 前上标 l, 前下标 m 且 前上标 n 其余前置下标、上标依次交替 o p 结束前置上下标",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mi>a</mi><mi>b</mi><mi>c</mi><mi>d</mi><mi>e</mi><mi>f</mi><mi>g</mi><mi>h</mi><mi>i</mi><mi>j</mi></mmultiscripts></math>",
        "大写 t 有 5 组后置上下标, 下标 a 且 上标 b 下标 c 且 上标 d 下标 e 且 上标 f 下标 g 且 上标 h 其余后置下标、上标依次交替 i j 结束后置上下标",
    )
}

#[test]
fn set_relations_use_textbook_containment_terms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2282;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2286;</mo><mi>D</mi><mo>,</mo><mi>E</mi><mo>&#x228a;</mo><mi>F</mi><mo>,</mo><mi>G</mi><mo>&#x2acb;</mo><mi>H</mi></math>",
        "大写 a 子集 大写 b, 逗号; 大写 c 子集或等于 大写 d; 逗号; 大写 e 真子集 大写 f; 逗号; 大写 g 真子集 大写 h",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2284;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2288;</mo><mi>D</mi></math>",
        "大写 a 非子集 大写 b; 逗号; 大写 c 既非子集也不等于, 大写 d",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 子集 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset-or-equal($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 子集或等于 y",
    )
}

#[test]
fn set_operations_use_standard_verb_forms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>&#x222a;</mo><mi>C</mi></math>",
        "大写 a 交 大写 b, 并 大写 c",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='intersection($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 交 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='union($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 并 y",
    )
}

#[test]
fn textbook_logic_and_geometry_symbols_use_standard_readings() -> Result<()> {
    // Prefer the relation and shape names used in mainland textbooks over visual Unicode descriptions.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>,</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x21d2;</mo><mi>r</mi><mo>&#x2228;</mo><mi>s</mi></math>",
        "任意 x 属于 大写 r; 逗号; p 且 q 推出 r 或 s",
    )?;
    test("zh", "SimpleSpeak", "<math><mo>&#x2205;</mo></math>", "空集")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x21a6;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi></math>",
        "x 映射到 x 平方, 逗号; 大写 a 差集 大写 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x27fc;</mo><msup><mi>x</mi><mn>2</mn></msup></math>",
        "x 映射到 x 平方",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>p</mi><mo>&#x27f9;</mo><mi>q</mi></math>",
        "p 推出 q",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='similar($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 相似于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x223d;</mo><mi>b</mi></math>",
        "a 反转波浪号 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>&#x2245;</mo><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></math>",
        "三角形, 大写 a 大写 b 大写 c; 近似等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='congruent($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 全等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2299;</mo><mi>O</mi><mo>,</mo><mo>&#x25b1;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mi>D</mi></math>",
        "圆 大写 o, 逗号; 白色平行四边形; 大写 a 大写 b 大写 c 大写 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2299;</mo><mi>b</mi></math>",
        "a 带圈点运算符 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x22bf;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>",
        "直角三角形, 大写 a 大写 b 大写 c",
    )
}

#[test]
fn plus_minus_symbols_follow_standard_contextual_readings() -> Result<()> {
    // GB 3102.11 distinguishes signs used alone from binary plus/minus operations.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo></math>",
        "正或负",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo><mi>x</mi></math>",
        "正或负 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>20</mn><mo>&#xb1;</mo><mn>0.5</mn></math>",
        "20 加或减 0.5",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2213;</mo></math>",
        "负或正",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2213;</mo><mi>b</mi></math>",
        "a 减或加 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='plus-or-minus($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 加或减 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='minus-or-plus($a,$b)'><mi arg='a'>c</mi><mi arg='b'>d</mi></mrow></math>",
        "c 减或加 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2266;</mo><mi>b</mi></math>",
        "a 小于等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2267;</mo><mi>d</mi></math>",
        "c 大于等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
        "a 小于或倾斜等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2a7e;</mo><mi>d</mi></math>",
        "c 大于或倾斜等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>f</mi><mo>&#x2243;</mo><mi>g</mi></math>",
        "f 渐近等于 g",
    )
}

#[test]
fn double_factorial_distinguishes_math_and_literal_contexts() -> Result<()> {
    // U+203C is a double factorial in formulas but remains punctuation in literal content.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x203c;</mo></math>",
        "双阶乘",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent=':literal'><mo>&#x203c;</mo></mrow></math>",
        "双感叹号",
    )
}

#[test]
fn standard_number_sets_use_textbook_names() -> Result<()> {
    let cases = [
        ("set-of-integers", "ℤ", "全体整数的集合"),
        ("set-of-reals", "ℝ", "全体实数的集合"),
        ("set-of-rationals", "ℚ", "全体有理数的集合"),
        ("set-of-natural-numbers", "ℕ", "全体自然数的集合"),
        ("set-of-complex-numbers", "ℂ", "全体复数的集合"),
        ("set-of-primes", "ℙ", "全体素数的集合"),
    ];

    for (intent, symbol, expected) in cases {
        let expr = format!("<math><mi intent='{intent}'>{symbol}</mi></math>");
        test("zh", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn set_builder_description_is_natural() -> Result<()> {
    let expr = "<math><mrow><mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&gt;</mo><mn>2</mn></mrow><mo>}</mo></mrow></math>";
    test("zh", "ClearSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Sets", "woAll")],
        expr,
        "满足 x 大于 2 的 x 组成的集合",
    )?;
    test("zh", "SimpleSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")
}

#[test]
fn multi_argument_set_intent_is_not_mistaken_for_empty_set() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "集合 x 逗号, y",
    )
}

#[test]
fn half_open_intervals_name_both_ends() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
        "左开右闭区间 a 逗号 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow></math>",
        "左闭右开区间 a 逗号 b",
    )
}

#[test]
fn quotient_and_remainder_follow_operand_order() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='quotient($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的商",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='remainder($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的余数",
    )
}

#[test]
fn coordinates_and_angles_use_standard_terms() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='cartesian-coordinate($a,$b,$c)'><mi arg='a'>x</mi><mi arg='b'>y</mi><mi arg='c'>z</mi></mrow></math>",
        "直角坐标 x 逗号, y 逗号, z",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='measured-angle:prefix($x)'><mi arg='x'>x</mi></mrow></math>",
        "角 x 的度数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='angle-measure:prefix($x)'><mi arg='x'>y</mi></mrow></math>",
        "角 y 的度数",
    )
}

#[test]
fn geometry_objects_use_textbook_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='arc($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "弧 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='measure-of-angle($a,$b,$c)'><mi arg='a'>a</mi><mi arg='b'>b</mi><mi arg='c'>c</mi></mrow></math>",
        "角 a b c 的度数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='length($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的长度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='area($x)'><mi arg='x'>s</mi></mrow></math>",
        "s 的面积",
    )
}

#[test]
fn definite_integral_announces_limits() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mn>1</mn></msubsup><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mi>d</mi><mi>x</mi></math>",
        "积分 从 0 到 1, f x d x",
    )
}

#[test]
fn matrix_announces_dimensions_and_rows() -> Result<()> {
    // Matrix row numbers follow the Chinese ordinal pattern "第 n 行".
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4",
    )
}

#[test]
fn menclose_names_the_mark_instead_of_the_result() -> Result<()> {
    // The notation describes the drawn enclosure: a radical sign or two crossing strike lines.
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='radical'><mi>x</mi></menclose></math>",
        "根号, 包围 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='updiagonalstrike downdiagonalstrike'><mi>x</mi></menclose></math>",
        "交叉, 划掉, 包围 x",
    )
}

#[test]
fn tau_and_dotted_minus_symbols_are_distinguishable() -> Result<()> {
    test("zh", "SimpleSpeak", "<math><mi>&#x03c4;</mi></math>", "陶")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2a2b;</mo><mo>,</mo><mo>&#x2a2c;</mo></math>",
        "带下降点列的减号, 逗号, 带上升点列的减号",
    )
}

#[test]
fn unicode_letter_currency_and_operator_names_match_character_identity() -> Result<()> {
    // These names distinguish characters that were blank or assigned to a different symbol.
    let cases = [
        ("0306", "上加短音符"),
        ("030c", "上加倒抑扬符"),
        ("0430", "西里尔字母阿"),
        ("043b", "西里尔字母埃勒"),
        ("0440", "西里尔字母埃尔"),
        ("0607", "阿拉伯-印度四次方根"),
        ("20b3", "奥斯特拉尔货币符号"),
        ("20e7", "年金符号"),
        ("2106", "每个"),
        ("2114", "磅符号"),
        ("2116", "序号符号"),
        ("2127", "姆欧"),
        ("2129", "倒置希腊小写字母约塔"),
        ("223c", "波浪运算符"),
        ("223d", "反转波浪号"),
        ("2240", "环积"),
        ("2244", "不渐近等于"),
        ("2246", "近似但不等于"),
        ("2257", "圆圈等于"),
        ("225c", "德尔塔等于"),
        ("226d", "不等价于"),
        ("22a3", "左断言符"),
        ("22a6", "断言符"),
        ("22a8", "为真"),
        ("22b8", "多重映射"),
        ("22c6", "星号运算符"),
        ("2327", "矩形框内的 X"),
        ("2332", "锥度"),
        ("23e3", "带圆圈的苯环"),
        ("260c", "合"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_arrows_shapes_and_ornaments_name_visible_features() -> Result<()> {
    // Direction, fill, quadrant, and bracket shape must remain distinguishable in speech.
    let cases = [
        ("219e", "向左双头箭头"),
        ("21ba", "逆时针开口圆箭头"),
        ("21bb", "顺时针开口圆箭头"),
        ("21dc", "向左曲线箭头"),
        ("21dd", "向右曲线箭头"),
        ("21c4", "上方右箭头下方左箭头"),
        ("21c5", "左侧上箭头右侧下箭头"),
        ("21c6", "上方左箭头下方右箭头"),
        ("21ea", "从横线向上的白色箭头"),
        ("25cd", "竖直线填充的圆"),
        ("25d4", "右上象限为黑色的圆"),
        ("25d5", "除左上象限外为黑色的圆"),
        ("25d9", "反白圆"),
        ("25da", "上半反白圆"),
        ("25db", "下半反白圆"),
        ("25e6", "复合"),
        ("29eb", "黑色长菱形"),
        ("2661", "空心红桃"),
        ("2665", "实心红桃"),
        ("2680", "骰子一点"),
        ("2688", "右侧带白点的实心圆"),
        ("2768", "中等左圆括号装饰符"),
        ("2774", "中等左花括号装饰符"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_composite_symbols_preserve_feature_and_spatial_order() -> Result<()> {
    // Expected names follow the Unicode character identity, including which component is above.
    let cases = [
        ("2105", "转交"),
        ("231c", "左上角"),
        ("23dc", "上置圆括号"),
        ("23df", "下置花括号"),
        ("23e0", "上置六角括号"),
        ("23e1", "下置六角括号"),
        ("2681", "骰子二点"),
        ("26aa", "中等白色圆"),
        ("27c1", "内含小型白色三角形的白色三角形"),
        ("27c3", "开子集"),
        ("27c5", "左 S 形多重集定界符"),
        ("27c6", "右 S 形多重集定界符"),
        ("27ca", "带横线的竖线"),
        ("27d0", "中心带点的白色菱形"),
        ("27e0", "被横线分割的长菱形"),
        ("2772", "细左六角括号装饰符"),
        ("2773", "细右六角括号装饰符"),
        ("27ec", "左白六角括号"),
        ("27ed", "右白六角括号"),
        ("27f2", "逆时针缺口圆箭头"),
        ("27f3", "顺时针缺口圆箭头"),
        ("27f4", "带圈加号的向右箭头"),
        ("27ff", "向右长曲线箭头"),
        ("2938", "顺时针右侧弧形箭头"),
        ("2942", "短向左箭头上方的向右箭头"),
        (
            "294a",
            "倒钩向上的向左鱼叉箭头与倒钩向下的向右鱼叉箭头",
        ),
        ("2970", "圆头向右双线箭头"),
        ("2971", "向右箭头上方的等号"),
        ("2976", "向左箭头上方的小于号"),
        ("298d", "上角带短线的左方括号"),
        ("2997", "左黑六角括号"),
        ("2998", "右黑六角括号"),
        ("29a8", "开口边末端带向上偏右箭头的测量角"),
        ("29ac", "开口边末端带向右偏上箭头的测量角"),
        ("29b1", "上方带横线的空集"),
        ("29b5", "带横线的圆"),
        ("2a22", "上方带小圆圈的加号"),
        ("2a48", "并集号、横线、交集号从上到下排列"),
        ("2a81", "上方带点的小于或倾斜等号"),
        ("2a82", "上方带点的大于或倾斜等号"),
        ("2a83", "右上方带点的小于或倾斜等号"),
        ("2a84", "左上方带点的大于或倾斜等号"),
        ("2a8b", "小于号、双线等号、大于号从上到下排列"),
        ("2aa8", "曲线闭合的小于号在斜等号上方"),
        ("2aa9", "曲线闭合的大于号在斜等号上方"),
        ("2acd", "左侧开口的方框运算符"),
        ("2acf", "闭子集"),
        ("2ada", "顶部带丁字的叉形符号"),
        ("2b00", "向右上方的白色箭头"),
        ("2b1a", "点状方框"),
        ("2b27", "黑色中等长菱形"),
        ("2b39", "带箭尾和竖线的向左箭头"),
        ("3248", "黑色方块上的带圈数字十"),
        ("1f8d1", "上方为长向右鱼叉箭头，下方为长向左鱼叉箭头"),
        ("1f8d2", "上方为长向右鱼叉箭头，下方为短向左鱼叉箭头"),
        ("1f8d3", "上方为短向右鱼叉箭头，下方为长向左鱼叉箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_full_relations_keep_distinct_mathematical_meanings() -> Result<()> {
    // These look similar to common relations, so retain the Unicode-defined distinction in speech.
    let cases = [
        (
            "natural-join",
            "<math><mi>R</mi><mo>&#x22c8;</mo><mi>S</mi></math>",
            "大写 r 自然连接 大写 s",
        ),
        (
            "slanted-less-or-equal",
            "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
            "a 小于或倾斜等于 b",
        ),
        (
            "slanted-greater-or-equal",
            "<math><mi>a</mi><mo>&#x2a7e;</mo><mi>b</mi></math>",
            "a 大于或倾斜等于 b",
        ),
        (
            "less-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8d;</mo><mi>b</mi></math>",
            "a 小于、相似或等于 b",
        ),
        (
            "greater-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8e;</mo><mi>b</mi></math>",
            "a 大于、相似或等于 b",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_full_structured_symbols_have_verified_readings() -> Result<()> {
    // Use real MathML structures where a symbol's role affects its spoken form.
    let cases = [
        (
            "U+2A00 n-ary circled dot",
            "<math><munderover><mo>&#x2a00;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈点运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A01 n-ary circled plus",
            "<math><munderover><mo>&#x2a01;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈加号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A02 n-ary circled times",
            "<math><munderover><mo>&#x2a02;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈乘号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A03 n-ary union with dot",
            "<math><munderover><mo>&#x2a03;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带点的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A04 n-ary union with plus",
            "<math><munderover><mo>&#x2a04;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带加号的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A05 n-ary square intersection",
            "<math><munderover><mo>&#x2a05;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方交集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A06 n-ary square union",
            "<math><munderover><mo>&#x2a06;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+228E multiset union",
            "<math><mi>A</mi><mo>&#x228e;</mo><mi>B</mi></math>",
            "大写 a 多重集并集 大写 b",
        ),
        (
            "U+2A0B summation with integral",
            "<math><mo>&#x2a0b;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带积分号的求和 f x d x",
        ),
        (
            "U+2A0C quadruple integral",
            "<math><mo>&#x2a0c;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四重积分运算符 f x d x",
        ),
        (
            "U+2A0D finite-part integral",
            "<math><mo>&#x2a0d;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "有限部积分 f x d x",
        ),
        (
            "U+2A0E integral with double stroke",
            "<math><mo>&#x2a0e;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "双线积分 f x d x",
        ),
        (
            "U+2A0F average integral",
            "<math><mo>&#x2a0f;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带斜线的平均积分号 f x d x",
        ),
        (
            "U+2A10 circulation function",
            "<math><mo>&#x2a10;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "环流函数 f x d x",
        ),
        (
            "U+2A11 anticlockwise integration",
            "<math><mo>&#x2a11;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "逆时针积分 f x d x",
        ),
        (
            "U+2A12 rectangular path around pole",
            "<math><mo>&#x2a12;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点矩形路径线积分 f x d x",
        ),
        (
            "U+2A13 semicircular path around pole",
            "<math><mo>&#x2a13;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点半圆路径线积分 f x d x",
        ),
        (
            "U+2A14 line integration not including pole",
            "<math><mo>&#x2a14;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "不包含极点的线积分 f x d x",
        ),
        (
            "U+2A15 integral around a point",
            "<math><mo>&#x2a15;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕点积分运算符 f x d x",
        ),
        (
            "U+2A16 quaternion integral",
            "<math><mo>&#x2a16;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四元数积分运算符 f x d x",
        ),
        (
            "U+2AC3 subset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac3;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的子集或等于号, 大写 b",
        ),
        (
            "U+2AC4 superset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac4;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的超集或等于号, 大写 b",
        ),
        (
            "U+2ADB transversal intersection",
            "<math><mi>A</mi><mo>&#x2adb;</mo><mi>B</mi></math>",
            "大写 a 横截相交 大写 b",
        ),
        (
            "U+2ADC forking",
            "<math><mi>A</mi><mo>&#x2adc;</mo><mi>B</mi></math>",
            "大写 a 分叉 大写 b",
        ),
        (
            "U+2ADD nonforking",
            "<math><mi>A</mi><mo>&#x2add;</mo><mi>B</mi></math>",
            "大写 a 非分叉 大写 b",
        ),
        (
            "U+2AF9 double-line slanted less or equal",
            "<math><mi>a</mi><mo>&#x2af9;</mo><mi>b</mi></math>",
            "a 双线倾斜小于或等于 b",
        ),
        (
            "U+2AFA double-line slanted greater or equal",
            "<math><mi>a</mi><mo>&#x2afa;</mo><mi>b</mi></math>",
            "a 双线倾斜大于或等于 b",
        ),
        (
            "U+2947 right arrow through x",
            "<math><mi>A</mi><mo>&#x2947;</mo><mi>B</mi></math>",
            "大写 a 穿过叉号的向右箭头, 大写 b",
        ),
        (
            "U+2948 left-right arrow through circle",
            "<math><mi>A</mi><mo>&#x2948;</mo><mi>B</mi></math>",
            "大写 a, 穿过小圆圈的左右箭头, 大写 b",
        ),
        (
            "U+2949 upward two-headed arrow from circle",
            "<math><mo>&#x2949;</mo><mi>x</mi></math>",
            "从小圆圈出发的向上双头箭头 x",
        ),
        (
            "U+297C left fish tail",
            "<math><mi>A</mi><mo>&#x297c;</mo><mi>B</mi></math>",
            "大写 a 左鱼尾 大写 b",
        ),
        (
            "U+297D right fish tail",
            "<math><mi>A</mi><mo>&#x297d;</mo><mi>B</mi></math>",
            "大写 a 右鱼尾 大写 b",
        ),
        (
            "U+297E upward fish tail",
            "<math><mi>A</mi><mo>&#x297e;</mo><mi>B</mi></math>",
            "大写 a 上鱼尾 大写 b",
        ),
        (
            "U+297F downward fish tail",
            "<math><mi>A</mi><mo>&#x297f;</mo><mi>B</mi></math>",
            "大写 a 下鱼尾 大写 b",
        ),
        (
            "U+29C4 squared rising diagonal slash",
            "<math><mo>&#x29c4;</mo><mi>x</mi></math>",
            "方框内上升斜线 x",
        ),
        (
            "U+29CA triangle with dot above",
            "<math><mo>&#x29ca;</mo><mi>x</mi></math>",
            "上方带点的三角形 x",
        ),
        (
            "U+20E1 combining left-right arrow above",
            "<math><mover><mi>v</mi><mo>&#x20e1;</mo></mover></math>",
            "v 上加左右箭头",
        ),
        (
            "U+20DE combining enclosing square",
            "<math><mover><mi>x</mi><mo>&#x20de;</mo></mover></math>",
            "x 外加方形",
        ),
        (
            "U+20E0 combining enclosing circle backslash",
            "<math><mover><mi>x</mi><mo>&#x20e0;</mo></mover></math>",
            "x 外加圆圈反斜杠",
        ),
        (
            "U+20DC combining four dots above",
            "<math><mover><mi>x</mi><mo>&#x20dc;</mo></mover></math>",
            "x 上加四点",
        ),
        (
            "U+20E9 combining wide bridge above",
            "<math><mover><mi>x</mi><mo>&#x20e9;</mo></mover></math>",
            "x 上加宽桥形符",
        ),
        (
            "U+20EB combining long double solidus overlay",
            "<math><mover><mi>x</mi><mo>&#x20eb;</mo></mover></math>",
            "x 叠加长双斜杠",
        ),
        (
            "U+20EF combining right arrow below",
            "<math><munder><mi>v</mi><mo>&#x20ef;</mo></munder></math>",
            "v 下加向右箭头",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_script_characters_keep_standalone_and_structural_readings() -> Result<()> {
    let superscripts = [
        ("2070", "mn", "零次方", "x 的 0 次方"),
        ("2071", "mi", "i次方", "x 的 i 次方"),
        ("2074", "mn", "四次方", "x 的 4 次方"),
        ("2075", "mn", "五次方", "x 的 5 次方"),
        ("2076", "mn", "六次方", "x 的 6 次方"),
        ("2077", "mn", "七次方", "x 的 7 次方"),
        ("2078", "mn", "八次方", "x 的 8 次方"),
        ("2079", "mn", "九次方", "x 的 9 次方"),
        ("207a", "mo", "上标加号", "x 上标 +"),
        ("207b", "mo", "上标减号", "x 上标 −"),
        ("207c", "mo", "上标等号", "x 上标 ="),
        ("207d", "mo", "上标左圆括号", "x 上标 ("),
        ("207e", "mo", "上标右圆括号", "x 上标 )"),
        ("207f", "mi", "n次方", "x 的 n 次方"),
    ];
    let subscripts = [
        ("2080", "mn", "下标零", "x 下标 0"),
        ("2081", "mn", "下标一", "x 下标 1"),
        ("2082", "mn", "下标二", "x 下标 2"),
        ("2083", "mn", "下标三", "x 下标 3"),
        ("2084", "mn", "下标四", "x 下标 4"),
        ("2085", "mn", "下标五", "x 下标 5"),
        ("2086", "mn", "下标六", "x 下标 6"),
        ("2087", "mn", "下标七", "x 下标 7"),
        ("2088", "mn", "下标八", "x 下标 8"),
        ("2089", "mn", "下标九", "x 下标 9"),
        ("208a", "mo", "下标加号", "x 下标 + 结束下标"),
        ("208b", "mo", "下标减号", "x 下标 − 结束下标"),
        ("208c", "mo", "下标等号", "x 下标 = 结束下标"),
        ("208d", "mo", "下标左圆括号", "x 下标 ( 结束下标"),
        ("208e", "mo", "下标右圆括号", "x 下标 ) 结束下标"),
        ("2090", "mi", "下标a", "x 下标 a"),
        ("2091", "mi", "下标e", "x 下标 e"),
        ("2092", "mi", "下标o", "x 下标 o"),
        ("2093", "mi", "下标x", "x 下标 x"),
        ("2095", "mi", "下标h", "x 下标 h"),
        ("2096", "mi", "下标k", "x 下标 k"),
        ("2097", "mi", "下标l", "x 下标 l"),
        ("2098", "mi", "下标m", "x 下标 m"),
        ("2099", "mi", "下标n", "x 下标 n"),
        ("209a", "mi", "下标p", "x 下标 p"),
        ("209b", "mi", "下标s", "x 下标 s"),
        ("209c", "mi", "下标t", "x 下标 t"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, standalone, structured) in &superscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msup><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msup></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msup: {error}"))?;
        }
        for &(codepoint, tag, standalone, structured) in &subscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msub><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msub></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msub: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn audited_unicode_gap_symbols_have_direct_and_contextual_coverage() -> Result<()> {
    // TINY and MINY are proper operator names, not translations of the size adjectives.
    let direct = [
        ("20e2", "外加屏幕"),
        ("220b", "包含"),
        ("220c", "不包含"),
        ("220d", "小型包含"),
        ("2256", "环等于"),
        ("228b", "真超集"),
        ("228d", "多重集乘法"),
        ("22e4", "方形像或不等于"),
        ("22e5", "方形原像或不等于"),
        ("22ee", "垂直省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
        ("2a07", "双逻辑与运算符"),
        ("2a08", "双逻辑或运算符"),
        ("2a87", "小于且单线不等于"),
        ("2a88", "大于且单线不等于"),
        ("2a89", "小于且不约等于"),
        ("2a8a", "大于且不约等于"),
        ("2acc", "真超集"),
        ("2298", "带圆圈除号斜线"),
        ("22d4", "横截于"),
        ("29fe", "Tiny 算子"),
        ("29ff", "Miny 算子"),
        ("2ae1", "带 S 的垂直符号"),
    ];
    let contextual = [
        ("220b", "x", "大写 a 包含 x"),
        ("220c", "x", "大写 a 不包含 x"),
        ("220d", "x", "大写 a 小型包含 x"),
        ("2298", "B", "大写 a 带圆圈除号斜线, 大写 b"),
        ("22d4", "B", "大写 a 横截于 大写 b"),
        ("2a87", "b", "大写 a 小于且单线不等于 b"),
        ("2a88", "b", "大写 a 大于且单线不等于 b"),
        ("2ae1", "B", "大写 a 带 S 的垂直符号, 大写 b"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &direct {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/direct: {error}"))?;
        }
        for &(codepoint, rhs, expected) in &contextual {
            let expr = format!(
                "<math><mi>A</mi><mo>&#x{codepoint};</mo><mi>{rhs}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/context: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_unicode_phase_symbols_have_verified_readings() -> Result<()> {
    // Unicode NamesList supplies the identities; the structured cases verify placement too.
    let combining = [
        ("20d0", "mover", "v", "v 上加左鱼叉箭头"),
        ("20d1", "mover", "v", "v 上加右鱼叉箭头"),
        ("20d2", "mover", "x", "x 叠加长竖直线"),
        ("20d3", "mover", "x", "x 叠加短竖直线"),
        ("20d4", "mover", "x", "x 上加逆时针箭头"),
        ("20d5", "mover", "x", "x 上加顺时针箭头"),
        ("20d6", "mover", "v", "v 上加向左箭头"),
        ("20d7", "mover", "v", "v 上加向右箭头"),
        ("20d8", "mover", "x", "x 叠加圆环"),
        ("20d9", "mover", "x", "x 叠加顺时针圆环"),
        ("20da", "mover", "x", "x 叠加逆时针圆环"),
        ("20db", "mover", "x", "x 上加三点"),
        ("20dd", "mover", "x", "x 外加圆圈"),
        ("20df", "mover", "x", "x 外加菱形"),
        ("20e3", "mover", "x", "x 外加键帽"),
        ("20e4", "mover", "x", "x 外加向上三角形"),
        ("20e5", "mover", "x", "x 叠加反斜杠"),
        ("20e6", "mover", "x", "x 叠加双竖线"),
        ("20e8", "munder", "x", "x 下加三点"),
        ("20ea", "mover", "x", "x 叠加向左箭头"),
        (
            "20ec",
            "munder",
            "v",
            "v 下加倒钩向下的向右鱼叉箭头",
        ),
        (
            "20ed",
            "munder",
            "v",
            "v 下加倒钩向下的向左鱼叉箭头",
        ),
        ("20ee", "munder", "v", "v 下加向左箭头"),
        ("20f0", "mover", "x", "x 上加星号"),
    ];
    let relations = [
        ("22d0", "a 双子集 b"),
        ("22d1", "a 双超集 b"),
        ("22d2", "a 双交集 b"),
        ("22d3", "a 双并集 b"),
        ("22d5", "a 等于且平行于 b"),
        ("22d6", "a 带点小于 b"),
        ("22d7", "a 带点大于 b"),
        // Keep these distinct from U+226A/U+226B ("远小于/远大于").
        ("22d8", "a 极小于 b"),
        ("22d9", "a 极大于 b"),
        ("22da", "a 小于、等于或大于 b"),
        ("22db", "a 大于、等于或小于 b"),
        ("22dd", "a 等于或大于 b"),
        ("22de", "a 等于或先于 b"),
        ("22df", "a 等于或后于 b"),
    ];
    let phase_relations = [
        ("22e0", "a 既不先于也不等于 b"),
        ("22e1", "a 既不后于也不等于 b"),
        ("22e2", "a 既非方形像也不等于 b"),
        ("22e3", "a 既非方形原像也不等于 b"),
        ("22e4", "a 方形像或不等于 b"),
        ("22e5", "a 方形原像或不等于 b"),
        ("22e6", "a 小于但不等价于 b"),
        ("22e7", "a 大于但不等价于 b"),
        ("22e8", "a 先于但不等价于 b"),
        ("22e9", "a 后于但不等价于 b"),
        ("22ea", "a 不是正规子群 b"),
        ("22eb", "a 不包含正规子群 b"),
        ("22ec", "a 既不是正规子群也不等于 b"),
        ("22ed", "a 既不包含正规子群也不等于 b"),
    ];
    let ellipses = [
        ("22ee", "垂直省略号"),
        ("22ef", "中线水平省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
    ];
    let set_relations = [
        ("22f2", "x 带长横线的属于号 a"),
        ("22f3", "x 横线末端带竖线的属于号 a"),
        ("22f4", "x 横线末端带竖线的较小属于号 a"),
        ("22f5", "x 上方带点的属于号 a"),
        ("22f6", "x 上方带横线的属于号 a"),
        ("22f7", "x 上方带横线的较小属于号 a"),
        ("22f8", "x 下方带横线的属于号 a"),
        ("22f9", "x 带两条横线的属于号 a"),
        ("22fa", "a 带长横线的包含号 x"),
        ("22fb", "a 横线末端带竖线的包含号 x"),
        ("22fc", "a 横线末端带竖线的较小包含号 x"),
        ("22fd", "a 上方带横线的包含号 x"),
        ("22fe", "a 上方带横线的较小包含号 x"),
        ("22ff", "x Z 记号多重集隶属符 a"),
    ];
    let integrals = [
        ("2a0a", "模二和 f"),
        ("2a17", "带钩向左箭头的积分号 f"),
        ("2a18", "带乘号的积分号 f"),
        ("2a19", "带交集号的积分号 f"),
        ("2a1a", "带并集号的积分号 f"),
        ("2a1b", "带上横线的积分号 f"),
        ("2a1c", "带下横线的积分号 f"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, base, expected) in &combining {
            let expr = format!(
                "<math><{tag}><mi>{base}</mi><mo>&#x{codepoint};</mo></{tag}></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/structured: {error}"))?;
        }
        for &(codepoint, expected) in &relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/relation: {error}"))?;
        }
        for &(codepoint, expected) in &phase_relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/phase-relation: {error}"))?;
        }
        for &(codepoint, expected) in &ellipses {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/ellipsis: {error}"))?;
        }
        for &(codepoint, expected) in &set_relations {
            let (left, right) = if matches!(codepoint, "22fa" | "22fb" | "22fc" | "22fd" | "22fe") {
                ("a", "x")
            } else {
                ("x", "a")
            };
            let expr = format!(
                "<math><mi>{left}</mi><mo>&#x{codepoint};</mo><mi>{right}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/set-relation: {error}"))?;
        }
        for &(codepoint, expected) in &integrals {
            let expr = format!("<math><mo>&#x{codepoint};</mo><mi>f</mi></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/integral: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn miscellaneous_technical_symbols_have_verified_readings() -> Result<()> {
    // Cover every U+2300-U+23E7 entry not already exercised by another zh test.
    let cases = [
        ("2300", "直径"),
        ("2301", "电箭头"),
        ("2302", "房屋"),
        ("2304", "向下箭头尖"),
        ("2305", "射影"),
        ("2306", "透视"),
        ("2307", "波浪线"),
        ("230c", "右下裁切符"),
        ("230d", "左下裁切符"),
        ("230e", "右上裁切符"),
        ("230f", "左上裁切符"),
        ("2310", "反向非"),
        ("2311", "方菱形"),
        ("2312", "弧"),
        ("2313", "弓形"),
        ("2314", "扇形"),
        ("2315", "电话记录器符号"),
        ("2316", "位置指示十字线"),
        ("2317", "视图数据方框"),
        ("2318", "命令键"),
        ("2319", "倒置非"),
        ("231a", "手表"),
        ("231b", "沙漏"),
        ("231d", "右上角"),
        ("231e", "左下角"),
        ("231f", "右下角"),
        ("2320", "积分号上半部"),
        ("2321", "积分号下半部"),
        ("2322", "皱眉"),
        ("2323", "微笑"),
        ("2324", "回车键"),
        ("2325", "选项键"),
        ("2326", "向前删除键"),
        ("2328", "键盘"),
        ("2329", "左尖括号"),
        ("232a", "右尖括号"),
        ("232b", "退格键"),
        ("232c", "苯环"),
        ("232d", "圆柱度"),
        ("232e", "全周轮廓符号"),
        ("232f", "对称度"),
        ("2330", "全跳动"),
        ("2331", "尺寸原点"),
        ("2334", "沉孔"),
        ("2335", "沉头孔"),
        ("2336", "APL 功能符号工字梁"),
        ("233d", "APL 功能符号圆竖线"),
        ("233f", "APL 功能符号斜杠横线"),
        ("2370", "APL 功能符号方框问号"),
        ("237c", "带向下之字形箭头的直角"),
        ("2394", "六边形"),
        ("2395", "APL 功能符号方框"),
        ("23b4", "上置方括号"),
        ("23b5", "下置方括号"),
        ("23b6", "上方为下置方括号，下方为上置方括号"),
        ("23dd", "下置圆括号"),
        ("23de", "上置花括号"),
        ("23e2", "白色梯形"),
        ("23e4", "直线度"),
        ("23e5", "平面度"),
        ("23e7", "电路交叉点"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn audited_unicode_symbols_keep_precise_names() -> Result<()> {
    // Each case locks a corrected identity, direction, shape, or spatial relationship.
    let cases = [
        ("00ab", "左双角引号"),
        ("21a8", "带底线的上下箭头"),
        ("21ab", "向左带环箭头"),
        ("21ac", "向右带环箭头"),
        ("21fd", "向左开口箭头"),
        ("21fe", "向右开口箭头"),
        ("21ff", "左右开口箭头"),
        ("224c", "全等于"),
        ("2247", "既不近似等于也不等于"),
        ("2298", "带圆圈除号斜线"),
        ("229f", "方框减号"),
        ("22a0", "方框乘号"),
        ("22a1", "方框点运算符"),
        ("22a9", "力迫"),
        ("22ae", "不力迫"),
        ("22dc", "等于或小于"),
        ("22e2", "既非方形像也不等于"),
        ("22e3", "既非方形原像也不等于"),
        ("2303", "向上箭头尖"),
        ("2333", "斜度"),
        ("23e6", "交流电"),
        ("2472", "带圈数字十九"),
        ("2736", "黑色六角星"),
        ("2794", "粗宽头向右箭头"),
        ("290a", "向上三重箭头"),
        ("290b", "向下三重箭头"),
        ("2983", "左白色花括号"),
        ("2984", "右白色花括号"),
        ("2993", "左弧小于括号"),
        ("2994", "右弧大于括号"),
        ("2995", "双左弧大于括号"),
        ("2996", "双右弧小于括号"),
        ("299b", "开口向左的测量角"),
        ("299d", "带点的测量直角"),
        ("2abd", "带点的子集号"),
        ("2abe", "带点的超集号"),
        ("2ad3", "子集号在超集号上方"),
        ("2ad4", "超集号在子集号上方"),
        ("2ad5", "子集号在子集号上方"),
        ("2ad6", "超集号在超集号上方"),
        ("e920", "双重方形并集"),
        ("e921", "双重方形交集"),
        ("e92c", "带点的恒等号"),
        ("e994", "带双斜杠的恒等号"),
        ("e997", "竖直正比号"),
        ("ea70", "带竖线的既不是正规子群也不等于"),
        ("ea71", "带竖线的既不包含正规子群也不等于"),
        ("eb60", "否定向右波浪箭头"),
        ("eb61", "否定向右弯曲箭头"),
        ("ec44", "水平全长三键"),
        ("ec47", "竖直全长三键"),
        ("ec4c", "水平半长三键"),
        ("fe35", "上置圆括号"),
        ("fe36", "下置圆括号"),
        ("fe37", "上置花括号"),
        ("fe38", "下置花括号"),
        ("fe3f", "上置角括号"),
        ("fe40", "下置角括号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_math_names_match_symbol_identity() -> Result<()> {
    // Each entry covers a corrected Unicode name or a standard mainland mathematical reading.
    let cases = [
        ("02d9", "上点符"),
        ("02ef", "修饰字母低位向下箭头尖"),
        ("02f0", "修饰字母低位向上箭头尖"),
        ("02f1", "修饰字母低位向左箭头尖"),
        ("02f2", "修饰字母低位向右箭头尖"),
        ("0332", "下加下划线"),
        ("0333", "下加双下划线"),
        ("2135", "阿列夫"),
        ("2136", "贝特"),
        ("2137", "吉梅尔"),
        ("2138", "达列特"),
        ("2140", "双线体求和号"),
        ("222f", "曲面积分"),
        ("2231", "顺时针积分"),
        ("225f", "问号等于"),
        ("22d8", "极小于"),
        ("22d9", "极大于"),
        ("299a", "竖直之字形线"),
        ("29e2", "混洗积"),
        ("2a00", "n 元带圈点运算符"),
        ("2a01", "n 元带圈加号运算符"),
        ("2a02", "n 元带圈乘号运算符"),
        ("2a03", "带点的 n 元并集运算符"),
        ("2a04", "带加号的 n 元并集运算符"),
        ("2a05", "n 元方交集运算符"),
        ("2a06", "n 元方并集运算符"),
        ("2a09", "n 元乘号运算符"),
        ("2a0f", "带斜线的平均积分号"),
        ("2a33", "压缩积"),
        ("2a3c", "内积"),
        ("2a3d", "右内积"),
        ("2a50", "带衬线和压缩积的闭合并集"),
        ("2a85", "小于或约等于"),
        ("2a86", "大于或约等于"),
        ("2af9", "双线倾斜小于或等于"),
        ("2afa", "双线倾斜大于或等于"),
        ("3372", "道尔顿"),
        ("3375", "小写 o 大写 V"),
        ("fb05", "长 s t 连字"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_mathtype_private_use_symbols_match_their_source_names() -> Result<()> {
    // MathType PUA symbols have no Unicode fallback, so each corrected identity needs a direct check.
    let cases = [
        ("e916", "带点的超集号"),
        ("e917", "带点的子集号"),
        ("e918", "下方带点的等号"),
        ("e92e", "竖线运算符"),
        ("e92f", "双竖线运算符"),
        ("e930", "三重竖线运算符"),
        ("e949", "最正值"),
        ("e950", "带竖线的正规包含于"),
        ("e951", "带竖线的包含正规子群"),
        ("e982", "带方框的直角变体"),
        ("e98f", "自由基点"),
        ("e991", "恒等于且平行于"),
        ("e992", "压缩积"),
        ("e993", "带横线的三重竖线运算符"),
        ("e995", "带三条竖线的三重横线"),
        ("e9a0", "负正弦波"),
        ("ea06", "既不小于也不等于"),
        ("ea07", "既不大于也不等于"),
        ("ea15", "既不后于也不相似于"),
        ("ea1d", "既不小于也不等于"),
        ("ea1e", "既不大于也不等于"),
        ("ea2e", "否定竖线运算符"),
        ("ea2f", "否定双竖线运算符"),
        ("ea30", "否定三重竖线运算符"),
        ("ea50", "带竖线的不是正规子群"),
        ("ea51", "带竖线的不包含正规子群"),
        ("ea55", "既不等于也不相似于"),
        ("ea63", "不严格等价于"),
        ("eb01", "小型向左箭头上方的向右箭头"),
        ("eb02", "向左箭头上方的小型向右箭头"),
        ("eb03", "小型向左鱼叉箭头上方的向右鱼叉箭头"),
        ("eb04", "向左鱼叉箭头上方的小型向右鱼叉箭头"),
        ("eb0f", "带斜线的大型左右箭头"),
        ("eb11", "带斜线的大型左右双箭头"),
        ("eb18", "带尾部和斜线的向右箭头"),
        (
            "eb36",
            "左侧倒钩向下、右侧倒钩向上的双鱼叉箭头",
        ),
        (
            "eb37",
            "左侧倒钩向上、右侧倒钩向下的双鱼叉箭头",
        ),
        ("eb3f", "右上与右下双箭头"),
        ("eb4c", "粗短黑色向左箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_double_struck_greek_matches_the_original_character_table() -> Result<()> {
    // Check every MathType PUA slot because the source order is non-alphabetic in both ranges.
    let capitals = [
        ("f201", "德尔塔"),
        ("f202", "克西"),
        ("f203", "拉姆达"),
        ("f204", "派"),
        ("f205", "西格马"),
        ("f206", "西塔"),
        ("f207", "伽马"),
        ("f208", "欧米伽"),
        ("f209", "宇普西隆"),
    ];
    for (codepoint, name) in capitals {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 大写 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let lowercase = [
        ("f220", "阿尔法"),
        ("f221", "贝塔"),
        ("f222", "斐"),
        ("f223", "泽塔"),
        ("f224", "普西"),
        ("f225", "德尔塔"),
        ("f226", "艾普西隆"),
        ("f227", "伽马"),
        ("f228", "伊塔"),
        ("f229", "约塔"),
        ("f22a", "克西"),
        ("f22b", "卡帕"),
        ("f22c", "拉姆达"),
        ("f22d", "缪"),
        ("f22e", "纽"),
        ("f22f", "艾普西隆"),
        ("f230", "派"),
        ("f231", "西塔"),
        ("f232", "柔"),
        ("f233", "西格马"),
        ("f234", "陶"),
        ("f235", "西塔"),
        ("f236", "欧米伽"),
    ];
    for (codepoint, name) in lowercase {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathematical_capital_theta_symbols_remain_speakable_in_every_variant() -> Result<()> {
    // Each 25-character range has an extra theta-symbol slot where U+03A2 must not be used.
    let cases = [
        ("1d6b9", "粗体 大写 西塔"),
        ("f419", "粗体 大写 西塔"),
        ("1d6f3", "大写 西塔"),
        ("f453", "大写 西塔"),
        ("1d72d", "粗体 大写 西塔"),
        ("f48d", "粗体 大写 西塔"),
        ("1d767", "粗体 大写 西塔"),
        ("f4c7", "粗体 大写 西塔"),
        ("1d7a1", "粗体 大写 西塔"),
        ("f501", "粗体 大写 西塔"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_ranges_keep_their_boundaries() -> Result<()> {
    // The first and last characters catch off-by-one and shifted translate mappings.
    let cases = [
        ("03aa", "大写 约塔 带分音符"),
        ("03ab", "大写 宇普西隆 带分音符"),
        ("03cf", "大写 凯"),
        ("24b6", "带圈 大写 a"),
        ("24cf", "带圈 大写 z"),
        ("24d0", "带圈 a"),
        ("24e9", "带圈 z"),
        ("1d538", "双线体 大写 a"),
        ("1d550", "双线体 大写 y"),
        ("f080", "双线体 大写 a"),
        ("f098", "双线体 大写 y"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn enclosed_alphanumeric_names_use_consistent_mainland_terms() -> Result<()> {
    // Negative circled capitals and double-circled digits must match NVDA's established terms.
    let cases = [
        ("1f150", "带圈反白 大写 a"),
        ("1f169", "带圈反白 大写 z"),
        ("24f5", "双圈 1"),
        ("24fd", "双圈 9"),
        ("24fe", "双圈数字十"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (codepoint, expected) in cases {
            let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_dingbats_editorial_marks_and_compatibility_units_are_precise() -> Result<()> {
    // Less common symbols still need exact names because visual context is unavailable to speech users.
    let cases = [
        ("2798", "粗向右下箭头"),
        ("27a2", "顶部高亮的立体向右箭头尖"),
        ("27b4", "黑色羽状向右下箭头"),
        ("27bc", "楔尾向右箭头"),
        ("2999", "点状围栏"),
        ("29cc", "三角形内字母 S"),
        ("2b1d", "黑色极小方块"),
        ("2b2a", "黑色小长菱形"),
        ("2b2c", "黑色横向椭圆"),
        ("2b30", "带小圆圈的向左箭头"),
        ("2b33", "向左长曲线箭头"),
        ("2b51", "黑色小星"),
        ("2b59", "粗圆圈内的叉号"),
        ("2e00", "直角替换标记"),
        ("2e08", "点状换位标记"),
        ("2e13", "带点奥贝洛斯符号"),
        ("2e16", "带点的右指角"),
        ("2e18", "倒置疑问感叹号"),
        ("2e19", "棕榈枝"),
        ("2e1b", "上方带圆环的波浪线"),
        ("2e30", "圆环点"),
        ("3014", "左六角括号"),
        ("3015", "右六角括号"),
        ("3018", "左白六角括号"),
        ("3019", "右白六角括号"),
        ("33c2", "上午"),
        ("33d8", "下午"),
        ("33da", "拍伦琴"),
        ("33d4", "毫巴"),
        ("33c7", "公司"),
        ("33ff", "伽"),
        ("fe64", "小型小于号"),
        ("fe65", "小型大于号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn gallon_unit_remains_distinct_from_the_gal_acceleration_symbol() -> Result<()> {
    // The unit token "gal" means gallon; U+33FF is the Gal acceleration unit and is tested above.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>gal</mi></math>",
        "1 加仑",
    )
}

fn init_navigation(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

#[test]
fn navigation_enters_fraction_numerator() -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mfrac id='frac'><mn id='num'>1</mn><mn id='den'>2</mn></mfrac></math>",
        )?;
        let speech = do_navigate_command("SetPlacemarker1")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("设置位置标记 1; 2 分之 1", speech);

        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; 2 分之 1", speech);

        let speech = do_navigate_command("ZoomIn")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("进入下一层; 进入 分子; 1", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_uses_chinese_row_and_column_number_order() -> Result<()> {
    // Cell location announcements use "第 n 行，第 n 列" rather than noun-first order.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn id='r1c1'>1</mn></mtd><mtd><mn id='r1c2'>2</mn></mtd></mtr><mtr><mtd><mn id='r2c1'>3</mn></mtd><mtd><mn id='r2c2'>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        )?;
        set_navigation_node("r2c2", 0)?;
        let speech = do_navigate_command("ReadCellCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("朗读当前单元格; 第 2 行, 第 2 列, 4", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_overview_keeps_the_root_possessive_marker() -> Result<()> {
    // Overview speech must keep 的 after a complex radicand, just like full speech.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        )?;
        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; x 加 y 的 n 次方根", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn clearspeak_multiline_labels_use_chinese_count_and_ordinal_order() -> Result<()> {
    // Overview counts take classifiers, while each branch is introduced as an ordinal.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
        "2 个分支; 第 1 个分支; x 大于 0; 第 2 个分支; x 小于 0",
    )
}

#[test]
fn calculus_formulas_cover_limits_derivatives_and_improper_integrals() -> Result<()> {
    // Exercise complete textbook formulas so contextual operators are not only tested in isolation.
    let cases = [
        (
            "limit-at-infinity",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mi>&#x221e;</mi></mrow></munder><mfrac><mn>1</mn><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 无穷大; x 分之 1",
        ),
        (
            "left-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从下方趋于 2; f x",
        ),
        (
            "right-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从上方趋于 2; f x",
        ),
        (
            "first-derivative",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 2 x",
        ),
        (
            "second-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>2</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "分数, d x 平方, 分之, d 平方 y, 结束分数",
        ),
        (
            "improper-integral",
            "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mi>&#x221e;</mi></msubsup><msup><mi>e</mi><mrow><mo>&#x2212;</mo><mi>x</mi></mrow></msup><mo>&#x2062;</mo><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 无穷大; e 的 负 x 次方, d x",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn nested_numbers_powers_and_functions_have_stable_readings() -> Result<()> {
    // Cover structures whose meaning depends on nesting or adjacent notation.
    let cases = [
        (
            "nested-fraction",
            "<math><mfrac><mn>1</mn><mfrac><mn>1</mn><mi>x</mi></mfrac></mfrac></math>",
            "分数, 分数, x 分之 1, 结束分数; 分之 1, 结束分数",
        ),
        (
            "mixed-number",
            "<math><mn>2</mn><mo>&#x2064;</mo><mfrac><mn>1</mn><mn>3</mn></mfrac></math>",
            "2 又 3 分之 1",
        ),
        (
            "decimal-fraction",
            "<math><mfrac><mn>0.25</mn><mn>0.5</mn></mfrac></math>",
            "0.5 分之 0.25",
        ),
        (
            "negative-exponent",
            "<math><msup><mi>x</mi><mrow><mo>&#x2212;</mo><mn>2</mn></mrow></msup></math>",
            "x 的 负 2 次方",
        ),
        (
            "natural-logarithm",
            "<math><mi>ln</mi><mo>&#x2061;</mo><mi>x</mi></math>",
            "自然对数 x",
        ),
        (
            "exponential-function",
            "<math><msup><mi>e</mi><mi>x</mi></msup></math>",
            "e 的 x 次方",
        ),
        (
            "factorial",
            "<math><mn>5</mn><mo>!</mo></math>",
            "5 阶乘",
        ),
        (
            "double-factorial",
            "<math><mn>5</mn><mo>&#x203c;</mo></math>",
            "5 双阶乘",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn symbolic_sets_logic_and_probability_read_as_complete_expressions() -> Result<()> {
    // Test the presentation notation users encounter, in addition to semantic intent rules.
    let cases = [
        (
            "set-membership",
            "<math><mi>x</mi><mo>&#x2208;</mo><mi>&#x211d;</mi><mo>,</mo><mi>n</mi><mo>&#x2209;</mo><mi>&#x2124;</mi></math>",
            "x 属于 实数集, 逗号; n 不属于 整数集",
        ),
        (
            "nested-quantifiers",
            "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2203;</mo><mi>y</mi><mo>:</mo><mi>P</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>",
            "任意 x 存在 y, 冒号; 大写 p, 左括号 x 逗号, y, 右括号",
        ),
        (
            "probability-function",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></math>",
            "大写 a 的概率",
        ),
        (
            "logical-equivalence",
            "<math><mi>p</mi><mo>&#x21d2;</mo><mi>q</mi><mo>&#x21d4;</mo><mi>r</mi><mo>&#x2227;</mo><mi>s</mi></math>",
            "p 推出, q 当且仅当 r 且 s",
        ),
        (
            "conditional-probability-identity",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></math>",
            "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 大写 a 交 大写 b 的概率, 结束分数",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn vectors_geometry_and_complex_conjugates_keep_mathematical_context() -> Result<()> {
    // Combine notation so arrows, bars, angle signs, and relation symbols retain their mathematical meanings.
    let cases = [
        (
            "vector-components",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>=</mo><mi>a</mi><mi>i</mi><mo>+</mo><mi>b</mi><mi>j</mi></math>",
            "向量 v, 等于, a i 加 b j",
        ),
        (
            "right-angle-and-parallel-lines",
            "<math><mo>&#x2220;</mo><mi>A</mi><mo>=</mo><mn>90</mn><mo>&#xb0;</mo><mo>,</mo><mi>l</mi><mo>&#x2225;</mo><mi>m</mi></math>",
            "角 大写 a 等于 90 度; 逗号, l 平行于 m",
        ),
        (
            "complex-conjugate-bar",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi><mo>,</mo><mover><mi>z</mi><mo>&#xaf;</mo></mover></math>",
            "z 等于 3 加 4 i; 逗号, z 上横线",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn common_compound_units_use_natural_chinese_order() -> Result<()> {
    // Keep number-unit adjacency and numerator-before-denominator unit order in real measurements.
    let cases = [
        (
            "speed",
            "<math><mfrac><mrow><mn>90</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "90 千米每 小时",
        ),
        (
            "temperature",
            "<math><mn>25</mn><mi intent=':unit'>&#xb0;C</mi></math>",
            "25 摄氏度",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_real_world_readings() -> Result<()> {
    let cases = [
        (
            "trig-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "trig-clear",
            "ClearSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "log-base-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "log-base-clear",
            "ClearSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "limit-simple",
            "SimpleSpeak",
            "Medium",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow><mi>x</mi></mfrac><mo>=</mo><mn>1</mn></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数; 等于 1",
        ),
        (
            "partial-derivative",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='partial-derivative($x)'><mi arg='x'>f</mi></mrow></math>",
            "f 的偏导数",
        ),
        (
            "sum",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x2211;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mi>k</mi><mn>2</mn></msup></math>",
            "求和 从 k 等于 1 到 n, k 平方",
        ),
        (
            "product",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x220f;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>k</mi></math>",
            "连乘 从 k 等于 1 到 n k",
        ),
        (
            "prime-derivative",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>f</mi><mo>&#x2032;</mo></msup><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "f 撇号, x, 等于 2 x",
        ),
        (
            "dot-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#x22c5;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 点乘 向量 w",
        ),
        (
            "explicit-dot-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的数量积",
        ),
        (
            "scalar-centered-dot",
            "SimpleSpeak",
            "Medium",
            "<math><mi>v</mi><mo>&#x22c5;</mo><mi>w</mi></math>",
            "v 乘 w",
        ),
        (
            "cross-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#xd7;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 叉乘 向量 w",
        ),
        (
            "explicit-cross-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的叉积",
        ),
        (
            "plain-norm",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>&#x2225;</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x2225;</mo></mrow></math>",
            "x 加 y 的范数",
        ),
        (
            "norm",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mrow><mo>&#x2225;</mo><mi>x</mi><mo>&#x2225;</mo></mrow><mn>2</mn></msub></math>",
            "x 的 2 范数",
        ),
        (
            "determinant",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "mean",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "standard-deviation",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='standard-deviation($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的标准差",
        ),
        (
            "variance",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "median",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='median($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的中位数",
        ),
        (
            "mode",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mode($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的众数",
        ),
        (
            "water-terse",
            "SimpleSpeak",
            "Terse",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 2 大写 o",
        ),
        (
            "water-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "water-verbose",
            "SimpleSpeak",
            "Verbose",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "sulfate-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow><mrow><mn>2</mn><mo>&#x2212;</mo></mrow></msup></math>",
            "左方括号, 大写 s, 大写 o, 下标 4; 右方括号 上标 2 负",
        ),
        (
            "aqueous-terse",
            "SimpleSpeak",
            "Terse",
            "<math><mi>Fe</mi><msub><mi>Cl</mi><mn>3</mn></msub><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow></math>",
            "大写 f e, 大写 c l, 3 水溶液",
        ),
        (
            "acceleration-unit",
            "SimpleSpeak",
            "Medium",
            "<math><mfrac><mrow><mn>3</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "3 米每 平方秒",
        ),
        (
            "piecewise",
            "SimpleSpeak",
            "Medium",
            "<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo><mtable><mtr><mtd><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&#x2265;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mo>&#x2212;</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
            "f x 等于; 2 个分支; 第 1 个分支; x 平方 当 x, 大于等于 0; 第 2 个分支; 负 x 当 x, 小于 0",
        ),
        (
            "system-of-equations",
            "SimpleSpeak",
            "Medium",
            "<math><mtable><mtr><mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>7</mn></mtd></mtr><mtr><mtd><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>17</mn></mtd></mtr></mtable></math>",
            "2 个方程; 第 1 个方程; x 加 y 等于 7; 第 2 个方程; 2 x 加 3 y; 等于 17",
        ),
        (
            "quadratic-formula-simple",
            "SimpleSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
        ),
        (
            "quadratic-formula-literal",
            "LiteralSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 减 b 加或减; 根号 b 上标 2 结束上标, 减 4 a c, 结束根号; 结束分数",
        ),
        (
            "binomial",
            "SimpleSpeak",
            "Medium",
            "<math><mmultiscripts><mi>C</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
            "n 取 k",
        ),
    ];

    for (name, style, verbosity, mathml, expected) in cases {
        test_prefs(
            "zh",
            style,
            vec![("Verbosity", verbosity)],
            mathml,
            expected,
        )
        .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_formula_cases_cover_common_notation() -> Result<()> {
    // Exercise complete formulas in both Chinese speech styles, not isolated symbols only.
    let cases = [
        (
            "linear-equation",
            "<math><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mo>=</mo><mn>7</mn></math>",
            "2 x 加 3, 等于 7",
            "2 x 加 3, 等于 7",
        ),
        (
            "quadratic-equation",
            "<math><mi>a</mi><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>b</mi><mi>x</mi><mo>+</mo><mi>c</mi><mo>=</mo><mn>0</mn></math>",
            "a x 平方, 加 b x 加 c; 等于 0",
            "a x 平方, 加 b x 加 c; 等于 0",
        ),
        (
            "quadratic-formula",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>−</mo><mi>b</mi><mo>±</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
            "x 等于; 分数，分子为; 根号 b 平方 减 4 a c; 分母为 2 a",
        ),
        (
            "inequality-chain",
            "<math><mrow><mo>−</mo><mn>1</mn><mo>&lt;</mo><mi>x</mi><mo>≤</mo><mn>3</mn></mrow></math>",
            "负 1 小于 x 小于等于 3",
            "负 1 小于 x 小于等于 3",
        ),
        (
            "absolute-inequality",
            "<math><mrow><mo>|</mo><mi>x</mi><mo>−</mo><mn>2</mn><mo>|</mo><mo>&lt;</mo><mn>5</mn></mrow></math>",
            "x 减 2 的绝对值, 小于 5",
            "x 减 2 的绝对值; 小于 5",
        ),
        (
            "rational-expression",
            "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>x</mi><mo>−</mo><mn>1</mn></mrow></mfrac></math>",
            "分数, x 减 1, 分之, x 加 1, 结束分数",
            "分数，分子为; x 加 1; 分母为 x 减 1",
        ),
        (
            "exponent-law",
            "<math><msup><mi>a</mi><mi>m</mi></msup><mo>×</mo><msup><mi>a</mi><mi>n</mi></msup><mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup></math>",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
        ),
        (
            "cube-root",
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mroot></math>",
            "x 加 1 的 立方根, 结束根号",
            "x 加 1 的 立方根",
        ),
        (
            "double-angle",
            "<math><mi>sin</mi><mo>⁡</mo><mrow><mo>(</mo><mn>2</mn><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡</mo><mi>x</mi><mi>cos</mi><mo>⁡</mo><mi>x</mi></math>",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
        ),
        (
            "binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow></math>",
            "n 取 k",
            "n 取 k",
        ),
        (
            "sequence-term",
            "<math><msub><mi>a</mi><mi>n</mi></msub><mo>=</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><mrow><mo>(</mo><mi>n</mi><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mi>d</mi></math>",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
        ),
        (
            "arithmetic-series",
            "<math><msub><mi>S</mi><mi>n</mi></msub><mo>=</mo><mfrac><mrow><mi>n</mi><mo>(</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub><mo>)</mo></mrow><mn>2</mn></mfrac></math>",
            "大写 s 下标 n; 等于; 分数, 2 分之, n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号, 结束分数",
            "大写 s 下标 n; 等于; 分数，分子为; n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号; 分母为 2",
        ),
        (
            "geometric-sum",
            "<math><munderover><mo>∑</mo><mrow><mi>k</mi><mo>=</mo><mn>0</mn></mrow><mi>n</mi></munderover><msup><mi>r</mi><mi>k</mi></msup></math>",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
        ),
        (
            "definite-integral",
            "<math><msubsup><mo>∫</mo><mn>0</mn><mn>1</mn></msubsup><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 1, x 平方 d x",
            "积分 从 0 到 1, x 平方 d x",
        ),
        (
            "derivative-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>3</mn><msup><mi>x</mi><mn>2</mn></msup></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 3 x 平方",
            "分数，分子为; d y; 分母为 d x; 等于 3 x 平方",
        ),
        (
            "limit-sine",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>⁡</mo><mi>x</mi></mrow><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数",
            "极限，当 x 趋于 0; x 分之 正弦 x",
        ),
        (
            "differential-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mi>k</mi><mi>y</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 k y",
            "分数，分子为; d y; 分母为 d x; 等于 k y",
        ),
        (
            "matrix-two-by-two",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "determinant-formula",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow><mo>=</mo><mi>a</mi><mi>d</mi><mo>−</mo><mi>b</mi><mi>c</mi></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
        ),
        (
            "transpose-notation",
            "<math><msup><mi>A</mi><mi>T</mi></msup></math>",
            "大写 a 的转置",
            "大写 a 的转置",
        ),
        (
            "eigen-equation",
            "<math><mi>A</mi><mi>v</mi><mo>=</mo><mi>λ</mi><mi>v</mi></math>",
            "大写 a v, 等于 拉姆达 v",
            "大写 a v, 等于 拉姆达 v",
        ),
        (
            "distance-formula",
            "<math><mi>d</mi><mo>=</mo><msqrt><msup><mrow><mo>(</mo><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup></msqrt></math>",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方 结束根号",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方",
        ),
        (
            "circle-equation",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>h</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><mi>y</mi><mo>−</mo><mi>k</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>r</mi><mn>2</mn></msup></math>",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
        ),
        (
            "pythagorean-theorem",
            "<math><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></math>",
            "a 平方 加 b 平方, 等于 c 平方",
            "a 平方 加 b 平方, 等于 c 平方",
        ),
        (
            "right-angle",
            "<math><mo>∠</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>=</mo><mn>90</mn><mo>°</mo></math>",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
        ),
        (
            "set-union-intersection",
            "<math><mi>A</mi><mo>∪</mo><mi>B</mi><mo>=</mo><mi>A</mi><mo>∩</mo><mi>B</mi></math>",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
        ),
        (
            "subset-chain",
            "<math><mi>A</mi><mo>⊂</mo><mi>B</mi><mo>⊆</mo><mi>C</mi></math>",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
        ),
        (
            "interval",
            "<math><mrow><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>)</mo></mrow></math>",
            "左闭右开区间 0 逗号 1",
            "从 0 到 1 的区间, 包含 0 但 不 包含 1",
        ),
        (
            "universal-inequality",
            "<math><mo>∀</mo><mi>x</mi><mo>∈</mo><mi>ℝ</mi><mo>,</mo><msup><mi>x</mi><mn>2</mn></msup><mo>≥</mo><mn>0</mn></math>",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
        ),
        (
            "complex-number",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi></math>",
            "z 等于 3 加 4 i",
            "z 等于 3 加 4 i",
        ),
        (
            "euler-identity",
            "<math><msup><mi>e</mi><mrow><mi>i</mi><mi>π</mi></mrow></msup><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>",
            "e 的 i 派 次方, 加 1; 等于 0",
            "e 的 i 派 次方, 加 1; 等于 0",
        ),
        (
            "speed-unit",
            "<math><mfrac><mrow><mn>72</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "72 千米每 小时",
            "72 千米每 小时",
        ),
        (
            "acceleration-unit",
            "<math><mfrac><mrow><mn>9.8</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "9.8 米每 平方秒",
            "9.8 米每 平方秒",
        ),
        (
            "temperature-unit",
            "<math><mn>20</mn><mi intent=':unit'>°C</mi></math>",
            "20 摄氏度",
            "20 摄氏度",
        ),
        (
            "chemical-water",
            "<math><mrow data-chem-formula='3'><msub><mi mathvariant='normal' data-chem-element='1'>H</mi><mn>2</mn></msub><mi mathvariant='normal' data-chem-element='1'>O</mi></mrow></math>",
            "大写 h, 下标 2, 大写 o",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "scientific-notation",
            "<math><mn>6.02</mn><mo>×</mo><msup><mn>10</mn><mn>23</mn></msup></math>",
            "6.02 乘 10 的 23 次方",
            "6.02 乘 10 的 23 次方",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_intent_cases_cover_named_terms() -> Result<()> {
    // Explicit intents provide deterministic checks for terms that presentation markup can hide.
    let cases = [
        (
            "conditional-probability",
            "<math><mrow intent='conditional-probability($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "在 大写 b 条件下 大写 a 的概率",
        ),
        (
            "gcd",
            "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>12</mi><mi arg='b'>18</mi></mrow></math>",
            "12 与 18 的最大公约数",
        ),
        (
            "lcm",
            "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>4</mi><mi arg='b'>6</mi></mrow></math>",
            "4 与 6 的最小公倍数",
        ),
        (
            "quotient",
            "<math><mrow intent='quotient($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的商",
        ),
        (
            "remainder",
            "<math><mrow intent='remainder($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的余数",
        ),
        (
            "mean",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "variance",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "complex-conjugate",
            "<math><mrow intent='complex-conjugate($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的共轭复数",
        ),
        (
            "real-part",
            "<math><mrow intent='real-part($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的实部",
        ),
        (
            "dot-product",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的数量积",
        ),
        (
            "cross-product",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的叉积",
        ),
        (
            "line-segment",
            "<math><mrow intent='line-segment($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "线段 大写 a 大写 b",
        ),
        (
            "cartesian-coordinate",
            "<math><mrow intent='cartesian-coordinate($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "直角坐标 x 逗号, y",
        ),
        (
            "trace",
            "<math><mrow intent='trace($A)'><mi arg='A'>A</mi></mrow></math>",
            "大写 a 的迹",
        ),
        (
            "kernel",
            "<math><mrow intent='kernel($T)'><mi arg='T'>T</mi></mrow></math>",
            "大写 t 的核",
        ),
        (
            "arcsine",
            "<math><mrow intent='arcsine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正弦 x",
        ),
        (
            "hyperbolic-sine",
            "<math><mi>sinh</mi><mo>⁡</mo><mi>x</mi></math>",
            "双曲正弦 x",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn additional_cn_edge_formulas_cover_uncommon_structure() -> Result<()> {
    // These formulas exercise structural branches that are easy to mis-order in Chinese speech.
    let cases = [
        (
            "polar-coordinate",
            "<math><mrow intent='polar-coordinate($x,$y)'><mi arg='x'>r</mi><mi arg='y'>θ</mi></mrow></math>",
            "极坐标 r 逗号, 西塔",
            "极坐标 r 逗号, 西塔",
        ),
        (
            "spherical-coordinate",
            "<math><mrow intent='spherical-coordinate($x,$y,$z)'><mi arg='x'>r</mi><mi arg='y'>θ</mi><mi arg='z'>φ</mi></mrow></math>",
            "球坐标 r 逗号, 西塔 逗号, 斐",
            "球坐标 r 逗号, 西塔 逗号, 斐",
        ),
        (
            "coordinate",
            "<math><mrow intent='coordinate($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
            "坐标 x 逗号, y 逗号, z",
            "坐标 x 逗号, y 逗号, z",
        ),
        (
            "open-interval-to-infinity",
            "<math><mrow><mo>(</mo><mi>c</mi><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "开区间 c 逗号 无穷大",
            "从 c 到 无穷大 的区间, 不 包含 c",
        ),
        (
            "closed-open-interval-to-infinity",
            "<math><mrow><mo>[</mo><mi>c</mi><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "左闭右开区间 c 逗号 无穷大",
            "从 c 到 无穷大 的区间, 包含 c",
        ),
        (
            "negative-infinity-to-closed",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mi>d</mi><mo>]</mo></mrow></math>",
            "左开右闭区间 负 无穷大 逗号 d",
            "从 负 无穷大 到 d 的区间, 包含 d",
        ),
        (
            "whole-real-line",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "开区间 负 无穷大 逗号 无穷大",
            "从 负 无穷大 到 无穷大 的区间",
        ),
        (
            "arccosine",
            "<math><mrow intent='arccosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反余弦 x",
            "反余弦 x",
        ),
        (
            "inverse-hyperbolic-tangent",
            "<math><mrow intent='arc-hyperbolic-tangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正切 x",
            "反双曲正切 x",
        ),
        (
            "evaluated-at",
            "<math><mrow intent='evaluated-at($x,$y)'><mi arg='x'>f</mi><mi arg='y'>2</mi></mrow></math>",
            "f 取值于 2",
            "f 取值于 2",
        ),
        (
            "power-intent",
            "<math><mrow intent='power($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "x 的 n 次方",
            "x 的 n 次方",
        ),
        (
            "mixed-partial-derivative",
            "<math><mfrac><mrow><msup><mo>∂</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>∂</mo><mi>x</mi><mo>∂</mo><mi>y</mi></mrow></mfrac></math>",
            "分数, 偏导数 x 偏导数 y, 分之, 偏导数 平方 f, 结束分数",
            "分数，分子为; 偏导数 平方 f; 分母为 偏导数 x 偏导数 y",
        ),
        (
            "third-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>3</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>3</mn></msup></mrow></mfrac></math>",
            "分数, d x 立方, 分之, d 立方 y, 结束分数",
            "分数，分子为; d 立方 y; 分母为 d x 立方",
        ),
        (
            "derivative-of-parenthesized-function",
            "<math><mfrac><mrow><mi>d</mi><mrow><mo>(</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></mrow></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
            "分数, d x, 分之, d, 左括号 f x 右括号, 结束分数",
            "分数，分子为; d, 左括号 f x 右括号; 分母为 d x",
        ),
        (
            "fractional-exponent",
            "<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>",
            "x 的 2 分之 1 次方",
            "x 的 2 分之 1 次方",
        ),
        (
            "fourth-root",
            "<math><mroot><mi>x</mi><mn>4</mn></mroot></math>",
            "x 的 4 次方根",
            "x 的 4 次方根",
        ),
        (
            "negative-cube-radicand",
            "<math><mroot><mrow><mo>−</mo><mn>8</mn></mrow><mn>3</mn></mroot></math>",
            "负 8 的 立方根, 结束根号",
            "负 8 的 立方根",
        ),
        (
            "subscript-and-superscript",
            "<math><msubsup><mi>x</mi><mi>i</mi><mi>j</mi></msubsup></math>",
            "x 下标 i, 的 j 次方",
            "x 下标 i, 的 j 次方",
        ),
        (
            "tensor-postscripts",
            "<math><mmultiscripts><mi>R</mi><mi>i</mi><none/><none/><mi>j</mi><mi>k</mi><none/></mmultiscripts></math>",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
        ),
        (
            "augmented-matrix",
            "<math><mrow><mo>[</mo><mtable columnlines='none solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
        ),
        (
            "matrix-product",
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo><mo>×</mo><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "three-branch-piecewise",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mn>0</mn><mtext> 当 </mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mo>−</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mtd></mtr></mtable></mrow></math>",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
        ),
        (
            "piecewise-without-condition",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mo>−</mo><mi>x</mi></mtd></mtr></mtable></mrow></math>",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    // ClearSpeak intentionally omits the closing marker for a nested radical; keep the
    // regression assertion on the complete SimpleSpeak reading instead.
    test(
        "zh",
        "SimpleSpeak",
        "<math><msqrt><mrow><mn>1</mn><mo>+</mo><msqrt><mi>x</mi></msqrt></mrow></msqrt></math>",
        "根号 1 加 根号 x, 结束根号",
    )?;
    Ok(())
}

#[test]
fn additional_cn_semantic_formulas_cover_probability_chemistry_and_algebra() -> Result<()> {
    // Cover named semantic branches and notation where operand order changes the meaning.
    let cases = [
        (
            "complement-event",
            "<math><mrow><mi>P</mi><mo>(</mo><mover><mi>A</mi><mo>¯</mo></mover><mo>)</mo><mo>=</mo><mn>1</mn><mo>−</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
        ),
        (
            "ordered-pair",
            "<math><mrow intent='ordered-pair($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "有序对 x 和 y",
            "有序对 x 和 y",
        ),
        (
            "cartesian-product",
            "<math><mrow intent='cartesian-product($x,$y)'><mi arg='x'>A</mi><mi arg='y'>B</mi></mrow></math>",
            "大写 a 笛卡尔积 大写 b",
            "大写 a 笛卡尔积 大写 b",
        ),
        (
            "direct-product",
            "<math><mrow intent='direct-product($x,$y)'><mi arg='x'>G</mi><mi arg='y'>H</mi></mrow></math>",
            "大写 g 直积 大写 h",
            "大写 g 直积 大写 h",
        ),
        (
            "inner-product",
            "<math><mrow intent='inner-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 内积 v",
            "u 内积 v",
        ),
        (
            "outer-product",
            "<math><mrow intent='outer-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 外积 v",
            "u 外积 v",
        ),
        (
            "rate",
            "<math><mrow intent='rate($x,$y)'><mi arg='x'>d</mi><mi arg='y'>t</mi></mrow></math>",
            "d 每 t",
            "d 每 t",
        ),
        (
            "constraint",
            "<math><mrow intent='constraint($x,$y)'><mi arg='x'>x</mi><mrow arg='y'><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mrow></math>",
            "x 条件为 x 大于 0",
            "x 条件为 x 大于 0",
        ),
        (
            "translation",
            "<math><mrow intent='translation($x,$y)'><mi arg='x'>f</mi><mi arg='y'>g</mi></mrow></math>",
            "平移 f 逗号, g",
            "平移 f 逗号, g",
        ),
        (
            "pochhammer-symbol",
            "<math><mrow intent='pochhammer($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "升阶乘 x 逗号, n",
            "升阶乘 x 逗号, n",
        ),
        (
            "directed-line-segment",
            "<math><mrow intent='directed-line-segment($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "有向线段 大写 a 大写 b",
            "有向线段 大写 a 大写 b",
        ),
        (
            "line",
            "<math><mrow intent='line($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "直线 大写 a 大写 b",
            "直线 大写 a 大写 b",
        ),
        (
            "point",
            "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>A</mi><mi arg='y'>B</mi><mi arg='z'>C</mi></mrow></math>",
            "点 大写 a 大写 b 大写 c",
            "点 大写 a 大写 b 大写 c",
        ),
        (
            "perpendicular",
            "<math><mrow intent='perpendicular($a,$b)'><mi arg='a'>l</mi><mi arg='b'>m</mi></mrow></math>",
            "l 垂直于 m",
            "l 垂直于 m",
        ),
        (
            "proportional",
            "<math><mrow intent='proportional($a,$b)'><mi arg='a'>y</mi><mi arg='b'>x</mi></mrow></math>",
            "y 正比于 x",
            "y 正比于 x",
        ),
        (
            "xor",
            "<math><mrow intent='xor($a,$b)'><mi arg='a'>p</mi><mi arg='b'>q</mi></mrow></math>",
            "p 异或 q",
            "p 异或 q",
        ),
        (
            "logical-not",
            "<math><mrow intent='not($x)'><mi arg='x'>p</mi></mrow></math>",
            "非 p",
            "非 p",
        ),
        (
            "there-does-not-exist",
            "<math><mrow intent='there-does-not-exist($x)'><mi arg='x'>x</mi></mrow></math>",
            "不存在 x",
            "不存在 x",
        ),
        (
            "evaluates-to",
            "<math><mrow intent='evaluates-to($x,$y)'><mi arg='x'>f</mi><mi arg='y'>3</mi></mrow></math>",
            "f 结果为 3",
            "f 结果为 3",
        ),
        (
            "approximately",
            "<math><mrow intent='approximately($a,$b)'><mi arg='a'>π</mi><mi arg='b'>3.14</mi></mrow></math>",
            "派 约等于 3.14",
            "派 约等于 3.14",
        ),
        (
            "identically-equals",
            "<math><mrow intent='identically-equals($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 恒等于 g",
            "f 恒等于 g",
        ),
        (
            "golden-ratio",
            "<math><mi intent='golden-ratio'>φ</mi></math>",
            "黄金分割比",
            "黄金分割比",
        ),
        (
            "upper-limit",
            "<math><mrow intent='lim-sup($x)'><mi arg='x'>a</mi></mrow></math>",
            "上极限，当 a",
            "上极限，当 a",
        ),
        (
            "lower-limit",
            "<math><mrow intent='lim-inf($x)'><mi arg='x'>a</mi></mrow></math>",
            "下极限，当 a",
            "下极限，当 a",
        ),
        (
            "unit-vector",
            "<math><mrow intent='unit-vector($x)'><mi arg='x'>v</mi></mrow></math>",
            "单位向量 v",
            "单位向量 v",
        ),
        (
            "identity-matrix",
            "<math><mi intent='identity-matrix'>I</mi></math>",
            "单位矩阵",
            "单位矩阵",
        ),
        (
            "diagonal-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>3</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
        ),
        (
            "matrix-row-separator",
            "<math><mrow><mo>[</mo><mtable rowlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
        ),
        (
            "charged-ion",
            "<math><msup><mrow><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub></mrow><mrow><mn>2</mn><mo>−</mo></mrow></msup></math>",
            "大写 s, 大写 o, 下标 4 上标 2 负",
            "大写 s, 大写 o, 下标 4 上标 2 负",
        ),
        (
            "chemical-states",
            "<math><mrow><mi>Na</mi><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow><mo>+</mo><mi>Cl</mi><mrow><mo>(</mo><mi>s</mi><mo>)</mo></mrow></mrow></math>",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
        ),
        (
            "chemical-single-bond",
            "<math><mrow><mi>C</mi><msub><mi>H</mi><mn>3</mn></msub><mo>−</mo><mi>O</mi><mi>H</mi></mrow></math>",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
        ),
        (
            "isotope-prescripts",
            "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts></math>",
            "上标 14, 下标 6, 大写 c",
            "上标 14, 下标 6, 大写 c",
        ),
        (
            "inverse-square-unit",
            "<math><mrow><mn>5</mn><msup><mi intent=':unit'>m</mi><mrow><mo>−</mo><mn>2</mn></mrow></msup></mrow></math>",
            "5 米的 负 2 次方",
            "5 米的 负 2 次方",
        ),
        (
            "micro-meter",
            "<math><mn>3</mn><mi intent=':unit'>µm</mi></math>",
            "3 微米",
            "3 微米",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    let bayes = "<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>|</mo><mi>A</mi><mo>)</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></mrow></math>";
    test(
        "zh",
        "SimpleSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数，分子为; 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率; 分母为 大写 b 的概率",
    )?;

    let mean = "<math><mover><mi>x</mi><mo>¯</mo></mover><mo>=</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        mean,
        "x 上横线, 等于, 分数, 2 分之, x 加 y, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        mean,
        "x 上横线, 等于, 分数，分子为; x 加 y; 分母为 2",
    )?;

    let variance = "<math><mi>σ</mi><mo>²</mo><mo>=</mo><mfrac><mrow><munderover><mo>∑</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mover><mi>x</mi><mo>¯</mo></mover><mo>)</mo></mrow><mn>2</mn></msup></mrow><mi>n</mi></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        variance,
        "西格马 上标 二; 等于; 分数, n 分之, 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        variance,
        "西格马 上标 二; 等于; 分数，分子为; 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方; 分母为 n",
    )?;

    let reaction = "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts><mo>→</mo><mmultiscripts><mtext>N</mtext><mprescripts/><mn>7</mn><mn>14</mn></mmultiscripts><mo>+</mo><mmultiscripts><mtext>e</mtext><mprescripts/><mrow><mo>−</mo><mn>1</mn></mrow><mn>0</mn></mmultiscripts></math>";
    for style in ["SimpleSpeak", "ClearSpeak"] {
        test_prefs(
            "zh",
            style,
            vec![("Verbosity", "Terse")],
            reaction,
            "14, 6, 大写 c; 形成, 14, 7, 大写 n; 加 0, 负 1, e",
        )
        .map_err(|error| anyhow::anyhow!("{style}/chemical-reaction: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_calculus_and_operator_cases_cover_high_risk_structure() -> Result<()> {
    // These formulas exercise operator order, limits, and nested operands that are easy to
    // misread when English word order is copied directly into Chinese.
    let cases = [
        (
            "principal-logarithm",
            "<math><mi>Log</mi><mo>⁡</mo><mi>z</mi></math>",
            "主值对数 z",
        ),
        (
            "logarithm-with-natural-base",
            "<math><msub><mi>log</mi><mi>e</mi></msub><mi>x</mi></math>",
            "以 e 为底, x 的对数",
        ),
        (
            "contour-integral",
            "<math><mo>∮</mo><mi>f</mi><mo>⁡</mo><mi>z</mi><mi>d</mi><mi>z</mi></math>",
            "围道积分 f z d z",
        ),
        (
            "surface-integral",
            "<math><mo>∯</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>S</mi></math>",
            "曲面积分 f x d 大写 s",
        ),
        (
            "volume-integral",
            "<math><mo>∰</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>V</mi></math>",
            "体积分 f x d 大写 v",
        ),
        (
            "triple-integral",
            "<math><mo>∭</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>y</mi><mi>z</mi><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi><mi>d</mi><mi>z</mi></math>",
            "三重积分 f xyzdxdydz",
        ),
        (
            "intersection-with-limits",
            "<math><munderover><mo>⋂</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元交集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "union-with-limits",
            "<math><munderover><mo>⋃</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元并集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "product-with-limits-and-expression",
            "<math><munderover><mo>∏</mo><mrow><mi>j</mi><mo>=</mo><mn>1</mn></mrow><mi>m</mi></munderover><mrow><mo>(</mo><mn>1</mn><mo>+</mo><msub><mi>x</mi><mi>j</mi></msub><mo>)</mo></mrow></math>",
            "连乘 从 j 等于 1 到 m; 左括号, 1 加 x 下标 j; 右括号",
        ),
        (
            "function-composition",
            "<math><mi>f</mi><mo>∘</mo><mi>g</mi></math>",
            "f 复合 g",
        ),
        (
            "evaluated-at-bar",
            "<math><mrow><mi>f</mi><mo>⁡</mo><mi>x</mi></mrow><msub><mo>|</mo><mn>0</mn></msub></math>",
            "f x 在 0 处的值",
        ),
        (
            "integration-by-parts",
            "<math><mo>∫</mo><mi>u</mi><mi>d</mi><mi>v</mi><mo>=</mo><mi>u</mi><mi>v</mi><mo>−</mo><mo>∫</mo><mi>v</mi><mi>d</mi><mi>u</mi></math>",
            "积分 u d v, 等于, u v 减 积分 v d u",
        ),
        (
            "binomial-expansion",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>x</mi><mi>y</mi><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></math>",
            "左括号 x 加 y 右括号 平方; 等于, x 平方 加 2 x y, 加 y 平方",
        ),
        (
            "divisibility-pair",
            "<math><mn>3</mn><mo>∣</mo><mn>12</mn><mo>,</mo><mn>5</mn><mo>∤</mo><mn>12</mn></math>",
            "3 整除 12, 逗号, 5 不整除 12",
        ),
        (
            "prime-sequence-term",
            "<math><msub><mi>p</mi><mi>n</mi></msub><mo>≥</mo><mn>2</mn></math>",
            "p 下标 n; 大于等于 2",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }

    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "分数, d x, 分之, d y, 结束分数; 等于; 分数, d u, 分之, d y, 结束分数; 分数, d x, 分之, d u, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "d x 分之 d y, 等于; d u 分之 d y; d x 分之 d u",
    )
}

#[test]
fn additional_cn_probability_linear_algebra_and_geometry_cases() -> Result<()> {
    // These are complete textbook formulas rather than isolated symbols, so operand order and
    // matrix boundaries are checked together.
    let cases = [
        (
            "covariance",
            "<math><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></math>",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
        ),
        (
            "correlation-coefficient",
            "<math><mi>ρ</mi><mo>=</mo><mfrac><mrow><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></mrow><mrow><msub><mi>σ</mi><mi>X</mi></msub><msub><mi>σ</mi><mi>Y</mi></msub></mrow></mfrac></math>",
            "柔 等于; 分数, 西格马 下标 大写 x; 西格马 下标 大写 y; 分之, 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号, 结束分数",
            "柔 等于; 分数，分子为; 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号; 分母为 西格马 下标 大写 x; 西格马 下标 大写 y",
        ),
        (
            "normal-density",
            "<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mrow><mi>σ</mi><msqrt><mn>2</mn><mi>π</mi></msqrt></mrow></mfrac><msup><mi>e</mi><mrow><mo>−</mo><mfrac><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>μ</mi><mo>)</mo></mrow><mn>2</mn></mfrac></mrow></msup></math>",
            "f x 等于; 分数, 西格马 乘, 根号 2 派 结束根号; 分之 1, 结束分数; 乘; e 的 负 分数, 2 分之, 左括号 x 减 缪, 右括号, 结束分数; 次方",
            "f x 等于; 分数，分子为 1; 分母为 根号 2 派; 乘; e 的 分数，分子为; 左括号 x 减 缪, 右括号; 分母为 2; 次方",
        ),
        (
            "binomial-probability",
            "<math><mi>P</mi><mo>(</mo><mi>X</mi><mo>=</mo><mi>k</mi><mo>)</mo><mo>=</mo><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow><msup><mi>p</mi><mi>k</mi></msup><msup><mrow><mo>(</mo><mn>1</mn><mo>−</mo><mi>p</mi><mo>)</mo></mrow><mrow><mi>n</mi><mo>−</mo><mi>k</mi></mrow></msup></math>",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
        ),
        (
            "complex-modulus",
            "<math><mo>|</mo><mi>z</mi><mo>|</mo><mo>=</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>",
            "z 的绝对值 等于, 根号 x 平方 加 y 平方 结束根号",
            "z 的绝对值, 等于, 根号 x 平方 加 y 平方",
        ),
        (
            "polar-complex-form",
            "<math><mi>z</mi><mo>=</mo><mi>r</mi><mrow><mo>(</mo><mi>cos</mi><mo>⁡</mo><mi>θ</mi><mo>+</mo><mi>i</mi><mi>sin</mi><mo>⁡</mo><mi>θ</mi><mo>)</mo></mrow></math>",
            "z 等于; r; 左括号; 余弦 西塔, 加, i 正弦 西塔; 右括号",
            "z 等于; r; 左括号; 余弦 西塔, 加, i 正弦 西塔; 右括号",
        ),
        (
            "slope-formula",
            "<math><mi>m</mi><mo>=</mo><mfrac><mrow><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub></mrow><mrow><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub></mrow></mfrac></math>",
            "m 等于; 分数, x 下标 2, 减 x 下标 1; 分之, y 下标 2, 减 y 下标 1; 结束分数",
            "m 等于; 分数，分子为; y 下标 2, 减 y 下标 1; 分母为 x 下标 2, 减 x 下标 1",
        ),
        (
            "law-of-cosines",
            "<math><msup><mi>c</mi><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>2</mn><mi>a</mi><mi>b</mi><mi>cos</mi><mo>⁡</mo><mi>C</mi></math>",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
        ),
        (
            "triangle-area",
            "<math><mi>S</mi><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>a</mi><mi>h</mi></math>",
            "大写 s 等于, 2 分之 1 a h",
            "大写 s 等于, 2 分之 1 a h",
        ),
        (
            "inverse-matrix-identity",
            "<math><msup><mi>A</mi><mrow><mo>−</mo><mn>1</mn></mrow></msup><mi>A</mi><mo>=</mo><mi>I</mi></math>",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
        ),
        (
            "trace-function-notation",
            "<math><mi>tr</mi><mo>⁡</mo><mrow><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 的迹",
            "大写 a 的迹",
        ),
        (
            "explicit-vector",
            "<math><mrow intent='vector($v)'><mover><mi arg='v'>v</mi><mo>→</mo></mover></mrow></math>",
            "向量 v",
            "向量 v",
        ),
        (
            "one-by-one-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "1 乘 1 矩阵 元素为 a",
            "1 乘 1 矩阵 元素为 a",
        ),
        (
            "three-by-three-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr><mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr><mtr><mtd><mi>g</mi></mtd><mtd><mi>h</mi></mtd><mtd><mi>i</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
        ),
        (
            "matrix-row-and-column-separators",
            "<math><mrow><mo>[</mo><mtable rowlines='solid' columnlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
        ),
        (
            "labeled-matrix",
            "<math><mrow><mo>[</mo><mtable><mlabeledtr><mtd><mi>A</mi></mtd><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mlabeledtr><mtr><mtd><mi>B</mi></mtd><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 矩阵; 第 1 行 带有标签 大写 a; a, b; 第 2 行; 大写 b, c, d",
            "2 乘 3 矩阵; 第 1 行 标签为 大写 a; a, b; 第 2 行; 大写 b, c, d",
        ),
        (
            "non-simple-binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>k</mi><mo>−</mo><mn>1</mn></mrow></mfrac><mo>)</mo></mrow></math>",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
        ),
        (
            "nested-power",
            "<math><msup><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow><mn>3</mn></msup></math>",
            "左括号 x 平方 右括号 立方",
            "左括号 x 平方 右括号 立方",
        ),
        (
            "decimal-power",
            "<math><msup><mi>x</mi><mn>2.5</mn></msup></math>",
            "x 的 2.5 次方",
            "x 的 2.5 次方",
        ),
        (
            "zero-power",
            "<math><msup><mi>x</mi><mn>0</mn></msup></math>",
            "x 的 0 次方",
            "x 的 0 次方",
        ),
        (
            "fraction-under-radical",
            "<math><msqrt><mfrac><mn>1</mn><mn>2</mn></mfrac></msqrt></math>",
            "根号 2 分之 1 结束根号",
            "根号 2 分之 1",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_units_chemistry_and_adornment_cases() -> Result<()> {
    // Units and chemical notation combine ordinary symbols with special contextual rules.
    let cases = [
        (
            "radian-unit",
            "<math><mfrac><mi>π</mi><mn>2</mn></mfrac><mi intent=':unit'>rad</mi></math>",
            "2 分之 派, 弧度",
            "2 分之 派 弧度",
        ),
        (
            "si-prefix",
            "<math><mn>5</mn><mi intent=':unit'>kN</mi></math>",
            "5 千牛顿",
            "5 千牛顿",
        ),
        (
            "density-unit",
            "<math><mfrac><mi intent=':unit'>kg</mi><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></mfrac></math>",
            "千克每 立方米",
            "千克每 立方米",
        ),
        (
            "molarity",
            "<math><mfrac><mrow><mn>0.5</mn><mi intent=':unit'>mol</mi></mrow><mi intent=':unit'>L</mi></mfrac></math>",
            "0.5 摩尔每 升",
            "0.5 摩尔每 升",
        ),
        (
            "fahrenheit-unit",
            "<math><mn>68</mn><mi intent=':unit'>°F</mi></math>",
            "68 华氏度",
            "68 华氏度",
        ),
        (
            "ohm-unit",
            "<math><mn>10</mn><mi intent=':unit'>Ω</mi></math>",
            "10 欧姆",
            "10 欧姆",
        ),
        (
            "aluminum-sulfate",
            "<math><mrow><msub><mi mathvariant='normal'>Al</mi><mn>2</mn></msub><msub><mrow><mo>(</mo><mi mathvariant='normal'>S</mi><msub><mi mathvariant='normal'>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
        ),
        (
            "precipitation-reaction",
            "<math><mrow><mi mathvariant='normal'>Ag</mi><mo>+</mo><mi mathvariant='normal'>Cl</mi><mo>→</mo><mi mathvariant='normal'>AgCl</mi><mo>↓</mo></mrow></math>",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
        ),
        (
            "reaction-condition",
            "<math><mrow><mi mathvariant='normal'>A</mi><mover><mo>→</mo><mi>Δ</mi></mover><mi mathvariant='normal'>B</mi></mrow></math>",
            "大写 a, 向右箭头 上方有 大写 德尔塔, 大写 b",
            "大写 a, 向右箭头 上方有 大写 德尔塔, 大写 b",
        ),
        (
            "hydrate-dot",
            "<math><mrow><mi mathvariant='normal'>Cu</mi><msub><mi mathvariant='normal'>SO</mi><mn>4</mn></msub><mo>·</mo><mn>5</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></mrow></math>",
            "大写 c u, 大写 s, 大写 o, 下标 4; 点, 5, 大写 h, 下标 2, 大写 o",
            "大写 c u, 大写 s, 大写 o, 下标 4; 乘, 5, 大写 h, 下标 2, 大写 o",
        ),
        (
            "chemical-triple-bond",
            "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>N</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>≡</mo><mi mathvariant='normal' data-chem-element='1'>N</mi></mrow></math>",
            "大写 n, 三键 大写 n",
            "大写 n, 三键 大写 n",
        ),
        (
            "double-prime",
            "<math><msup><mi>f</mi><mo>″</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 双撇号, x",
            "f 双撇号, x",
        ),
        (
            "triple-prime",
            "<math><msup><mi>f</mi><mo>‴</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 三撇号, x",
            "f 三撇号, x",
        ),
        (
            "hat-variable",
            "<math><mover><mi>x</mi><mo>^</mo></mover></math>",
            "x 帽符",
            "x 帽符",
        ),
        (
            "dot-variable",
            "<math><mover><mi>x</mi><mo>˙</mo></mover></math>",
            "x 上点符",
            "x 上点符",
        ),
        (
            "overbrace",
            "<math><mover><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏞</mo></mover></math>",
            "a 加 b 上方有 上置花括号",
            "a 加 b 上方有 上置花括号",
        ),
        (
            "underbrace",
            "<math><munder><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏟</mo></munder></math>",
            "a 加 b 下方有 下置花括号",
            "a 加 b 下方有 下置花括号",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_semantic_function_cases_cover_unseen_terms() -> Result<()> {
    // Explicit intents make the intended Chinese mathematical term unambiguous.
    let cases = [
        (
            "arctangent",
            "<math><mrow intent='arctangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正切 x",
        ),
        (
            "arcsecant",
            "<math><mrow intent='arcsecant($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正割 x",
        ),
        (
            "hyperbolic-cosine",
            "<math><mrow intent='hyperbolic-cosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "双曲余弦 x",
        ),
        (
            "arc-hyperbolic-sine",
            "<math><mrow intent='arc-hyperbolic-sine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正弦 x",
        ),
        (
            "tuple",
            "<math><mrow intent='tuple($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "元组 x 逗号, y",
        ),
        (
            "defined-as",
            "<math><mrow intent='defined-as($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 定义为 g",
        ),
        (
            "equivalent-to",
            "<math><mrow intent='equivalent-to($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 等价于 b",
        ),
        (
            "similar",
            "<math><mrow intent='similar($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 相似于 大写 b",
        ),
        (
            "ratio",
            "<math><mrow intent='ratio($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 比 b",
        ),
        (
            "superset",
            "<math><mrow intent='superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 超集 大写 b",
        ),
        (
            "not-superset",
            "<math><mrow intent='not-superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 非超集 大写 b",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}
