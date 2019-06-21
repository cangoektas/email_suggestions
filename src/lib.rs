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
  let mut result = vec![];
  for c in input.split("@") {
    result.push(c);
  }
  // If we encountered more than one @-sign, result will have > 2 items.
  println!("input={}; result={};", input, format!("{:?}", result));
  if input.contains("@") {
    ALL_DOMAINS
  } else {
    &[]
  }
}
