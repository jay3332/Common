/// JSON accepted for POST /api/v0/users/ (create user)
#[derive(Serialize, Deserialize)]
pub struct UserCreateJson {
    /// User's preferred username.
    pub username: String,

    /// User's email.
    ///
    /// Note: no serverside validation is done on this string.
    /// This allows for anonymous signup, but means account recovery is
    /// impossible if a invalid email is given
    pub email: String,

    /// User's password.
    ///
    /// Password must be at least one Unicode codepoint.
    /// No restrictions are applied besides that.
    pub password: String,
}

/// JSON accepted for PATCH /api/v0/users/{user_id}
#[derive(Serialize, Deserialize)]
pub struct UserUpdateJson {
    /// Users's preferred username.
    pub username: Option<String>,

    /// User's email.
    ///
    /// Note: no serverside validation is done on this string.
    /// This allows for anonymous signup, but means account recovery is
    /// impossible if a invalid email is given
    pub email: Option<String>,

    /// User's avatar, base64 encoded.
    pub avatar: Option<String>,

    /// User's Password.
    ///
    /// Password must be at least one Unicode codepoint.
    /// No restrictions are applied besides that.
    pub password: Option<String>,
}

/// JSON accepted for DELETE /api/v0/users/{user_id}
#[derive(Serialize, Deserialize)]
pub struct UserDeleteJson {
    /// Always set this to true.
    pub are_you_sure: bool,

    /// Also always set this to true
    pub are_you_really_sure: bool,

    /// Just for fun, if you want, set this to true.
    pub oletko_varma: Option<bool>,
}
