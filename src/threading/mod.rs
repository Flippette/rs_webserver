use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(threads: usize) -> Result<ThreadPool, PoolCreationError> {
        if threads == 0 {
            return Err(PoolCreationError);
        }

        println!("Creating ThreadPool with {} thread(s)...", threads);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(threads);

        for i in 0..threads {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let message = Message::NewJob(Box::new(f));
        self.sender.send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination signal to all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Worker {} is shutting down...", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        println!("Creating worker with id {}", id);
        Worker {
            id,
            handle: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job, executing...", id);
                        job();
                    }
                    Message::Terminate => {
                        println!(
                            "Worker {} received a termination command, terminating...",
                            id
                        );
                        break;
                    }
                }
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to create ThreadPool instance!")
    }
}
