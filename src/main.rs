mod config;
mod fuse;
mod hport;

fn main() {
    let body = async {
        //Edit fuse.rs
        if let Err(_) = fuse::fuse().await {
            std::process::exit(0);
        }

        let config = match config::Config::load("grpml_config.toml".to_string()) {
            Ok(cfg) => cfg,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        hport::run(
            config.input_file,
            config.output_file,
            config.threads,
            config.timeout,
        )
        .await;

        println!("All done.")
    };
    let cpu = num_cpus::get();
    println!("worker thread {}", cpu);
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpu)
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
