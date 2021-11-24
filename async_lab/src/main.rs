use futures::task::Spawn;

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        pin::Pin,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

// our mutex is composed of a shared value which
// indicates whether or not the time has elapsed
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

// shared state between the thread and the future
struct SharedState {
    // Whether or not the sleep time has elapsed
    completed: bool,
    // waker for the task that the future is runnign on
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    // describes what to do when the future is polled
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // get the mutex and unwrap the value in the mutex
        let mut shared_state = self.shared_state.lock().unwrap();
        // test the value protected by the mutex
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // cx.waker().clone() returns a reference to the waker of the task.
            // The thread can use this reference to the waker to check if he
            // needs to poll again the future
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        // create the shared variable in a mutex
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // To simulate an asynchronous operation, spawn a thread and wait
        // for a while in it. This simulates a blocking I/O operation.
        // When the operation is done (the sleep has completed), wake up
        // the future by sending a signal to the async axecutor. In an I/O
        // peripheral, this would be done in an interrupt callback.
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            // once the sleep function has completed, update shared_state value
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            // call wake function to tell the executor the future has completed
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        // return the future
        TimerFuture { shared_state }
    }
}

// create an executor which has to manage a task queue
// all task are send in a channel. The executor can only
// receives from this channel
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

// spawner adds new tasks in the Tasks' queue
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    // BoxFuture contains reference to the future to
    // be pushed to completion
    // Create this future in a Mutex, so futures can be
    // dispatched between several threads
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    // handler to put the task in the tasks' queue when
    // the futures' waker has been called
    task_send: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    // create a tasks' queue in a channel with a fixed size and
    // get sender / receiver structs for this channel
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        // create a new reference to the future
        let future = future.boxed();
        // create a task with the new future reference and
        // an associated task sender
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_send: self.task_sender.clone(),
        });
        // when spawning a new task, push it to the tasks' queue
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// implement the task's waker
impl ArcWake for Task {
    // call this function when the waker is called
    // a reference to this function can be found with
    // waker_ref() method
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        // the waker uses the task.task_send() method
        // to push the task in the queue
        arc_self
            .task_send
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // get the future from the task
            let mut future_slot = task.future.lock().unwrap();
            // extract the future from the task, this will leave None in the task
            if let Some(mut future) = future_slot.take() {
                // find the waker of the task
                let waker = waker_ref(&task);
                // get the context from the task's waker
                let context = &mut Context::from_waker(&*waker);
                // poll the future in the task
                if future.as_mut().poll(context).is_pending() {
                    // the future is not completed yet
                    // push it back in the task's mutex
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    spawner.spawn(async {
        println!("hello");
        TimerFuture::new(Duration::new(1, 0)).await;
        println!("good bye!");
    });

    drop(spawner);

    println!("Let's launch the executor !");
    executor.run();
}
