use email_suggestions::*;

#[test]
fn suggestions_returns_no_suggestions_for_input_without_at_sign() {
  let result: Vec<&str> = vec![];
  assert_eq!(suggestions(""), result);
  assert_eq!(suggestions("foo"), result);
}

#[test]
fn suggestions_returns_no_suggestions_for_input_without_local_part() {
  let result: Vec<&str> = vec![];
  assert_eq!(suggestions("@"), result);
  assert_eq!(suggestions("@g"), result);
}

#[test]
fn suggestions_returns_no_suggestions_for_input_with_multiple_at_signs() {
  let result: Vec<&str> = vec![];
  assert_eq!(suggestions("foo@@"), result);
  assert_eq!(suggestions("foo@bar@baz"), result);
  assert_eq!(suggestions("foo@bar@baz@"), result);
}

#[test]
fn suggestions_returns_no_suggestions_for_non_matching_domains() {
  let result: Vec<&str> = vec![];
  assert_eq!(suggestions("foo@bar"), result);
}

#[test]
fn suggestions_returns_default_domains_for_input_ending_with_single_at_sign() {
  assert_eq!(
    suggestions("foo@"),
    vec![
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
    ]
  );
}

#[test]
fn suggestions_returns_all_matching_domains() {
  assert_eq!(
    suggestions("foo@g"),
    vec!["gmail.com", "google.com", "gmx.com"]
  );
}

#[test]
fn suggestionsn_returns_at_most_n_matching_domains() {
  assert_eq!(suggestionsn(2, "foo@g"), vec!["gmail.com", "google.com"]);
  assert_eq!(suggestionsn(5, "foo@y"), vec!["yahoo.com", "ymail.com"]);
}
