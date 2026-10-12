use std::sync::{mpsc::{self, Receiver, Sender},Arc, Mutex,};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Job(Job),
    Shutdown,
}

pub struct Worker {
    sender: Sender<Message>,
    threads: Vec<JoinHandle<()>>,
}

impl Worker {
    pub fn new(thread_count: usize) -> Self {
        assert!(thread_count > 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(thread_count);

        for _ in 0..thread_count {
            let receiver = Arc::clone(&receiver);
            let thread = thread::spawn(move || {
                loop {
                    let message = {
                        let receiver = receiver.lock().unwrap();
                        receiver.recv()
                    };
                    match message {
                        Ok(Message::Job(job)) => {
                            job();
                        }
                        Ok(Message::Shutdown) | Err(_) => {
                            break;
                        }
                    }
                }
            });
            threads.push(thread);
        }
        Self {sender, threads}
    }

    pub fn execute<F, R>(&self, job: F) -> Receiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (result_sender, result_receiver) = mpsc::channel();
        let job = Box::new(move || {let result = job(); let _ = result_sender.send(result);});

        self.sender
            .send(Message::Job(job))
            .expect("Failed to execute: worker threads already stopped");

        result_receiver
    }
}

impl Drop for Worker {

    fn drop(&mut self) {
        for _ in &self.threads {
            let _ = self.sender.send(Message::Shutdown);
        }

        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}