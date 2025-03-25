use std::{fs, path::PathBuf};
use zed_extension_api::{
    self as zed, serde_json, settings::LspSettings, DownloadedFileType, Result,
};

const LS_EXECUTABLE_NAME: &str = "ltex-ls-plus";
const LS_REPO: &str = "ltex-plus/ltex-ls-plus";

struct LTeXPlusExecutable {
    path: PathBuf,
}

struct LTeXPlusExtension {
    executable_cache: Option<PathBuf>,
}

impl LTeXPlusExtension {
    fn new() -> Self {
        Self {
            executable_cache: None,
        }
    }

    fn get_ls_executable(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LTeXPlusExecutable> {
        if let Some(path) = worktree.which(LS_EXECUTABLE_NAME) {
            return Ok(LTeXPlusExecutable {
                path: PathBuf::from(path),
            });
        };

        if let Some(path) = &self.executable_cache {
            return Ok(LTeXPlusExecutable { path: path.clone() });
        };

        self.install_ltex_plus(language_server_id)
    }

    fn install_ltex_plus(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<LTeXPlusExecutable> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            LS_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("Failed to fetch latest release: {}", e))?;

        let version = release.version.clone();

        let (os, arch) = zed::current_platform();

        let (os_str, file_ext) = match os {
            zed::Os::Linux => ("linux", "tar.gz"),
            zed::Os::Mac => ("macos", "tar.gz"),
            zed::Os::Windows => ("windows", "zip"),
        };
        let arch_name = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => return Err("x86 is not supported by this extension.".into()),
        };

        let asset_name = format!("ltex-ls-plus-{version}-{os_str}-{arch_name}.{file_ext}");

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("Failed to find asset with name {}", asset_name))?;

        let version_dir = format!("ltex-ls-plus-{version}");
        let mut binary_path = PathBuf::from(&version_dir).join("bin/ltex-ls-plus");
        if os == zed::Os::Windows {
            binary_path.set_extension("bat");
        }

        if !binary_path.exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_result = (|| -> Result<()> {
                zed::download_file(
                    &asset.download_url,
                    "", // When extracted automatically creates a version directory
                    if os == zed::Os::Windows {
                        zed::DownloadedFileType::Zip
                    } else {
                        DownloadedFileType::GzipTar
                    },
                )
                .map_err(|e| format!("Failed to download LTeX+ Language Server: {}", e))?;

                zed::make_file_executable(binary_path.to_str().ok_or("Invalid binary path")?)
                    .map_err(|e| format!("Failed to make binary executable: {}", e))?;

                Ok(())
            })();

            if let Err(e) = download_result {
                fs::remove_dir_all(&version_dir).ok();
                return Err(e);
            }

            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name != version_dir {
                            fs::remove_dir_all(entry.path()).ok();
                        }
                    }
                }
            }
        }

        self.executable_cache = Some(binary_path.clone());
        Ok(LTeXPlusExecutable { path: binary_path })
    }
}

impl zed::Extension for LTeXPlusExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let executable = self.get_ls_executable(language_server_id, worktree)?;
        Ok(zed::Command {
            command: executable
                .path
                .to_str()
                .ok_or("Invalid binary path")?
                .to_string(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("ltex", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(LTeXPlusExtension);
