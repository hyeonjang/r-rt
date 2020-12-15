use std::thread::*;

struct RenderContext {

}

struct RenderTask {
    scanline:u16,
    max_depth:u16,
    samples_per_pixel:u32,
}

struct RenderResult {
  
}

struct RenderThread {
    handle:JoinHandle<()>,

    ctx:RenderContext, 
    tasks:Vec<RenderTask>,
    results:Vec<RenderResult>,
}

impl RenderThread {
    pub fn push_task(&mut self, task:RenderTask) {
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        self.handle = std::thread::spawn(move || {
        })
    }

    pub fn render_loop() {

    }

    pub fn join(self) {
        self.handle.join();
    }
}

struct RenderThreadPool {
    context:RenderContext,
    threads:Vec<RenderThread>,
}

impl RenderThreadPool {
    pub fn start(&self) {
        for thread in &self.threads {

        }
    }
}