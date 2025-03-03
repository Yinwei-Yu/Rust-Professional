pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let birth_year = time[0..4].parse::<i32>().unwrap();
    let birth_month = time[5..7].parse::<u8>().unwrap();

    // 获取基本退休年龄和人员类型标识
    let (base_age, category) = match tp {
        "男职工" => (60, "male"),
        "原法定退休年龄55周岁女职工" => (55, "female55"),
        "原法定退休年龄50周岁女职工" => (50, "female50"),
        _ => panic!("无效人员类型"),
    };

    // 延迟退休规则表 (出生年份, 类型) -> (每年延迟月数，最大延迟年数)
    let delay_rules: Vec<((i32, i32), &str, (u8, u8))> = vec![
        ((1961, 1965), "male", (3, 3)), // 男职工1961-1965年出生，每年延迟3个月
        ((1965, 1975), "male", (4, 3)), // 男职工1965年后出生，每年延迟4个月
        ((1971, 1981), "female55", (4, 5)), // 55周岁女职工1971年后出生，每年延迟4个月
        ((1990, 2000), "female50", (6, 10)), // 50周岁女职工1990年后出生，每年延迟6个月
    ];

    // 计算延迟月数
    let (delay_months, actual_age) = delay_rules
        .iter()
        .find(|((start, end), t, _)| birth_year >= *start && birth_year <= *end && *t == category)
        .map_or((0, base_age), |(_, _, (month_step, max_years))| {
            let delay_years = (birth_year - 1960).min(*max_years as i32);
            (
                delay_years as u8 * *month_step,
                base_age + delay_years as u8,
            )
        });

    // 计算退休月份
    let total_months = birth_month as u16 + delay_months as u16;
    let (retire_year_add, retire_month) = if total_months > 12 {
        (total_months / 12, total_months % 12)
    } else {
        (0, total_months)
    };

    // 格式化输出
    format!(
        "{:04}-{:02},{:.2},{}",
        birth_year + actual_age as i32 + retire_year_add as i32,
        if retire_month == 0 { 12 } else { retire_month },
        actual_age as f32 + delay_months as f32 / 12.0,
        delay_months
    )
}
