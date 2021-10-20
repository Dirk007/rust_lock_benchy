use std::{
    process::exit,
    sync::Arc,
    time::{Duration, Instant},
};

type Inner = String;

#[cfg(feature = "plmutex")]
use parking_lot::Mutex;
#[cfg(feature = "plrwlock")]
use parking_lot::RwLock;
#[cfg(feature = "mutex")]
use std::sync::Mutex;
#[cfg(feature = "rwlock")]
use std::sync::RwLock;
#[cfg(feature = "tmutex")]
use tokio::sync::Mutex;
#[cfg(feature = "trwlock")]
use tokio::sync::RwLock;

#[cfg(feature = "mutex")]
type Sync = Mutex<Inner>;
#[cfg(feature = "rwlock")]
type Sync = RwLock<Inner>;
#[cfg(feature = "tmutex")]
type Sync = Mutex<Inner>;
#[cfg(feature = "trwlock")]
type Sync = RwLock<Inner>;
#[cfg(feature = "plmutex")]
type Sync = Mutex<Inner>;
#[cfg(feature = "plrwlock")]
type Sync = RwLock<Inner>;

type Lock = Arc<Sync>;

#[macro_use]
mod macros;

mod command_line;

const SAMPLES: usize = 10;

async fn reader(_id: usize, thing: Lock, rounds: usize) {
    for _ in 0..rounds {
        let _ = lock_read!(thing);
    }
}

async fn writer(_id: usize, thing: Lock, rounds: usize, modulo: usize) {
    for u in 0..rounds {
        let _ = lock_read!(thing);
        if u % modulo == 0 {
            *lock_write!(thing) = "foobarbaz".into();
        }
    }
}

async fn acquire_one_sample(
    task_count: usize,
    writers: usize,
    rounds: usize,
    write_modulo: usize,
) -> Duration {
    let thing = create_lock!("bazbarfoo".to_string());

    let mut tasks = Vec::with_capacity(task_count);
    for r in 0..task_count - writers {
        tasks.push(tokio::spawn(reader(r, thing.clone(), rounds)));
    }
    for w in 0..writers {
        tasks.push(tokio::task::spawn(writer(
            w,
            thing.clone(),
            rounds,
            write_modulo,
        )));
    }

    let start = Instant::now();
    let _ = futures::future::join_all(&mut tasks).await;

    start.elapsed()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let matches = command_line::parse_arguments();

    if matches.is_present("version") {
        println!(env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let reads = to_usize!(matches, "reads");
    let write_modulo = to_usize!(matches, "writemod");
    let tasks = to_usize!(matches, "tasks");
    let writers = to_usize!(matches, "writers");

    let mut took_ms = 0f64;

    for _ in 0..SAMPLES {
        took_ms += acquire_one_sample(tasks, writers, reads, write_modulo)
            .await
            .as_micros() as f64
            / 1000f64;
    }

    println!(
        "({} samples) took {:.3}ms ({} readers, {} writers, {} reads, {} writes)",
        SAMPLES,
        took_ms / SAMPLES as f64,
        tasks - writers,
        writers,
        reads,
        reads / write_modulo
    );
}
