use cmd_lib::run_cmd;
use std::env;
use std::path::PathBuf;

struct SparseCheckout {
    remote: String,
    branch: String,
    repo_path: String,
    checkout_to: PathBuf,
}

impl SparseCheckout {
    pub fn new(branch: impl Into<String>) -> Self {
        SparseCheckout {
            remote: "https://github.com/vegaprotocol/vega.git".into(),
            branch: branch.into(),
            repo_path: "protos/sources".into(),
            checkout_to: "external/vega".into(),
        }
    }

    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        let initial_dir = std::env::current_dir().unwrap();
        let repo_dir = initial_dir.join(self.checkout_to);

        if !repo_dir.exists() {
            std::fs::create_dir_all(repo_dir.clone())?;
        }
        std::env::set_current_dir(repo_dir.as_path())?;

        let remote = &self.remote;
        let repo_path = &self.repo_path;
        if !repo_dir.join(".git").exists() {
            run_cmd! {
                git init;
                git sparse-checkout set --sparse-index;
                git sparse-checkout add $repo_path;
                git remote add origin $remote;
            }?;
        }

        let branch = &self.branch;
        run_cmd! {
            git pull origin $branch --depth 1;
        }?;

        std::env::set_current_dir(initial_dir)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SparseCheckout::new("develop").exec()?;
    // prost_build::compile_protos(
    //     &["external/vega/protos/sources/vega/events/v1/events.proto"],
    //     &["external/vega/protos/sources/"],
    // )?;

    let out_dir = env::var("OUT_DIR").unwrap();
    run_cmd! {
        mkdir -p generated;
        ln -sf $out_dir generated/protos;
    }?;

    Ok(())
}
