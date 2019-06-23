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

pub struct EmailSuggestion<'a> {
  complete_address: &'a str,
  text_to_append: &'a str,
}

impl<'a> EmailSuggestion<'a> {
  fn new(complete_address: &'a str, text_to_append: &'a str) -> EmailSuggestion<'a> {
    EmailSuggestion {
      complete_address,
      text_to_append,
    }
  }
}

pub fn suggestions<'a>(
  input: &'a str,
  domains: Option<&'a [&'a str]>,
) -> impl Iterator<Item = EmailSuggestion<'a>> {
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
        .filter(|text_to_append| !text_to_append.is_empty())
        .map(move |text_to_append| {
          let mut complete_address = input.to_owned();
          complete_address.push_str(text_to_append);
          EmailSuggestion::new(&complete_address, text_to_append)
        }),
    )
  }
  .into_iter()
  .flat_map(|suggestions| suggestions)
}
