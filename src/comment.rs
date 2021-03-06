
/// Comment represents a comment on a commit or issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub issue_url: Option<String>,
    pub original_author: Option<String>,
    pub original_author_id: Option<i64>,
    pub pull_request_url: Option<String>,
    pub updated_at: Option<String>,
    pub user: Option<crate::user::User>,
}

impl Comment {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommentBuilder {
        CommentBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_get_repo_comments() -> CommentGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        CommentGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_comment() -> CommentGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        CommentGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_comments() -> CommentGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        CommentGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<Comment> for CommentBuilder {
    fn into(self) -> Comment {
        self.body
    }
}

/// Builder for [`Comment`](./struct.Comment.html) object.
#[derive(Debug, Clone)]
pub struct CommentBuilder {
    body: self::Comment,
}

impl CommentBuilder {
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
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn issue_url(mut self, value: impl Into<String>) -> Self {
        self.body.issue_url = Some(value.into());
        self
    }

    #[inline]
    pub fn original_author(mut self, value: impl Into<String>) -> Self {
        self.body.original_author = Some(value.into());
        self
    }

    #[inline]
    pub fn original_author_id(mut self, value: impl Into<i64>) -> Self {
        self.body.original_author_id = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_request_url(mut self, value: impl Into<String>) -> Self {
        self.body.pull_request_url = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`Comment::issue_get_repo_comments`](./struct.Comment.html#method.issue_get_repo_comments) method for a `GET` operation associated with `Comment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommentGetBuilder<Owner, Repo> {
    inner: CommentGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct CommentGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> CommentGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommentGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommentGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// if provided, only comments updated since the provided time are returned.
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.inner.param_since = Some(value.into());
        self
    }

    /// if provided, only comments updated before the provided time are returned.
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.inner.param_before = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommentGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Comment>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("since", self.inner.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.inner.param_before.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Comment::issue_get_comment`](./struct.Comment.html#method.issue_get_comment) method for a `GET` operation associated with `Comment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommentGetBuilder1<Owner, Repo, Id> {
    inner: CommentGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct CommentGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> CommentGetBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommentGetBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommentGetBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> CommentGetBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommentGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Comment;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

impl crate::client::ResponseWrapper<Comment, CommentGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Comment::issue_get_comments`](./struct.Comment.html#method.issue_get_comments) method for a `GET` operation associated with `Comment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommentGetBuilder2<Owner, Repo, Index> {
    inner: CommentGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct CommentGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_since: Option<String>,
    param_before: Option<String>,
}

impl<Owner, Repo, Index> CommentGetBuilder2<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommentGetBuilder2<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommentGetBuilder2<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> CommentGetBuilder2<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// if provided, only comments updated since the specified time are returned.
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.inner.param_since = Some(value.into());
        self
    }

    /// if provided, only comments updated before the provided time are returned.
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.inner.param_before = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommentGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<Comment>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/comments", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("since", self.inner.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.inner.param_before.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
