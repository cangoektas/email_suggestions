use email_suggestions::*;

#[test]
fn returns_no_suggestions_for_input_without_at_sign() {
  let expected_result: Vec<EmailSuggestion> = vec![];
  let mut actual_result: Vec<EmailSuggestion>;

  actual_result = suggestions("", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_without_local_part() {
  let expected_result: Vec<EmailSuggestion> = vec![];
  let mut actual_result: Vec<EmailSuggestion>;

  actual_result = suggestions("@", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("@g", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_input_with_multiple_at_signs() {
  let expected_result: Vec<EmailSuggestion> = vec![];
  let mut actual_result: Vec<EmailSuggestion>;

  actual_result = suggestions("foo@@", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz", None).collect();
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@bar@baz@", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_for_non_matching_domains() {
  let expected_result: Vec<EmailSuggestion> = vec![];
  let actual_result: Vec<EmailSuggestion> = suggestions("foo@bar", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_no_suggestions_when_input_fully_matches_a_domain() {
  let expected_result: Vec<EmailSuggestion> = vec![];
  let actual_result: Vec<EmailSuggestion> = suggestions("foo@gmail.com", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_default_domains_for_input_ending_with_single_at_sign() {
  let expected_result: Vec<EmailSuggestion> = vec![
    EmailSuggestion::new(String::from("foo@aol.com"), String::from("aol.com")),
    EmailSuggestion::new(String::from("foo@gmail.com"), String::from("gmail.com")),
    EmailSuggestion::new(String::from("foo@google.com"), String::from("google.com")),
    EmailSuggestion::new(String::from("foo@yahoo.com"), String::from("yahoo.com")),
    EmailSuggestion::new(String::from("foo@ymail.com"), String::from("ymail.com")),
    EmailSuggestion::new(String::from("foo@hotmail.com"), String::from("hotmail.com")),
    EmailSuggestion::new(String::from("foo@live.com"), String::from("live.com")),
    EmailSuggestion::new(String::from("foo@outlook.com"), String::from("outlook.com")),
    EmailSuggestion::new(String::from("foo@inbox.com"), String::from("inbox.com")),
    EmailSuggestion::new(String::from("foo@mail.com"), String::from("mail.com")),
    EmailSuggestion::new(String::from("foo@gmx.com"), String::from("gmx.com")),
    EmailSuggestion::new(String::from("foo@icloud.com"), String::from("icloud.com")),
  ];
  let actual_result: Vec<EmailSuggestion> = suggestions("foo@", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn returns_matching_domains() {
  let expected_result: Vec<EmailSuggestion> = vec![
    EmailSuggestion::new(String::from("foo@gmail.com"), String::from("mail.com")),
    EmailSuggestion::new(String::from("foo@google.com"), String::from("oogle.com")),
    EmailSuggestion::new(String::from("foo@gmx.com"), String::from("mx.com")),
  ];
  let actual_result: Vec<EmailSuggestion> = suggestions("foo@g", None).collect();
  assert_eq!(actual_result, expected_result);
}

#[test]
fn results_can_be_limited_with_take() {
  let mut expected_result: Vec<EmailSuggestion>;
  let mut actual_result: Vec<EmailSuggestion>;

  actual_result = suggestions("foo@g", None).take(2).collect();
  expected_result = vec![
    EmailSuggestion::new(String::from("foo@gmail.com"), String::from("mail.com")),
    EmailSuggestion::new(String::from("foo@google.com"), String::from("oogle.com")),
  ];
  assert_eq!(actual_result, expected_result);

  actual_result = suggestions("foo@y", None).take(5).collect();
  expected_result = vec![
    EmailSuggestion::new(String::from("foo@yahoo.com"), String::from("ahoo.com")),
    EmailSuggestion::new(String::from("foo@ymail.com"), String::from("mail.com")),
  ];
  assert_eq!(actual_result, expected_result);
}

#[test]
fn supports_custom_list_of_domains() {
  let expected_result: Vec<EmailSuggestion> = vec![
    EmailSuggestion::new(String::from("foo@bar.com"), String::from("ar.com")),
    EmailSuggestion::new(String::from("foo@baz.com"), String::from("az.com")),
  ];
  let actual_result: Vec<EmailSuggestion> = suggestions(
    "foo@b",
    Some(&["hello.com", "world.com", "bar.com", "baz.com"]),
  )
  .collect();
  assert_eq!(actual_result, expected_result);
}
