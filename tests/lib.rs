use email_suggestions::*;

#[test]
fn returns_no_suggestions_for_input_without_at_sign() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("").collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_without_local_part() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("@").collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("@g").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_with_multiple_at_signs() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("foo@@").collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz").collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz@").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_non_matching_domains() {
  let expected_result: Vec<&str> = vec![];
  let actual_result: Vec<&str> = suggestions("foo@bar").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_default_domains_for_input_ending_with_single_at_sign() {
  let expected_result: Vec<&str> = vec![
    "aol.com",
    "gmail.com",
    "google.com",
    "yahoo.com",
    "ymail.com",
    "hotmail.com",
    "live.com",
    "outlook.com",
    "inbox.com",
    "mail.com",
    "gmx.com",
    "icloud.com",
  ];
  let actual_result: Vec<&str> = suggestions("foo@").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_all_matching_domains() {
  let expected_result: Vec<&str> = vec!["gmail.com", "google.com", "gmx.com"];
  let actual_result: Vec<&str> = suggestions("foo@g").collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_results_that_can_be_limited_with_take() {
  let mut expected_result: Vec<&str>;
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("foo@g").take(2).collect();
  expected_result = vec!["gmail.com", "google.com"];
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@y").take(5).collect();
  expected_result = vec!["yahoo.com", "ymail.com"];
  assert_eq!(actual_result, expected_result);
}
