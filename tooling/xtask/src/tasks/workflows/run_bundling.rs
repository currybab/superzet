use std::path::Path;

use crate::tasks::workflows::{
    runners::Arch,
    steps::{FluentBuilder, NamedJob, dependant_job, named},
    vars::assets,
};

use super::{runners, steps};
use gh_workflow::*;
use indoc::indoc;

pub fn run_bundling() -> Workflow {
    let bundle_mac_aarch64 = bundle_mac_aarch64();
    named::workflow()
        .on(Event::default().pull_request(
            PullRequest::default().types([PullRequestType::Labeled, PullRequestType::Synchronize]),
        ))
        .concurrency(
            Concurrency::new(Expression::new(
                "${{ github.workflow }}-${{ github.head_ref || github.ref }}",
            ))
            .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", "1"))
        .add_job(bundle_mac_aarch64.name, bundle_mac_aarch64.job)
}

fn bundle_job(deps: &[&NamedJob]) -> Job {
    dependant_job(deps)
        .when(deps.len() == 0, |job|
            job.cond(Expression::new(
                indoc! {
                    r#"(github.event.action == 'labeled' && github.event.label.name == 'run-bundling') ||
                    (github.event.action == 'synchronize' && contains(github.event.pull_request.labels.*.name, 'run-bundling'))"#,
                })))
        .timeout_minutes(60u32)
}

fn bundle_mac_aarch64() -> NamedJob {
    let arch = Arch::AARCH64;
    NamedJob {
        name: format!("bundle_mac_{arch}"),
        job: bundle_job(&[])
            .runs_on(runners::MAC_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_node())
            .add_step(steps::setup_sentry())
            .add_step(steps::clear_target_dir_if_large(runners::Platform::Mac))
            .add_step(named::bash(&format!(
                "./script/bundle-mac {arch}-apple-darwin"
            )))
            .add_step(upload_artifact(&format!(
                "target/{arch}-apple-darwin/release/{}",
                assets::MAC_AARCH64
            ))),
    }
}

pub fn upload_artifact(path: &str) -> Step<Use> {
    let name = Path::new(path).file_name().unwrap().to_str().unwrap();
    Step::new(format!("@actions/upload-artifact {}", name))
        .uses(
            "actions",
            "upload-artifact",
            "330a01c490aca151604b8cf639adc76d48f6c5d4", // v5.0.0
        )
        // N.B. "name" is the name for the asset. The uploaded
        // file retains its filename.
        .add_with(("name", name))
        .add_with(("path", path))
        .add_with(("if-no-files-found", "error"))
}
