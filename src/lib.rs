static DEFAULT_DOMAINS: &'static [&'static str] = &[
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

#[derive(PartialEq, Eq, Debug)]
pub struct EmailSuggestion {
  pub complete_address: String,
  pub text_to_append: String,
}

impl EmailSuggestion {
  pub fn new(complete_address: String, text_to_append: String) -> EmailSuggestion {
    EmailSuggestion {
      complete_address,
      text_to_append,
    }
  }
}

pub fn suggestions<'a>(
  input: &'a str,
  domains: Option<&'a [&'a str]>,
) -> impl Iterator<Item = EmailSuggestion> + 'a {
  let sub_strings: Vec<&'a str> = input.splitn(2, "@").collect();

  if sub_strings.len() < 2 || sub_strings[0].is_empty() {
    None
  } else {
    let parsed_domain_prefix: &'a str = sub_strings[1];
    let domains: &'a [&'a str] = domains.unwrap_or(DEFAULT_DOMAINS);
    Some(
      domains
        .iter()
        .filter(move |domain| {
          parsed_domain_prefix.len() < domain.len() && domain.starts_with(parsed_domain_prefix)
        })
        .map(move |domain| &domain[parsed_domain_prefix.len()..])
        .map(move |text_to_append| {
          let mut complete_address: String = input.to_owned();
          complete_address.push_str(text_to_append);
          let text_to_append: String = text_to_append.to_owned();

          EmailSuggestion::new(complete_address, text_to_append)
        }),
    )
  }
  .into_iter()
  .flat_map(|suggestions| suggestions)
}
