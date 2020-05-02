
/// DeployKey a deploy key
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetReposOwnerRepoKeysIdResponse {
    pub created_at: Option<String>,
    pub fingerprint: Option<String>,
    pub id: Option<i64>,
    pub key: Option<String>,
    pub key_id: Option<i64>,
    pub read_only: Option<bool>,
    pub repository: Option<crate::repository::Repository>,
    pub title: Option<String>,
    pub url: Option<String>,
}

impl GetReposOwnerRepoKeysIdResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetReposOwnerRepoKeysIdResponseBuilder {
        GetReposOwnerRepoKeysIdResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_keys() -> GetReposOwnerRepoKeysIdResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        GetReposOwnerRepoKeysIdResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_key() -> GetReposOwnerRepoKeysIdResponseGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        GetReposOwnerRepoKeysIdResponseGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<GetReposOwnerRepoKeysIdResponse> for GetReposOwnerRepoKeysIdResponseBuilder {
    fn into(self) -> GetReposOwnerRepoKeysIdResponse {
        self.body
    }
}

/// Builder for [`GetReposOwnerRepoKeysIdResponse`](./struct.GetReposOwnerRepoKeysIdResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetReposOwnerRepoKeysIdResponseBuilder {
    body: self::GetReposOwnerRepoKeysIdResponse,
}

impl GetReposOwnerRepoKeysIdResponseBuilder {
    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.body.fingerprint = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.body.key = Some(value.into());
        self
    }

    #[inline]
    pub fn key_id(mut self, value: impl Into<i64>) -> Self {
        self.body.key_id = Some(value.into());
        self
    }

    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.body.read_only = Some(value.into());
        self
    }

    #[inline]
    pub fn repository(mut self, value: crate::repository::Repository) -> Self {
        self.body.repository = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`GetReposOwnerRepoKeysIdResponse::repo_list_keys`](./struct.GetReposOwnerRepoKeysIdResponse.html#method.repo_list_keys) method for a `GET` operation associated with `GetReposOwnerRepoKeysIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetReposOwnerRepoKeysIdResponseGetBuilder<Owner, Repo> {
    inner: GetReposOwnerRepoKeysIdResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct GetReposOwnerRepoKeysIdResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_key_id: Option<i64>,
    param_fingerprint: Option<String>,
}

impl<Owner, Repo> GetReposOwnerRepoKeysIdResponseGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GetReposOwnerRepoKeysIdResponseGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GetReposOwnerRepoKeysIdResponseGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// the key_id to search for
    #[inline]
    pub fn key_id(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_key_id = Some(value.into());
        self
    }

    /// fingerprint of the key
    #[inline]
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.inner.param_fingerprint = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetReposOwnerRepoKeysIdResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<GetReposOwnerRepoKeysIdResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/keys", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("key_id", self.inner.param_key_id.as_ref().map(std::string::ToString::to_string)),
            ("fingerprint", self.inner.param_fingerprint.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`GetReposOwnerRepoKeysIdResponse::repo_get_key`](./struct.GetReposOwnerRepoKeysIdResponse.html#method.repo_get_key) method for a `GET` operation associated with `GetReposOwnerRepoKeysIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetReposOwnerRepoKeysIdResponseGetBuilder1<Owner, Repo, Id> {
    inner: GetReposOwnerRepoKeysIdResponseGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GetReposOwnerRepoKeysIdResponseGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> GetReposOwnerRepoKeysIdResponseGetBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GetReposOwnerRepoKeysIdResponseGetBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GetReposOwnerRepoKeysIdResponseGetBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the key to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> GetReposOwnerRepoKeysIdResponseGetBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetReposOwnerRepoKeysIdResponseGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = GetReposOwnerRepoKeysIdResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/keys/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
