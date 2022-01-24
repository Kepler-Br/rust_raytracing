use std::sync::{mpsc, Arc, Mutex};

use crate::misc::scene::Scene;
use crate::scene::scene_info::SceneInfo;
use crate::thread_pool::trace_result::TraceResult;

use super::job::Message;
use super::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    command_sender: mpsc::Sender<Message>,
    job_sender: mpsc::Sender<SceneInfo>,
    result_receiver: mpsc::Receiver<TraceResult>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel();
        let (command_sender, command_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let command_receiver = Arc::new(Mutex::new(command_receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&job_receiver),
                Arc::clone(&command_receiver),
                result_sender.clone(),
            ));
        }

        return ThreadPool {
            workers,
            command_sender,
            job_sender,
            result_receiver,
        };
    }
}

impl ThreadPool {
    pub fn try_receive(&mut self) -> Option<TraceResult> {
        return if let Ok(result) = self.result_receiver.try_recv() {
            Option::Some(result)
        } else {
            Option::None
        };
    }

    pub fn execute_scene(&self, scene_info: SceneInfo, count: usize) {
        if count == 0 {
            return;
        }

        for _ in 0..(count - 1) {
            self.job_sender.send(scene_info.clone()).unwrap();
        }

        self.job_sender.send(scene_info).unwrap();
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Fn() -> Scene + Send + 'static,
    {
        // let job = Box::new(f);

        // self.sender.send(Message::RenderJob(job)).unwrap();
    }

    pub fn execute_multiple<F>(&self, f: F, times: usize)
    where
        F: Fn() -> Scene + Send + Clone + 'static,
    {
        // if times == 0 {
        //     return;
        // }
        //
        // for _ in 0..(times-1) {
        //     let job = Box::new(f.clone());
        //     self.sender.send(Message::RenderJob(job)).unwrap();
        //
        // }
        //
        // let job = Box::new(f);
        // self.sender.send(Message::RenderJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down ThreadPool");

        for worker in &self.workers {
            if worker.thread.is_some() {
                self.command_sender.send(Message::Terminate).unwrap();
            }
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
