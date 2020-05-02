
/// MergePullRequestForm form for merging Pull Request
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MergePullRequestOption {
    #[serde(rename = "MergeMessageField")]
    pub merge_message_field: Option<String>,
    #[serde(rename = "MergeTitleField")]
    pub merge_title_field: Option<String>,
}

impl MergePullRequestOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> MergePullRequestOptionBuilder {
        MergePullRequestOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_merge_pull_request() -> MergePullRequestOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        MergePullRequestOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<MergePullRequestOption> for MergePullRequestOptionBuilder {
    fn into(self) -> MergePullRequestOption {
        self.body
    }
}

impl Into<MergePullRequestOption> for MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> MergePullRequestOption {
        self.inner.body
    }
}

/// Builder for [`MergePullRequestOption`](./struct.MergePullRequestOption.html) object.
#[derive(Debug, Clone)]
pub struct MergePullRequestOptionBuilder {
    body: self::MergePullRequestOption,
}

impl MergePullRequestOptionBuilder {
    #[inline]
    pub fn merge_message_field(mut self, value: impl Into<String>) -> Self {
        self.body.merge_message_field = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_title_field(mut self, value: impl Into<String>) -> Self {
        self.body.merge_title_field = Some(value.into());
        self
    }
}

/// Builder created by [`MergePullRequestOption::repo_merge_pull_request`](./struct.MergePullRequestOption.html#method.repo_merge_pull_request) method for a `POST` operation associated with `MergePullRequestOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MergePullRequestOptionPostBuilder<Owner, Repo, Index> {
    inner: MergePullRequestOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct MergePullRequestOptionPostBuilderContainer {
    body: self::MergePullRequestOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> MergePullRequestOptionPostBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> MergePullRequestOptionPostBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request to merge
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> MergePullRequestOptionPostBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn merge_message_field(mut self, value: impl Into<String>) -> Self {
        self.inner.body.merge_message_field = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_title_field(mut self, value: impl Into<String>) -> Self {
        self.inner.body.merge_title_field = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/merge", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}