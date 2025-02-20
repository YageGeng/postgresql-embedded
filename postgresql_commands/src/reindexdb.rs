use crate::Settings;
use crate::traits::CommandBuilder;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `reindexdb` reindexes a `PostgreSQL` database.
#[derive(Clone, Debug, Default)]
pub struct ReindexDbBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    all: bool,
    concurrently: bool,
    dbname: Option<OsString>,
    echo: bool,
    index: Option<OsString>,
    jobs: Option<u32>,
    quiet: bool,
    system: bool,
    schema: Option<OsString>,
    table: Option<OsString>,
    tablespace: Option<OsString>,
    verbose: bool,
    version: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
    maintenance_db: Option<OsString>,
}

impl ReindexDbBuilder {
    /// Create a new [`ReindexDbBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`ReindexDbBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
            .pg_password(settings.get_password())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// reindex all databases
    #[must_use]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// reindex concurrently
    #[must_use]
    pub fn concurrently(mut self) -> Self {
        self.concurrently = true;
        self
    }

    /// database to reindex
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// show the commands being sent to the server
    #[must_use]
    pub fn echo(mut self) -> Self {
        self.echo = true;
        self
    }

    /// recreate specific index(es) only
    #[must_use]
    pub fn index<S: AsRef<OsStr>>(mut self, index: S) -> Self {
        self.index = Some(index.as_ref().to_os_string());
        self
    }

    /// use this many concurrent connections to reindex
    #[must_use]
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.jobs = Some(jobs);
        self
    }

    /// don't write any messages
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// reindex system catalogs only
    #[must_use]
    pub fn system(mut self) -> Self {
        self.system = true;
        self
    }

    /// reindex specific schema(s) only
    #[must_use]
    pub fn schema<S: AsRef<OsStr>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.as_ref().to_os_string());
        self
    }

    /// reindex specific table(s) only
    #[must_use]
    pub fn table<S: AsRef<OsStr>>(mut self, table: S) -> Self {
        self.table = Some(table.as_ref().to_os_string());
        self
    }

    /// tablespace where indexes are rebuilt
    #[must_use]
    pub fn tablespace<S: AsRef<OsStr>>(mut self, tablespace: S) -> Self {
        self.tablespace = Some(tablespace.as_ref().to_os_string());
        self
    }

    /// write a lot of output
    #[must_use]
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// user name to connect as
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    #[must_use]
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt
    #[must_use]
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// user password
    #[must_use]
    pub fn pg_password<S: AsRef<OsStr>>(mut self, pg_password: S) -> Self {
        self.pg_password = Some(pg_password.as_ref().to_os_string());
        self
    }

    /// alternate maintenance database
    #[must_use]
    pub fn maintenance_db<S: AsRef<OsStr>>(mut self, maintenance_db: S) -> Self {
        self.maintenance_db = Some(maintenance_db.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for ReindexDbBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "reindexdb".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.all {
            args.push("--all".into());
        }

        if self.concurrently {
            args.push("--concurrently".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if self.echo {
            args.push("--echo".into());
        }

        if let Some(index) = &self.index {
            args.push("--index".into());
            args.push(index.into());
        }

        if let Some(jobs) = &self.jobs {
            args.push("--jobs".into());
            args.push(jobs.to_string().into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.system {
            args.push("--system".into());
        }

        if let Some(schema) = &self.schema {
            args.push("--schema".into());
            args.push(schema.into());
        }

        if let Some(table) = &self.table {
            args.push("--table".into());
            args.push(table.into());
        }

        if let Some(tablespace) = &self.tablespace {
            args.push("--tablespace".into());
            args.push(tablespace.into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        if let Some(maintenance_db) = &self.maintenance_db {
            args.push("--maintenance-db".into());
            args.push(maintenance_db.into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        let mut envs: Vec<(OsString, OsString)> = self.envs.clone();

        if let Some(password) = &self.pg_password {
            envs.push(("PGPASSWORD".into(), password.into()));
        }

        envs
    }

    /// Set an environment variable for the command
    fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestSettings;
    use crate::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = ReindexDbBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("reindexdb"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = ReindexDbBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGPASSWORD="password" "./reindexdb" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\reindexdb" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = ReindexDbBuilder::new()
            .env("PGDATABASE", "database")
            .all()
            .concurrently()
            .dbname("dbname")
            .echo()
            .index("index")
            .jobs(1)
            .quiet()
            .system()
            .schema("schema")
            .table("table")
            .tablespace("tablespace")
            .verbose()
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .username("username")
            .no_password()
            .password()
            .pg_password("password")
            .maintenance_db("maintenance-db")
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" PGPASSWORD="password" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"reindexdb" "--all" "--concurrently" "--dbname" "dbname" "--echo" "--index" "index" "--jobs" "1" "--quiet" "--system" "--schema" "schema" "--table" "table" "--tablespace" "tablespace" "--verbose" "--version" "--help" "--host" "localhost" "--port" "5432" "--username" "username" "--no-password" "--password" "--maintenance-db" "maintenance-db""#
            ),
            command.to_command_string()
        );
    }
}
