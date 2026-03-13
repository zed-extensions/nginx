use zed_extension_api::{self as zed, LanguageServerId, Result};

struct NginxExtension {}

impl zed::Extension for NginxExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("nginx-language-server").ok_or_else(|| {
            "Must install https://github.com/pappasam/nginx-language-server".to_string()
        })?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(NginxExtension);
