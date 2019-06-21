use email_suggestions::*;

#[test]
fn returns_no_suggestions_for_input_without_at_sign() {
    let result: &[&str] = &[];
    assert_eq!(get_email_suggestions(""), result);
    assert_eq!(get_email_suggestions("foo"), result);
}

#[test]
fn returns_no_suggestions_for_input_without_local_part() {
    let result: &[&str] = &[];
    assert_eq!(get_email_suggestions("@"), result);
    assert_eq!(get_email_suggestions("@g"), result);
}

#[test]
fn returns_no_suggestions_for_input_with_multiple_at_signs() {
    let result: &[&str] = &[];
    assert_eq!(get_email_suggestions("foo@@"), result);
    assert_eq!(get_email_suggestions("foo@bar@baz"), result);
    assert_eq!(get_email_suggestions("foo@bar@baz@"), result);
}

#[test]
fn returns_all_domains_for_input_ending_with_single_at_sign() {
    let result: &[&str] = &[
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
    assert_eq!(get_email_suggestions("foo@"), result);
}
