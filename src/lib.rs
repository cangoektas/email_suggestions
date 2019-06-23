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

pub fn suggestions<'a>(
  input: &'a str,
  domains: Option<&'a [&'a str]>,
) -> impl Iterator<Item = &'a str> {
  let sub_strings: Vec<&str> = input.splitn(2, "@").collect();

  if sub_strings.len() < 2 || sub_strings[0].is_empty() {
    None
  } else {
    let parsed_domain_prefix = sub_strings[1];
    Some(
      domains
        .unwrap_or(DEFAULT_DOMAINS)
        .iter()
        .filter(move |domain| domain.starts_with(parsed_domain_prefix))
        .map(move |domain| &domain[parsed_domain_prefix.len()..])
        .filter(|domain| !domain.is_empty()),
    )
  }
  .into_iter()
  .flat_map(|suggestions| suggestions)
}
