use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
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
