use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use nalgebra_glm::IVec2;
use rand::{Rng, thread_rng};

use crate::misc::tracer::Tracer;
use crate::scene::scene_info::SceneInfo;
use crate::thread_pool::trace_result::TraceResult;

use super::job::Message;

pub(super) struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub(super) fn new(
        id: usize,
        job_receiver: Arc<Mutex<mpsc::Receiver<SceneInfo>>>,
        command_receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        sender: mpsc::Sender<TraceResult>,
    ) -> Worker {
        let thread = thread::spawn(move || 'stop: loop {
            let job_option = job_receiver.lock().unwrap().try_recv();

            if let Ok(scene_info) = job_option {
                println!("Worker {} got a job; executing.", id);
                let mut tracer = Tracer::new(IVec2::new(800, 600), scene_info.build());
                let mut iterations: usize = thread_rng().gen_range(1..20);

                loop {
                    tracer.trace();

                    iterations += 1;

                    if iterations % 20 == 0 {
                        let result = tracer.construct_trace_result();

                        sender.send(result).unwrap();
                    }

                    if let Ok(message) = command_receiver.lock().unwrap().try_recv() {
                        match message {
                            Message::Terminate => break 'stop,
                        }
                    }
                }
            }
        });

        return Worker {
            id,
            thread: Some(thread),
        };
    }
}
