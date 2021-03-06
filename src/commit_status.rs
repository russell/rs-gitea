
/// CommitStatus holds a single status of a single Commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitStatus {
    pub context: Option<String>,
    pub created_at: Option<String>,
    pub creator: Option<crate::user::User>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub status: Option<String>,
    pub target_url: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
}

impl CommitStatus {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitStatusBuilder {
        CommitStatusBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_statuses_by_ref() -> CommitStatusGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingRef> {
        CommitStatusGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_ref: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_statuses() -> CommitStatusGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        CommitStatusGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<CommitStatus> for CommitStatusBuilder {
    fn into(self) -> CommitStatus {
        self.body
    }
}

/// Builder for [`CommitStatus`](./struct.CommitStatus.html) object.
#[derive(Debug, Clone)]
pub struct CommitStatusBuilder {
    body: self::CommitStatus,
}

impl CommitStatusBuilder {
    #[inline]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.body.context = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn creator(mut self, value: crate::user::User) -> Self {
        self.body.creator = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.body.status = Some(value.into());
        self
    }

    #[inline]
    pub fn target_url(mut self, value: impl Into<String>) -> Self {
        self.body.target_url = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`CommitStatus::repo_list_statuses_by_ref`](./struct.CommitStatus.html#method.repo_list_statuses_by_ref) method for a `GET` operation associated with `CommitStatus`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommitStatusGetBuilder<Owner, Repo, Ref> {
    inner: CommitStatusGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_ref: core::marker::PhantomData<Ref>,
}

#[derive(Debug, Default, Clone)]
struct CommitStatusGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_ref: Option<String>,
    param_sort: Option<String>,
    param_state: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Ref> CommitStatusGetBuilder<Owner, Repo, Ref> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommitStatusGetBuilder<crate::generics::OwnerExists, Repo, Ref> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommitStatusGetBuilder<Owner, crate::generics::RepoExists, Ref> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of branch/tag/commit
    #[inline]
    pub fn ref_(mut self, value: impl Into<String>) -> CommitStatusGetBuilder<Owner, Repo, crate::generics::RefExists> {
        self.inner.param_ref = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// type of sort
    #[inline]
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.inner.param_sort = Some(value.into());
        self
    }

    /// type of state
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.param_state = Some(value.into());
        self
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommitStatusGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::RefExists> {
    type Output = Vec<CommitStatus>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/commits/{ref}/statuses", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), ref=self.inner.param_ref.as_ref().expect("missing parameter ref?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("sort", self.inner.param_sort.as_ref().map(std::string::ToString::to_string)),
            ("state", self.inner.param_state.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<Vec<CommitStatus>, CommitStatusGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::RefExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`CommitStatus::repo_list_statuses`](./struct.CommitStatus.html#method.repo_list_statuses) method for a `GET` operation associated with `CommitStatus`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommitStatusGetBuilder1<Owner, Repo, Sha> {
    inner: CommitStatusGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct CommitStatusGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
    param_sort: Option<String>,
    param_state: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Sha> CommitStatusGetBuilder1<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommitStatusGetBuilder1<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommitStatusGetBuilder1<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// sha of the commit
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> CommitStatusGetBuilder1<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// type of sort
    #[inline]
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.inner.param_sort = Some(value.into());
        self
    }

    /// type of state
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.param_state = Some(value.into());
        self
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommitStatusGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = Vec<CommitStatus>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/statuses/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("sort", self.inner.param_sort.as_ref().map(std::string::ToString::to_string)),
            ("state", self.inner.param_state.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<Vec<CommitStatus>, CommitStatusGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
