
/// Release represents a repository release
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Release {
    pub assets: Option<Vec<crate::patch_repos_owner_repo_releases_id_assets_attachment_id_response::PatchReposOwnerRepoReleasesIdAssetsAttachmentIdResponse>>,
    pub author: Option<crate::user::User>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub draft: Option<bool>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub prerelease: Option<bool>,
    pub published_at: Option<String>,
    pub tag_name: Option<String>,
    pub tarball_url: Option<String>,
    pub target_commitish: Option<String>,
    pub url: Option<String>,
    pub zipball_url: Option<String>,
}

impl Release {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ReleaseBuilder {
        ReleaseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_releases() -> ReleaseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        ReleaseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_release() -> ReleaseGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        ReleaseGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<Release> for ReleaseBuilder {
    fn into(self) -> Release {
        self.body
    }
}

/// Builder for [`Release`](./struct.Release.html) object.
#[derive(Debug, Clone)]
pub struct ReleaseBuilder {
    body: self::Release,
}

impl ReleaseBuilder {
    #[inline]
    pub fn assets(mut self, value: impl Iterator<Item = crate::patch_repos_owner_repo_releases_id_assets_attachment_id_response::PatchReposOwnerRepoReleasesIdAssetsAttachmentIdResponse>) -> Self {
        self.body.assets = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn author(mut self, value: crate::user::User) -> Self {
        self.body.author = Some(value.into());
        self
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn draft(mut self, value: impl Into<bool>) -> Self {
        self.body.draft = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn prerelease(mut self, value: impl Into<bool>) -> Self {
        self.body.prerelease = Some(value.into());
        self
    }

    #[inline]
    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.body.published_at = Some(value.into());
        self
    }

    #[inline]
    pub fn tag_name(mut self, value: impl Into<String>) -> Self {
        self.body.tag_name = Some(value.into());
        self
    }

    #[inline]
    pub fn tarball_url(mut self, value: impl Into<String>) -> Self {
        self.body.tarball_url = Some(value.into());
        self
    }

    #[inline]
    pub fn target_commitish(mut self, value: impl Into<String>) -> Self {
        self.body.target_commitish = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }

    #[inline]
    pub fn zipball_url(mut self, value: impl Into<String>) -> Self {
        self.body.zipball_url = Some(value.into());
        self
    }
}

/// Builder created by [`Release::repo_list_releases`](./struct.Release.html#method.repo_list_releases) method for a `GET` operation associated with `Release`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReleaseGetBuilder<Owner, Repo> {
    inner: ReleaseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct ReleaseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_per_page: Option<i64>,
}

impl<Owner, Repo> ReleaseGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReleaseGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReleaseGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page wants to load
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// items count every page wants to load
    #[inline]
    pub fn per_page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_per_page = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReleaseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Release>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("per_page", self.inner.param_per_page.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Release::repo_get_release`](./struct.Release.html#method.repo_get_release) method for a `GET` operation associated with `Release`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReleaseGetBuilder1<Owner, Repo, Id> {
    inner: ReleaseGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct ReleaseGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> ReleaseGetBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReleaseGetBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReleaseGetBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> ReleaseGetBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReleaseGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Release;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
