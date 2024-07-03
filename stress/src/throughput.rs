use core_affinity;
use num_format::{Locale, ToFormattedString};
use std::env;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
#[cfg(feature = "stats")]
use sysinfo::{Pid, System};

const SLIDING_WINDOW_SIZE: u64 = 2; // In seconds
const BATCH_SIZE: u64 = 1000;

static STOP: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Default)]
struct WorkerStats {
    count: AtomicU64,
    // Padding to prevent false sharing
    padding: [u64; 15],
}

pub fn test_throughput<F>(func: F)
where
    F: Fn() + Sync + Send + 'static,
{
    ctrlc::set_handler(move || {
        STOP.store(true, Ordering::Release);
    })
    .expect("Error setting Ctrl-C handler");

    let mut num_threads = num_cpus::get();
    let mut args_iter = env::args();

    if let Some(arg_str) = args_iter.nth(1) {
        if let Ok(arg_num) = arg_str.parse::<usize>() {
            if arg_num > 0 {
                if arg_num > num_cpus::get() {
                    println!(
                        "Specified {} threads which is larger than the number of logical cores ({})!",
                        arg_num, num_threads
                    );
                }
                num_threads = arg_num;
            } else {
                eprintln!(
                    "Invalid number of threads: {}. Must be greater than 0.",
                    arg_num
                );
                std::process::exit(1);
            }
        } else {
            eprintln!(
                "Invalid command line argument '{}'. Must be a positive integer.",
                arg_str
            );
            std::process::exit(1);
        }
    }

    println!("Number of threads: {}\n", num_threads);
    let mut handles = Vec::with_capacity(num_threads + 1); // +1 for the main thread handle
    let func_arc = Arc::new(func);
    let mut worker_stats_vec: Vec<WorkerStats> = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        worker_stats_vec.push(WorkerStats::default());
    }

    let worker_stats_shared = Arc::new(worker_stats_vec);
    let worker_stats_shared_monitor = Arc::clone(&worker_stats_shared);

    let handle_main_thread = thread::spawn(move || {
        let mut start_time = Instant::now();
        let mut total_count_old: u64 = 0;

        #[cfg(feature = "stats")]
        let pid = Pid::from(std::process::id() as usize);
        #[cfg(feature = "stats")]
        let mut system = System::new_all();

        loop {
            if STOP.load(Ordering::Acquire) {
                break;
            }

            let elapsed = start_time.elapsed().as_secs();
            if elapsed >= SLIDING_WINDOW_SIZE {
                let total_count_u64: u64 = worker_stats_shared_monitor
                    .iter()
                    .map(|worker_stat| worker_stat.count.load(Ordering::Relaxed))
                    .sum();
                let current_count = total_count_u64 - total_count_old;
                total_count_old = total_count_u64;
                let throughput = current_count / elapsed;
                println!(
                    "Throughput: {} iterations/sec",
                    throughput.to_formatted_string(&Locale::en)
                );

                #[cfg(feature = "stats")]
                {
                    system.refresh_all();
                    if let Some(process) = system.process(pid) {
                        println!(
                            "Memory usage: {:.2} MB",
                            process.memory() as f64 / (1024.0 * 1024.0)
                        );
                        println!("CPU usage: {}%", process.cpu_usage() / num_threads as f32);
                        println!(
                            "Virtual memory usage: {:.2} MB",
                            process.virtual_memory() as f64 / (1024.0 * 1024.0)
                        );
                    } else {
                        println!("Process not found");
                    }
                }

                println!("\n");
                start_time = Instant::now();
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    handles.push(handle_main_thread);

    let core_ids = core_affinity::get_core_ids().unwrap();
    for thread_index in 0..num_threads {
        let worker_stats_for_thread = Arc::clone(&worker_stats_shared);
        let func_arc_clone = Arc::clone(&func_arc);
        let core_id = core_ids[thread_index % core_ids.len()];
        let handle = thread::spawn(move || {
            core_affinity::set_for_current(core_id);
            loop {
                for _ in 0..BATCH_SIZE {
                    func_arc_clone();
                }
                worker_stats_for_thread[thread_index]
                    .count
                    .fetch_add(BATCH_SIZE, Ordering::Relaxed);
                if STOP.load(Ordering::Acquire) {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
