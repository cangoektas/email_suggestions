use email_suggestions::*;

#[test]
fn returns_no_suggestions_for_input_without_at_sign() {
    let result: Vec<&str> = vec![];
    assert_eq!(suggestions(""), result);
    assert_eq!(suggestions("foo"), result);
}

#[test]
fn returns_no_suggestions_for_input_without_local_part() {
    let result: Vec<&str> = vec![];
    assert_eq!(suggestions("@"), result);
    assert_eq!(suggestions("@g"), result);
}

#[test]
fn returns_no_suggestions_for_input_with_multiple_at_signs() {
    let result: Vec<&str> = vec![];
    assert_eq!(suggestions("foo@@"), result);
    assert_eq!(suggestions("foo@bar@baz"), result);
    assert_eq!(suggestions("foo@bar@baz@"), result);
}

#[test]
fn returns_default_domains_for_input_ending_with_single_at_sign() {
    let result: Vec<&str> = vec![
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
    assert_eq!(suggestions("foo@"), result);
}
