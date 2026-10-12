use task3_1_template::mirror_double;

#[test]
fn empty_vector_stays_empty() {
    assert_eq!(mirror_double(&[]), Vec::<i32>::new());
}

#[test]
fn one_positive_number_is_doubled() {
    assert_eq!(mirror_double(&[4]), vec![8]);
}

#[test]
fn zero_stays_zero() {
    assert_eq!(mirror_double(&[0]), vec![0]);
}

#[test]
fn negative_number_is_doubled() {
    assert_eq!(mirror_double(&[-7]), vec![-14]);
}

#[test]
fn two_numbers_are_reversed_and_doubled() {
    assert_eq!(mirror_double(&[2, 5]), vec![10, 4]);
}

#[test]
fn several_numbers_are_reversed_and_doubled() {
    assert_eq!(mirror_double(&[1, 2, 3, 4]), vec![8, 6, 4, 2]);
}

#[test]
fn mixed_signs_and_zero_are_handled() {
    assert_eq!(mirror_double(&[-3, 0, 6]), vec![12, 0, -6]);
}

#[test]
fn duplicate_values_are_preserved() {
    assert_eq!(mirror_double(&[3, 3, 1]), vec![2, 6, 6]);
}

#[test]
fn palindrome_order_is_still_doubled() {
    assert_eq!(mirror_double(&[1, 2, 1]), vec![2, 4, 2]);
}

#[test]
fn input_vector_is_not_modified() {
    let input = vec![1, -2, 3];
    let _ = mirror_double(&input);
    assert_eq!(input, vec![1, -2, 3]);
}

#[test]
fn safe_integer_boundaries_are_handled() {
    assert_eq!(
        mirror_double(&[i32::MIN / 2, i32::MAX / 2]),
        vec![i32::MAX - 1, i32::MIN]
    );
}

#[test]
fn repeated_zeroes_are_preserved() {
    assert_eq!(mirror_double(&[0, 0, 0]), vec![0, 0, 0]);
}
