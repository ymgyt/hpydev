use hpydev::cli;

async fn run() -> Result<(), anyhow::Error> {
    use structopt::StructOpt;
    let opt = cli::Opt::from_args();

    init_logger(opt.verbose)?;

    tracing::info!("running...");
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
