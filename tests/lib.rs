use email_suggestions::*;

#[test]
fn returns_no_suggestions_for_input_without_at_sign() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_without_local_part() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("@", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("@g", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_with_multiple_at_signs() {
  let expected_result: Vec<&str> = vec![];
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("foo@@", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz@", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_non_matching_domains() {
  let expected_result: Vec<&str> = vec![];
  let actual_result: Vec<&str> = suggestions("foo@bar", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_when_input_fully_matches_a_domain() {
  let expected_result: Vec<&str> = vec![];
  let actual_result: Vec<&str> = suggestions("foo@gmail.com", None).collect();
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
  let actual_result: Vec<&str> = suggestions("foo@", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_matching_domains() {
  let expected_result: Vec<&str> = vec!["mail.com", "oogle.com", "mx.com"];
  let actual_result: Vec<&str> = suggestions("foo@g", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn results_can_be_limited_with_take() {
  let mut expected_result: Vec<&str>;
  let mut actual_result: Vec<&str>;

  actual_result = suggestions("foo@g", None).take(2).collect();
  expected_result = vec!["mail.com", "oogle.com"];
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@y", None).take(5).collect();
  expected_result = vec!["ahoo.com", "mail.com"];
  assert_eq!(actual_result, expected_result);
}

#[test]
fn supports_custom_list_of_domains() {
  let expected_result: Vec<&str> = vec!["ar.com", "az.com"];;
  let actual_result: Vec<&str> = suggestions(
    "foo@b",
    Some(&["hello.com", "world.com", "bar.com", "baz.com"]),
  )
  .collect();
  assert_eq!(actual_result, expected_result);
}
