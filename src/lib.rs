static DEFAULT_DOMAINS: &[&str] = &[
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

pub fn suggestions(input: &str) -> impl Iterator<Item = &str> {
  let sub_strings: Vec<&str> = input.splitn(2, "@").collect();

  if sub_strings.len() < 2 || sub_strings[0].is_empty() {
    return std::iter::empty::<&str>();
  }

  let domain = sub_strings[1];
  return DEFAULT_DOMAINS.iter().filter(|d| d.starts_with(domain));
}
