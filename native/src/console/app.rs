#![allow(clippy::large_enum_variant)]

use std::path::PathBuf;

use clap::{ArgAction, Parser};
use ulid::Ulid;
use url::Url;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
#[clap(propagate_version = true)]
#[clap(version)]
pub enum App {
    /// Manage web apps
    #[clap(subcommand)]
    Site(SiteCommand),

    /// Manage profiles
    #[clap(subcommand)]
    Profile(ProfileCommand),

    /// Manage the runtime
    #[clap(subcommand)]
    Runtime(RuntimeCommand),
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub enum SiteCommand {
    /// Launch a web app
    Launch(SiteLaunchCommand),

    /// Install a web app
    Install(SiteInstallCommand),

    /// Uninstall a web app
    Uninstall(SiteUninstallCommand),

    /// Update a web app
    Update(SiteUpdateCommand),
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct SiteLaunchCommand {
    /// Web app ID
    pub id: Ulid,

    /// Arguments passed to the runtime
    pub arguments: Vec<String>,

    /// Launch web app on a custom start URL
    #[clap(long, conflicts_with = "protocol")]
    pub url: Option<Url>,

    /// Launch web app on a protocol handler URL
    #[clap(long, conflicts_with = "url")]
    pub protocol: Option<Option<Url>>,

    /// Internal: Directly launch web app without system integration
    #[cfg(target_os = "macos")]
    #[clap(long, hide = true)]
    pub direct_launch: bool,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct SiteInstallCommand {
    /// Direct URL of the site's web app manifest
    pub manifest_url: Url,

    /// Direct URL of the site's main document
    /// {n}Defaults to the result of parsing a manifest URL with `.`
    #[clap(long)]
    pub document_url: Option<Url>,

    /// Profile where this web app will be installed
    /// {n}Defaults to the shared profile
    #[clap(long)]
    pub profile: Option<Ulid>,

    /// Set a custom web app start URL
    #[clap(long)]
    pub start_url: Option<Url>,

    /// Set a custom web app icon URL
    #[clap(long)]
    pub icon_url: Option<Url>,

    /// Set a custom web app name
    #[clap(long)]
    pub name: Option<String>,

    /// Set a custom web app description
    #[clap(long)]
    pub description: Option<String>,

    /// Set custom web app categories
    #[clap(long)]
    pub categories: Option<Vec<String>>,

    /// Set custom web app keywords
    #[clap(long)]
    pub keywords: Option<Vec<String>>,

    /// Disable system integration
    #[clap(long = "no-system-integration", action = ArgAction::SetFalse)]
    pub system_integration: bool,

    /// Configuration of the HTTP client.
    #[clap(flatten)]
    pub client: HTTPClientConfig,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct SiteUninstallCommand {
    /// Web app ID
    pub id: Ulid,

    /// Disable any interactive prompts
    #[clap(short, long)]
    pub quiet: bool,

    /// Disable system integration
    #[clap(long = "no-system-integration", action = ArgAction::SetFalse)]
    pub system_integration: bool,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct SiteUpdateCommand {
    /// Web app ID
    pub id: Ulid,

    /// Set a custom web app start URL
    #[clap(long)]
    pub start_url: Option<Option<Url>>,

    /// Set a custom web app icon URL
    #[clap(long)]
    pub icon_url: Option<Option<Url>>,

    /// Set a custom web app name
    #[clap(long)]
    pub name: Option<Option<String>>,

    /// Set a custom web app description
    #[clap(long)]
    pub description: Option<Option<String>>,

    /// Set custom web app categories
    #[clap(long)]
    pub categories: Option<Vec<String>>,

    /// Set custom web app keywords
    #[clap(long)]
    pub keywords: Option<Vec<String>>,

    /// Set enabled URL handlers
    #[clap(long)]
    pub enabled_url_handlers: Option<Vec<String>>,

    /// Set enabled protocol handlers
    #[clap(long)]
    pub enabled_protocol_handlers: Option<Vec<String>>,

    /// Disable manifest updates
    #[clap(long = "no-manifest-updates", action = ArgAction::SetFalse)]
    pub update_manifest: bool,

    /// Disable icon updates
    #[clap(long = "no-icon-updates", action = ArgAction::SetFalse)]
    pub update_icons: bool,

    /// Disable system integration
    #[clap(long = "no-system-integration", action = ArgAction::SetFalse)]
    pub system_integration: bool,

    /// Configuration of the HTTP client.
    #[clap(flatten)]
    pub client: HTTPClientConfig,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub enum ProfileCommand {
    /// List available profiles and their web apps
    List(ProfileListCommand),

    /// Create a new profile
    Create(ProfileCreateCommand),

    /// Remove an existing profile
    Remove(ProfileRemoveCommand),

    /// Update an existing profile
    Update(ProfileUpdateCommand),
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct ProfileListCommand {}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct ProfileCreateCommand {
    /// Set a profile name
    #[clap(long)]
    pub name: Option<String>,

    /// Set a profile description
    #[clap(long)]
    pub description: Option<String>,

    /// Set a profile template
    /// {n}All contents of the template directory
    /// will be copied to a newly-created profile
    #[clap(long)]
    pub template: Option<PathBuf>,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct ProfileRemoveCommand {
    /// Profile ID
    pub id: Ulid,

    /// Disable any interactive prompts
    #[clap(short, long)]
    pub quiet: bool,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct ProfileUpdateCommand {
    /// Profile ID
    pub id: Ulid,

    /// Set a profile name
    #[clap(long)]
    pub name: Option<Option<String>>,

    /// Set a profile description
    #[clap(long)]
    pub description: Option<Option<String>>,
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub enum RuntimeCommand {
    /// Install the runtime
    Install(RuntimeInstallCommand),

    /// Uninstall the runtime
    Uninstall(RuntimeUninstallCommand),
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct RuntimeInstallCommand {}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct RuntimeUninstallCommand {}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct HTTPClientConfig {
    /// Import additional root certificates from a DER file
    #[clap(long)]
    pub tls_root_certificates_der: Option<Vec<PathBuf>>,

    /// Import additional root certificates from a PEM file
    #[clap(long)]
    pub tls_root_certificates_pem: Option<Vec<PathBuf>>,

    /// Dangerous: Allow client to client accept invalid certs
    #[clap(long)]
    pub tls_danger_accept_invalid_certs: bool,

    /// Dangerous: Allow client to client accept invalid hostnames
    #[clap(long)]
    pub tls_danger_accept_invalid_hostnames: bool,
}
