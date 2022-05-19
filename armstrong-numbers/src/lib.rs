
pub fn is_armstrong_number(num: u32) -> bool {
    let non_powered_numbers: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let splited_number_array_lenght = non_powered_numbers.len() as u32;

    let mut powered_numbers_sum: u32 = 0;

    for number in non_powered_numbers {
        powered_numbers_sum += u32::pow(number, splited_number_array_lenght);
    }

    num == powered_numbers_sum
}
