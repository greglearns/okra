/* 
 * Okta API
 *
 * Allows customers to easily access the Okta API
 *
 * OpenAPI spec version: 1.9.0
 * Contact: devex-public@okta.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryQuestionCredential {
  #[serde(rename = "answer")]
  answer: Option<String>,
  #[serde(rename = "question")]
  question: Option<String>
}

impl RecoveryQuestionCredential {
  pub fn new() -> RecoveryQuestionCredential {
    RecoveryQuestionCredential {
      answer: None,
      question: None
    }
  }

  pub fn set_answer(&mut self, answer: String) {
    self.answer = Some(answer);
  }

  pub fn with_answer(mut self, answer: String) -> RecoveryQuestionCredential {
    self.answer = Some(answer);
    self
  }

  pub fn answer(&self) -> Option<&String> {
    self.answer.as_ref()
  }

  pub fn reset_answer(&mut self) {
    self.answer = None;
  }

  pub fn set_question(&mut self, question: String) {
    self.question = Some(question);
  }

  pub fn with_question(mut self, question: String) -> RecoveryQuestionCredential {
    self.question = Some(question);
    self
  }

  pub fn question(&self) -> Option<&String> {
    self.question.as_ref()
  }

  pub fn reset_question(&mut self) {
    self.question = None;
  }

}



