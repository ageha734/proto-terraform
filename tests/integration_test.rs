use proto_pdk_test_utils::*;

mod terraform_tool {
    use super::*;

    generate_resolve_versions_tests!("terraform-test", {
        "1.9" => "1.9.8",
        "1.5.0" => "1.5.0",
    });

    #[tokio::test(flavor = "multi_thread")]
    async fn registers_tool_metadata() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin.register_tool(RegisterToolInput::default()).await;

        assert_eq!(output.name, "Terraform");
        assert_eq!(output.type_of, PluginType::CommandLine);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn loads_versions_from_hashicorp() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sets_latest_alias() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(output.latest.is_some());
        assert!(output.aliases.contains_key("latest"));
        assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn detects_version_files() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin.detect_version_files().await;

        assert_eq!(output.files, vec![".terraform-version".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parses_terraform_version_file() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin
            .parse_version_file(ParseVersionFileInput {
                content: "1.5.7\n".into(),
                file: ".terraform-version".into(),
                ..Default::default()
            })
            .await;

        assert_eq!(
            output.version.unwrap(),
            UnresolvedVersionSpec::parse("1.5.7").unwrap()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn returns_none_for_empty_version_file() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("terraform-test").await;

        let output = plugin
            .parse_version_file(ParseVersionFileInput {
                content: "".into(),
                file: ".terraform-version".into(),
                ..Default::default()
            })
            .await;

        assert_eq!(output.version, None);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_linux_amd64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::Linux, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_linux_amd64.zip"
                        .into(),
                download_name: Some("terraform_1.5.7_linux_amd64.zip".into()),
                checksum_url: Some(
                    "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_SHA256SUMS"
                        .into()
                ),
                checksum_public_key: Some(
                    "https://www.hashicorp.com/.well-known/pgp-key.txt".into()
                ),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_linux_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_linux_arm64.zip"
                        .into(),
                download_name: Some("terraform_1.5.7_linux_arm64.zip".into()),
                checksum_url: Some(
                    "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_SHA256SUMS"
                        .into()
                ),
                checksum_public_key: Some(
                    "https://www.hashicorp.com/.well-known/pgp-key.txt".into()
                ),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url: "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_darwin_arm64.zip".into(),
                download_name: Some("terraform_1.5.7_darwin_arm64.zip".into()),
                checksum_url: Some("https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_SHA256SUMS".into()),
                checksum_public_key: Some("https://www.hashicorp.com/.well-known/pgp-key.txt".into()),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::MacOS, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url: "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_darwin_amd64.zip".into(),
                download_name: Some("terraform_1.5.7_darwin_amd64.zip".into()),
                checksum_url: Some("https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_SHA256SUMS".into()),
                checksum_public_key: Some("https://www.hashicorp.com/.well-known/pgp-key.txt".into()),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_windows_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url: "https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_windows_amd64.zip".into(),
                download_name: Some("terraform_1.5.7_windows_amd64.zip".into()),
                checksum_url: Some("https://releases.hashicorp.com/terraform/1.5.7/terraform_1.5.7_SHA256SUMS".into()),
                checksum_public_key: Some("https://www.hashicorp.com/.well-known/pgp-key.txt".into()),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_unix_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::Linux, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await
                .exes
                .get("terraform")
                .unwrap()
                .exe_path,
            Some("terraform".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("terraform-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: PluginContext {
                        version: VersionSpec::parse("1.5.7").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await
                .exes
                .get("terraform")
                .unwrap()
                .exe_path,
            Some("terraform.exe".into())
        );
    }
}
