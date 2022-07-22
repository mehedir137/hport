use futures::{stream, StreamExt};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::net::Ipv4Addr;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::time::Duration;

pub async fn run(input: String, results_path: String, threads: usize, timeout: u64) {
    let file = std::fs::File::open(input).expect("failed to open sites.txt");
    let filebuf = BufReader::new(file);

    for line in filebuf.lines() {
        let _ = check_ports(line.unwrap(), results_path.clone(), threads, timeout).await;
    }
}

async fn check_ports(
    cidr: String,
    results_path: String,
    threads: usize,
    timeout: u64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut cidr = cidr.split('/');
    let ip = cidr.next().ok_or("")?;
    let range = cidr.next().ok_or("")?;

    let cidr = cidr::Ipv4Cidr::new(ip.parse()?, range.parse()?)?;

    let to = Duration::from_secs(timeout);
    let mut stream = stream::iter(cidr)
        .map(|ip| tokio::spawn(check_port(ip.address(), to)))
        .buffer_unordered(threads);

    let mut result_f = open_file(results_path);
    while let Some(result) = stream.next().await {
        if let Ok(Ok(ip)) = result {
            println!("{}", ip);
            result_f.write_all(ip.to_string().as_bytes()).unwrap();
            result_f.write("\n".as_bytes()).unwrap();
            result_f.flush().unwrap();
        }
    }
    Ok(())
}

async fn check_port(ip: Ipv4Addr, to: Duration) -> Result<Ipv4Addr, Box<dyn Error + Send + Sync>> {
    println!("checking {}...", ip);
    timeout(to, TcpStream::connect((ip, 80))).await??;
    Ok(ip)
}

fn open_file(filename: String) -> LineWriter<File> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    LineWriter::new(file)
}
