pub mod testcfg;

use anyhow::Result;
use fuelup::{fmt::format_toolchain_with_target, target_triple::TargetTriple};
use testcfg::{FuelupState, CUSTOM_TOOLCHAIN_NAME, DATE};

#[test]
fn fuelup_default_empty() -> Result<()> {
    testcfg::setup(FuelupState::Empty, &|cfg| {
        let output = cfg.fuelup(&["default"]);
        let expected_stdout =
            "No default toolchain detected. Please install or create a toolchain first.\n";

        assert_eq!(output.stdout, expected_stdout);
        assert!(!cfg.home.join("settings.toml").exists());
    })?;
    Ok(())
}

#[test]
fn fuelup_default() -> Result<()> {
    let latest = format_toolchain_with_target("latest");
    testcfg::setup(FuelupState::LatestToolchainInstalled, &|cfg| {
        let output = cfg.fuelup(&["default"]);
        let expected_stdout = format!("{latest} (default)\n");
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_latest_and_custom() -> Result<()> {
    testcfg::setup(FuelupState::LatestAndCustomInstalled, &|cfg| {
        let output = cfg.fuelup(&["default", "latest"]);
        let expected_stdout = format!(
            "Default toolchain set to 'latest-{}'\n",
            TargetTriple::from_host().unwrap()
        );
        assert_eq!(output.stdout, expected_stdout);

        let output = cfg.fuelup(&["default", CUSTOM_TOOLCHAIN_NAME]);
        let expected_stdout = format!("Default toolchain set to '{CUSTOM_TOOLCHAIN_NAME}'\n");
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_uninstalled_toolchain() -> Result<()> {
    testcfg::setup(FuelupState::LatestToolchainInstalled, &|cfg| {
        let triple = TargetTriple::from_host().unwrap();
        let output = cfg.fuelup(&["default", "nightly"]);
        let expected_stdout = format!(
            "Toolchain with name 'nightly-{}' does not exist\n\n\
            \u{1b}[1mInstalled toolchains\u{1b}[0m\n\
            --------------------\n\
            latest-{}\n\n",
            triple, triple
        );
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_nightly() -> Result<()> {
    testcfg::setup(FuelupState::AllInstalled, &|cfg| {
        let output = cfg.fuelup(&["default", "nightly"]);
        let expected_stdout = format!(
            "Default toolchain set to 'nightly-{}'\n",
            TargetTriple::from_host().unwrap()
        );
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_nightly_and_nightly_date() -> Result<()> {
    testcfg::setup(FuelupState::NightlyAndNightlyDateInstalled, &|cfg| {
        let stripped = strip_ansi_escapes::strip(cfg.fuelup(&["default", "nightly"]).stdout);
        let stdout = String::from_utf8_lossy(&stripped);
        let expected_stdout = format!(
            "Default toolchain set to 'nightly-{}'\n",
            TargetTriple::from_host().unwrap()
        );
        assert_eq!(stdout, expected_stdout);

        let stripped =
            strip_ansi_escapes::strip(cfg.fuelup(&["default", &format!("nightly-{DATE}")]).stdout);
        let stdout = String::from_utf8_lossy(&stripped);
        let expected_stdout = format!(
            "Default toolchain set to 'nightly-{}-{}'\n",
            DATE,
            TargetTriple::from_host().unwrap()
        );
        assert_eq!(stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_override() -> Result<()> {
    testcfg::setup(FuelupState::LatestWithTestnetOverride, &|cfg| {
        let output = cfg.fuelup(&["default"]);
        let triple = TargetTriple::from_host().unwrap();
        let expected_stdout = format!("testnet-{triple} (override), latest-{triple} (default)\n");
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}

#[test]
fn fuelup_default_missing_toolchain() -> Result<()> {
    testcfg::setup(FuelupState::AllInstalled, &|cfg| {
        let triple = TargetTriple::from_host().unwrap();
        let output = cfg.fuelup(&["default", "abcd"]);
        let expected_stdout = format!(
            "Toolchain with name 'abcd' does not exist\n\n\
            \u{1b}[1mInstalled toolchains\u{1b}[0m\n\
            --------------------\n\
            latest-{triple}\n\
            nightly-{triple}\n\
            nightly-2022-08-30-{triple}\n\n"
        );

        println!("\n{}", output.stdout);
        assert_eq!(output.stdout, expected_stdout);
    })?;
    Ok(())
}
