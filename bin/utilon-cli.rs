use clap::{Parser, Subcommand};
use uuid::Uuid;
use serde_json::json;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Submit { payload: String, reward: u64 },
    Vote { job: String, worker: String, hash: String },
    Finalize { job: String },
    Chain,
    Balance,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.cmd {
        Cmd::Submit { payload, reward } => {
            let res = client.post("http://127.0.0.1:9933/job")
                .json(&json!({"id": Uuid::nil(), "payload": payload, "reward": reward, "finalized": false}))
                .send().await.unwrap();
            println!("{}", res.text().await.unwrap());
        }
        Cmd::Vote { job, worker, hash } => {
            client.post("http://127.0.0.1:9933/vote")
                .json(&json!({"job_id": job, "worker": worker, "result_hash": hash}))
                .send().await.unwrap();
        }
        Cmd::Finalize { job } => {
            client.post("http://127.0.0.1:9933/finalize")
                .json(&job)
                .send().await.unwrap();
        }
        Cmd::Chain => {
            let r = client.get("http://127.0.0.1:9933/chain").send().await.unwrap();
            println!("{}", r.text().await.unwrap());
        }
        Cmd::Balance => {
            let r = client.get("http://127.0.0.1:9933/balance").send().await.unwrap();
            println!("{}", r.text().await.unwrap());
        }
    }
}
