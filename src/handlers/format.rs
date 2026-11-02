use rust_decimal::Decimal;

pub fn format_rupiah(price: Decimal) -> String {
    let value = price.normalize().to_string();
    let (integer, fraction) = value.split_once('.').unwrap_or((&value, ""));
    let mut reversed_grouped = String::new();

    for (index, digit) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            reversed_grouped.push('.');
        }
        reversed_grouped.push(digit);
    }

    let grouped: String = reversed_grouped.chars().rev().collect();
    if fraction.is_empty() {
        format!("Rp{grouped}")
    } else {
        format!("Rp{grouped},{fraction}")
    }
}

#[cfg(test)]
pub mod tests {
    use rust_decimal::Decimal;

    use super::format_rupiah;

    #[test]
    pub fn formats_rupiah_with_thousand_separators() {
        assert_eq!(format_rupiah(Decimal::new(2500000, 2)), "Rp25.000");
        assert_eq!(format_rupiah(Decimal::new(125050, 2)), "Rp1.250,5");
    }
}
