use hpydev::{cli, go};
use std::convert::TryInto;

async fn run() -> Result<(), anyhow::Error> {
    use structopt::StructOpt;
    let opt = cli::Opt::from_args();

    init_logger(opt.verbose)?;

    match opt.cmd {
        cli::SubCommand::Go(go) => match go.action {
            cli::go::Action::Install(opt) => {
                go::Operator::new().install(opt.try_into()?).await?;
            }
            cli::go::Action::Uninstall(opt) => {
                go::Operator::new().uninstall(opt.try_into()?).await?;
            }
            cli::go::Action::Upgrade(_opt) => {
                tracing::info!("upgrade...");
            }
        },
    }

    Ok(())
}

fn init_logger(verbose: u8) -> Result<(), anyhow::Error> {
    tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(true)
        .with_thread_names(true)
        .with_env_filter(match verbose {
            0 => "hpydev=info",
            1 => "hpydev=debug",
            2 => "hpydev=trace",
            _ => "trace",
        })
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))
}

fn main() {
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(e) = run().await {
                tracing::error!("{}", e);
                std::process::exit(1);
            }
        })
}
