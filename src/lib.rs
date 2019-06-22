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

pub fn suggestions(input: &str) -> Vec<&str> {
  return suggestionsn((*DEFAULT_DOMAINS).len(), input);
}

pub fn suggestionsn(n: usize, input: &str) -> Vec<&str> {
  let sub_strings: Vec<&str> = input.splitn(2, "@").collect();

  if sub_strings.len() < 2 || sub_strings[0].is_empty() {
    return vec![];
  }

  let domain = sub_strings[1];
  let matching_domains: Vec<&str> = (*DEFAULT_DOMAINS)
    .iter()
    .map(|x| *x)
    .filter(|x| x.starts_with(domain))
    .take(n)
    .collect();

  return matching_domains;
}
