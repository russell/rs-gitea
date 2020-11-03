
/// EditBranchProtectionOption options for editing a branch protection
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditBranchProtectionOption {
    pub approvals_whitelist_teams: Option<Vec<String>>,
    pub approvals_whitelist_username: Option<Vec<String>>,
    pub block_on_outdated_branch: Option<bool>,
    pub block_on_rejected_reviews: Option<bool>,
    pub dismiss_stale_approvals: Option<bool>,
    pub enable_approvals_whitelist: Option<bool>,
    pub enable_merge_whitelist: Option<bool>,
    pub enable_push: Option<bool>,
    pub enable_push_whitelist: Option<bool>,
    pub enable_status_check: Option<bool>,
    pub merge_whitelist_teams: Option<Vec<String>>,
    pub merge_whitelist_usernames: Option<Vec<String>>,
    pub protected_file_patterns: Option<String>,
    pub push_whitelist_deploy_keys: Option<bool>,
    pub push_whitelist_teams: Option<Vec<String>>,
    pub push_whitelist_usernames: Option<Vec<String>>,
    pub require_signed_commits: Option<bool>,
    pub required_approvals: Option<i64>,
    pub status_check_contexts: Option<Vec<String>>,
}

impl EditBranchProtectionOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditBranchProtectionOptionBuilder {
        EditBranchProtectionOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_edit_branch_protection() -> EditBranchProtectionOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingName> {
        EditBranchProtectionOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_name: core::marker::PhantomData,
        }
    }
}

impl Into<EditBranchProtectionOption> for EditBranchProtectionOptionBuilder {
    fn into(self) -> EditBranchProtectionOption {
        self.body
    }
}

impl Into<EditBranchProtectionOption> for EditBranchProtectionOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NameExists> {
    fn into(self) -> EditBranchProtectionOption {
        self.inner.body
    }
}

/// Builder for [`EditBranchProtectionOption`](./struct.EditBranchProtectionOption.html) object.
#[derive(Debug, Clone)]
pub struct EditBranchProtectionOptionBuilder {
    body: self::EditBranchProtectionOption,
}

impl EditBranchProtectionOptionBuilder {
    #[inline]
    pub fn approvals_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.approvals_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn approvals_whitelist_username(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.approvals_whitelist_username = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn block_on_outdated_branch(mut self, value: impl Into<bool>) -> Self {
        self.body.block_on_outdated_branch = Some(value.into());
        self
    }

    #[inline]
    pub fn block_on_rejected_reviews(mut self, value: impl Into<bool>) -> Self {
        self.body.block_on_rejected_reviews = Some(value.into());
        self
    }

    #[inline]
    pub fn dismiss_stale_approvals(mut self, value: impl Into<bool>) -> Self {
        self.body.dismiss_stale_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_approvals_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_approvals_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_merge_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_merge_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_push = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_push_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_status_check(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_status_check = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.merge_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn merge_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.merge_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn protected_file_patterns(mut self, value: impl Into<String>) -> Self {
        self.body.protected_file_patterns = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_deploy_keys(mut self, value: impl Into<bool>) -> Self {
        self.body.push_whitelist_deploy_keys = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.push_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn push_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.push_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn require_signed_commits(mut self, value: impl Into<bool>) -> Self {
        self.body.require_signed_commits = Some(value.into());
        self
    }

    #[inline]
    pub fn required_approvals(mut self, value: impl Into<i64>) -> Self {
        self.body.required_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn status_check_contexts(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.status_check_contexts = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`EditBranchProtectionOption::repo_edit_branch_protection`](./struct.EditBranchProtectionOption.html#method.repo_edit_branch_protection) method for a `PATCH` operation associated with `EditBranchProtectionOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditBranchProtectionOptionPatchBuilder<Owner, Repo, Name> {
    inner: EditBranchProtectionOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct EditBranchProtectionOptionPatchBuilderContainer {
    body: self::EditBranchProtectionOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_name: Option<String>,
}

impl<Owner, Repo, Name> EditBranchProtectionOptionPatchBuilder<Owner, Repo, Name> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditBranchProtectionOptionPatchBuilder<crate::generics::OwnerExists, Repo, Name> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditBranchProtectionOptionPatchBuilder<Owner, crate::generics::RepoExists, Name> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of protected branch
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> EditBranchProtectionOptionPatchBuilder<Owner, Repo, crate::generics::NameExists> {
        self.inner.param_name = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn approvals_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.approvals_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn approvals_whitelist_username(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.approvals_whitelist_username = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn block_on_outdated_branch(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.block_on_outdated_branch = Some(value.into());
        self
    }

    #[inline]
    pub fn block_on_rejected_reviews(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.block_on_rejected_reviews = Some(value.into());
        self
    }

    #[inline]
    pub fn dismiss_stale_approvals(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.dismiss_stale_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_approvals_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.enable_approvals_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_merge_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.enable_merge_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.enable_push = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.enable_push_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_status_check(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.enable_status_check = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.merge_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn merge_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.merge_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn protected_file_patterns(mut self, value: impl Into<String>) -> Self {
        self.inner.body.protected_file_patterns = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_deploy_keys(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.push_whitelist_deploy_keys = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.push_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn push_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.push_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn require_signed_commits(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.require_signed_commits = Some(value.into());
        self
    }

    #[inline]
    pub fn required_approvals(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.required_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn status_check_contexts(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.status_check_contexts = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditBranchProtectionOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NameExists> {
    type Output = crate::branch_protection::BranchProtection;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branch_protections/{name}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), name=self.inner.param_name.as_ref().expect("missing parameter name?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::branch_protection::BranchProtection, EditBranchProtectionOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}