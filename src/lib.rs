static ALL_DOMAINS: &[&str] = &[
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

pub fn get_email_suggestions(input: &str) -> &[&str] {
  if input.contains("@") {
    ALL_DOMAINS
  } else {
    &[]
  }
}
