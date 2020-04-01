/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Issue : Issue represents an issue in a repository

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
  #[serde(rename = "assignee")]
  assignee: Option<::models::User>,
  #[serde(rename = "assignees")]
  assignees: Option<Vec<::models::User>>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "closed_at")]
  closed_at: Option<String>,
  #[serde(rename = "comments")]
  comments: Option<i64>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "due_date")]
  due_date: Option<String>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "labels")]
  labels: Option<Vec<::models::Label>>,
  #[serde(rename = "milestone")]
  milestone: Option<::models::Milestone>,
  #[serde(rename = "number")]
  number: Option<i64>,
  #[serde(rename = "original_author")]
  original_author: Option<String>,
  #[serde(rename = "original_author_id")]
  original_author_id: Option<i64>,
  #[serde(rename = "pull_request")]
  pull_request: Option<::models::PullRequestMeta>,
  #[serde(rename = "repository")]
  repository: Option<::models::RepositoryMeta>,
  #[serde(rename = "state")]
  state: Option<::models::StateType>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::User>
}

impl Issue {
  /// Issue represents an issue in a repository
  pub fn new() -> Issue {
    Issue {
      assignee: None,
      assignees: None,
      body: None,
      closed_at: None,
      comments: None,
      created_at: None,
      due_date: None,
      id: None,
      labels: None,
      milestone: None,
      number: None,
      original_author: None,
      original_author_id: None,
      pull_request: None,
      repository: None,
      state: None,
      title: None,
      updated_at: None,
      url: None,
      user: None
    }
  }

  pub fn set_assignee(&mut self, assignee: ::models::User) {
    self.assignee = Some(assignee);
  }

  pub fn with_assignee(mut self, assignee: ::models::User) -> Issue {
    self.assignee = Some(assignee);
    self
  }

  pub fn assignee(&self) -> Option<&::models::User> {
    self.assignee.as_ref()
  }

  pub fn reset_assignee(&mut self) {
    self.assignee = None;
  }

  pub fn set_assignees(&mut self, assignees: Vec<::models::User>) {
    self.assignees = Some(assignees);
  }

  pub fn with_assignees(mut self, assignees: Vec<::models::User>) -> Issue {
    self.assignees = Some(assignees);
    self
  }

  pub fn assignees(&self) -> Option<&Vec<::models::User>> {
    self.assignees.as_ref()
  }

  pub fn reset_assignees(&mut self) {
    self.assignees = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> Issue {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_closed_at(&mut self, closed_at: String) {
    self.closed_at = Some(closed_at);
  }

  pub fn with_closed_at(mut self, closed_at: String) -> Issue {
    self.closed_at = Some(closed_at);
    self
  }

  pub fn closed_at(&self) -> Option<&String> {
    self.closed_at.as_ref()
  }

  pub fn reset_closed_at(&mut self) {
    self.closed_at = None;
  }

  pub fn set_comments(&mut self, comments: i64) {
    self.comments = Some(comments);
  }

  pub fn with_comments(mut self, comments: i64) -> Issue {
    self.comments = Some(comments);
    self
  }

  pub fn comments(&self) -> Option<&i64> {
    self.comments.as_ref()
  }

  pub fn reset_comments(&mut self) {
    self.comments = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Issue {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_due_date(&mut self, due_date: String) {
    self.due_date = Some(due_date);
  }

  pub fn with_due_date(mut self, due_date: String) -> Issue {
    self.due_date = Some(due_date);
    self
  }

  pub fn due_date(&self) -> Option<&String> {
    self.due_date.as_ref()
  }

  pub fn reset_due_date(&mut self) {
    self.due_date = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Issue {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_labels(&mut self, labels: Vec<::models::Label>) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: Vec<::models::Label>) -> Issue {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&Vec<::models::Label>> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_milestone(&mut self, milestone: ::models::Milestone) {
    self.milestone = Some(milestone);
  }

  pub fn with_milestone(mut self, milestone: ::models::Milestone) -> Issue {
    self.milestone = Some(milestone);
    self
  }

  pub fn milestone(&self) -> Option<&::models::Milestone> {
    self.milestone.as_ref()
  }

  pub fn reset_milestone(&mut self) {
    self.milestone = None;
  }

  pub fn set_number(&mut self, number: i64) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: i64) -> Issue {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&i64> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_original_author(&mut self, original_author: String) {
    self.original_author = Some(original_author);
  }

  pub fn with_original_author(mut self, original_author: String) -> Issue {
    self.original_author = Some(original_author);
    self
  }

  pub fn original_author(&self) -> Option<&String> {
    self.original_author.as_ref()
  }

  pub fn reset_original_author(&mut self) {
    self.original_author = None;
  }

  pub fn set_original_author_id(&mut self, original_author_id: i64) {
    self.original_author_id = Some(original_author_id);
  }

  pub fn with_original_author_id(mut self, original_author_id: i64) -> Issue {
    self.original_author_id = Some(original_author_id);
    self
  }

  pub fn original_author_id(&self) -> Option<&i64> {
    self.original_author_id.as_ref()
  }

  pub fn reset_original_author_id(&mut self) {
    self.original_author_id = None;
  }

  pub fn set_pull_request(&mut self, pull_request: ::models::PullRequestMeta) {
    self.pull_request = Some(pull_request);
  }

  pub fn with_pull_request(mut self, pull_request: ::models::PullRequestMeta) -> Issue {
    self.pull_request = Some(pull_request);
    self
  }

  pub fn pull_request(&self) -> Option<&::models::PullRequestMeta> {
    self.pull_request.as_ref()
  }

  pub fn reset_pull_request(&mut self) {
    self.pull_request = None;
  }

  pub fn set_repository(&mut self, repository: ::models::RepositoryMeta) {
    self.repository = Some(repository);
  }

  pub fn with_repository(mut self, repository: ::models::RepositoryMeta) -> Issue {
    self.repository = Some(repository);
    self
  }

  pub fn repository(&self) -> Option<&::models::RepositoryMeta> {
    self.repository.as_ref()
  }

  pub fn reset_repository(&mut self) {
    self.repository = None;
  }

  pub fn set_state(&mut self, state: ::models::StateType) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::StateType) -> Issue {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::StateType> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> Issue {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> Issue {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Issue {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_user(&mut self, user: ::models::User) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::User) -> Issue {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::User> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}


