use schemars::JsonSchema;
use serde::Deserialize;

use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "olostep-mcp";
const SERVER_PATH: &str = "node_modules/olostep-mcp/build/index.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-olostep";

struct OlostepModelContextExtension;

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct OlostepContextServerSettings {
    /// Your Olostep API key. Get one at https://olostep.com/auth
    #[serde(default)]
    olostep_api_key: Option<String>,
}

impl OlostepModelContextExtension {
    /// Installs `olostep-mcp` from npm on first use. Later starts reuse the
    /// installed copy, so the server starts without a network round trip.
    fn ensure_server_installed(&self) -> Result<()> {
        if zed::npm_package_installed_version(PACKAGE_NAME)?.is_none() {
            let latest = zed::npm_package_latest_version(PACKAGE_NAME)?;
            zed::npm_install_package(PACKAGE_NAME, &latest)?;
        }
        Ok(())
    }
}

impl zed::Extension for OlostepModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let settings: OlostepContextServerSettings = settings
            .settings
            .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
            .transpose()?
            .unwrap_or_default();

        let api_key = settings
            .olostep_api_key
            .filter(|key| !key.trim().is_empty())
            .ok_or_else(|| {
                "Missing `olostep_api_key` in the mcp-server-olostep settings. \
                 Get a key at https://olostep.com/auth"
                    .to_string()
            })?;

        self.ensure_server_installed()?;

        let server_path = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path],
            env: vec![("OLOSTEP_API_KEY".to_string(), api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(OlostepContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(OlostepModelContextExtension);
