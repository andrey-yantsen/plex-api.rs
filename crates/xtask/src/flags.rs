xflags::xflags! {
    src "./src/flags.rs"

    /// Run custom build command.
    cmd xtask {
        /// Run the tests.
        cmd test {
            /// A tag from https://hub.docker.com/r/plexinc/pms-docker/tags to use for the tests.
            /// By default, the min supported version is used (see README.md for details).
            ///
            /// WARNING! When you specify a tag without defining `--plex-data-path`
            ///          the default path will be changed to `plex-data-{tag}`.
            ///          This allows to run tests over multiple docker tags in
            ///          parallel.
            optional --docker-tag tag: String

            /// Use the provided authentication token to run the functional tests.
            /// If provided without --online argument, the tests will run twice — once with the
            /// provided token and once without.
            optional --token token: String

            /// Use the provided authentication token to claim the server when
            /// running the functional tests.
            /// If not provided, the value from --token will be used.
            /// If provided, the server will be shared with the user who owns the main token
            /// provided via --token.
            ///
            /// If set, --token must be provided as well.
            ///
            /// You can run tests with authenticated user over an unclaimed server providing
            /// an empty --server-owner-token, while providing a valid value for --token.
            optional --server-owner-token server_owner_token: String

            /// Run only offline tests.
            optional --offline

            /// Run only online tests. When this flag is set and the token is provided, only the tests with
            /// authentication will be executed.
            optional --online

            /// The X-Plex-Client-Identifier header to use for the tests.
            optional --client-id id: String

            /// If passed, the online tests will run with the `tests_deny_unknown_fields` feature enabled.
            optional --deny-unknown-fields

            /// Path where to store the data. See `plex-data` subcommand for details.
            optional --plex-data-path path: String

            /// Test name to pass to cargo test
            optional --test-name testname: String

            /// Marks the run as triggered from GitHub Actions.
            /// To enable some additional secrets masking.
            optional --github-actions
        }

        /// Generate the data files to be fed into the Plex instance during testing.
        cmd plex-data {
            /// Replace everything with `plex-stub-data` folder.
            optional --replace

            /// Path where to store the data. Defaults to `plex-data`.
            optional -d, --plex-data-path path: String

            /// Print every copied filename.
            optional --verbose
        }

        /// Starts a plex server to modify the test data.
        cmd modify-data {
            /// A tag from https://hub.docker.com/r/plexinc/pms-docker/tags to use for the server.
            /// By default the minimum supported version is used.
            optional --docker-tag tag: String

            /// Don't replace the temporary data in plex-data with plex-stub-data when starting.
            optional --no-replace

            /// Path where to store the data. Defaults to `plex-data`.
            optional -d, --plex-data-path path: String

            /// Print every copied filename.
            optional --verbose
        }

        /// Returns a list of the latest Plex image tags from Docker Hub.
        cmd get-last-plex-tags {
            /// The number of versions to build into matrix. 3 is the default.
            optional -n, --num number: u8

            /// Jump over <number> minor versions. 1 is the default. That means in the list of `1.23.0`, `1.22.0` and
            /// `1.21.0` the version `1.22.0` will be skipped.
            optional -j, --jump number: u8

            /// Skip the specified tags. Can be used multiple times. Possible values are: `latest` or any semver-valid
            /// version. By default "beta" and "plexpass" are skipped.
            repeated --skip-tag tag: String
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Test(Test),
    PlexData(PlexData),
    ModifyData(ModifyData),
    GetLastPlexTags(GetLastPlexTags),
}

#[derive(Debug)]
pub struct Test {
    pub docker_tag: Option<String>,
    pub token: Option<String>,
    pub server_owner_token: Option<String>,
    pub offline: bool,
    pub online: bool,
    pub client_id: Option<String>,
    pub deny_unknown_fields: bool,
    pub plex_data_path: Option<String>,
    pub test_name: Option<String>,
    pub github_actions: bool,
}

#[derive(Debug)]
pub struct PlexData {
    pub replace: bool,
    pub plex_data_path: Option<String>,
    pub verbose: bool,
}

#[derive(Debug)]
pub struct ModifyData {
    pub docker_tag: Option<String>,
    pub no_replace: bool,
    pub plex_data_path: Option<String>,
    pub verbose: bool,
}

#[derive(Debug)]
pub struct GetLastPlexTags {
    pub num: Option<u8>,
    pub jump: Option<u8>,
    pub skip_tag: Vec<String>,
}

impl Xtask {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
