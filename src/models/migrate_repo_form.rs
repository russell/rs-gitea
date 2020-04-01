/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MigrateRepoForm : MigrateRepoForm form for migrating repository

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrateRepoForm {
  #[serde(rename = "auth_password")]
  auth_password: Option<String>,
  #[serde(rename = "auth_username")]
  auth_username: Option<String>,
  #[serde(rename = "clone_addr")]
  clone_addr: String,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "issues")]
  issues: Option<bool>,
  #[serde(rename = "labels")]
  labels: Option<bool>,
  #[serde(rename = "milestones")]
  milestones: Option<bool>,
  #[serde(rename = "mirror")]
  mirror: Option<bool>,
  #[serde(rename = "private")]
  private: Option<bool>,
  #[serde(rename = "pull_requests")]
  pull_requests: Option<bool>,
  #[serde(rename = "releases")]
  releases: Option<bool>,
  #[serde(rename = "repo_name")]
  repo_name: String,
  #[serde(rename = "uid")]
  uid: i64,
  #[serde(rename = "wiki")]
  wiki: Option<bool>
}

impl MigrateRepoForm {
  /// MigrateRepoForm form for migrating repository
  pub fn new(clone_addr: String, repo_name: String, uid: i64) -> MigrateRepoForm {
    MigrateRepoForm {
      auth_password: None,
      auth_username: None,
      clone_addr: clone_addr,
      description: None,
      issues: None,
      labels: None,
      milestones: None,
      mirror: None,
      private: None,
      pull_requests: None,
      releases: None,
      repo_name: repo_name,
      uid: uid,
      wiki: None
    }
  }

  pub fn set_auth_password(&mut self, auth_password: String) {
    self.auth_password = Some(auth_password);
  }

  pub fn with_auth_password(mut self, auth_password: String) -> MigrateRepoForm {
    self.auth_password = Some(auth_password);
    self
  }

  pub fn auth_password(&self) -> Option<&String> {
    self.auth_password.as_ref()
  }

  pub fn reset_auth_password(&mut self) {
    self.auth_password = None;
  }

  pub fn set_auth_username(&mut self, auth_username: String) {
    self.auth_username = Some(auth_username);
  }

  pub fn with_auth_username(mut self, auth_username: String) -> MigrateRepoForm {
    self.auth_username = Some(auth_username);
    self
  }

  pub fn auth_username(&self) -> Option<&String> {
    self.auth_username.as_ref()
  }

  pub fn reset_auth_username(&mut self) {
    self.auth_username = None;
  }

  pub fn set_clone_addr(&mut self, clone_addr: String) {
    self.clone_addr = clone_addr;
  }

  pub fn with_clone_addr(mut self, clone_addr: String) -> MigrateRepoForm {
    self.clone_addr = clone_addr;
    self
  }

  pub fn clone_addr(&self) -> &String {
    &self.clone_addr
  }


  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> MigrateRepoForm {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_issues(&mut self, issues: bool) {
    self.issues = Some(issues);
  }

  pub fn with_issues(mut self, issues: bool) -> MigrateRepoForm {
    self.issues = Some(issues);
    self
  }

  pub fn issues(&self) -> Option<&bool> {
    self.issues.as_ref()
  }

  pub fn reset_issues(&mut self) {
    self.issues = None;
  }

  pub fn set_labels(&mut self, labels: bool) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: bool) -> MigrateRepoForm {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&bool> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_milestones(&mut self, milestones: bool) {
    self.milestones = Some(milestones);
  }

  pub fn with_milestones(mut self, milestones: bool) -> MigrateRepoForm {
    self.milestones = Some(milestones);
    self
  }

  pub fn milestones(&self) -> Option<&bool> {
    self.milestones.as_ref()
  }

  pub fn reset_milestones(&mut self) {
    self.milestones = None;
  }

  pub fn set_mirror(&mut self, mirror: bool) {
    self.mirror = Some(mirror);
  }

  pub fn with_mirror(mut self, mirror: bool) -> MigrateRepoForm {
    self.mirror = Some(mirror);
    self
  }

  pub fn mirror(&self) -> Option<&bool> {
    self.mirror.as_ref()
  }

  pub fn reset_mirror(&mut self) {
    self.mirror = None;
  }

  pub fn set_private(&mut self, private: bool) {
    self.private = Some(private);
  }

  pub fn with_private(mut self, private: bool) -> MigrateRepoForm {
    self.private = Some(private);
    self
  }

  pub fn private(&self) -> Option<&bool> {
    self.private.as_ref()
  }

  pub fn reset_private(&mut self) {
    self.private = None;
  }

  pub fn set_pull_requests(&mut self, pull_requests: bool) {
    self.pull_requests = Some(pull_requests);
  }

  pub fn with_pull_requests(mut self, pull_requests: bool) -> MigrateRepoForm {
    self.pull_requests = Some(pull_requests);
    self
  }

  pub fn pull_requests(&self) -> Option<&bool> {
    self.pull_requests.as_ref()
  }

  pub fn reset_pull_requests(&mut self) {
    self.pull_requests = None;
  }

  pub fn set_releases(&mut self, releases: bool) {
    self.releases = Some(releases);
  }

  pub fn with_releases(mut self, releases: bool) -> MigrateRepoForm {
    self.releases = Some(releases);
    self
  }

  pub fn releases(&self) -> Option<&bool> {
    self.releases.as_ref()
  }

  pub fn reset_releases(&mut self) {
    self.releases = None;
  }

  pub fn set_repo_name(&mut self, repo_name: String) {
    self.repo_name = repo_name;
  }

  pub fn with_repo_name(mut self, repo_name: String) -> MigrateRepoForm {
    self.repo_name = repo_name;
    self
  }

  pub fn repo_name(&self) -> &String {
    &self.repo_name
  }


  pub fn set_uid(&mut self, uid: i64) {
    self.uid = uid;
  }

  pub fn with_uid(mut self, uid: i64) -> MigrateRepoForm {
    self.uid = uid;
    self
  }

  pub fn uid(&self) -> &i64 {
    &self.uid
  }


  pub fn set_wiki(&mut self, wiki: bool) {
    self.wiki = Some(wiki);
  }

  pub fn with_wiki(mut self, wiki: bool) -> MigrateRepoForm {
    self.wiki = Some(wiki);
    self
  }

  pub fn wiki(&self) -> Option<&bool> {
    self.wiki.as_ref()
  }

  pub fn reset_wiki(&mut self) {
    self.wiki = None;
  }

}


