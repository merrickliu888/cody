use std::process::{Command, Stdio};

pub struct ChildGuard {
    child: std::process::Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

pub fn init_local_llm() -> (ChildGuard, ChildGuard) {
    let ollama_server = Command::new("ollama")
        .arg("serve")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to initialize Cody: ollama serve");
    let ollama_llm = Command::new("ollama")
        .args(["run", "cody"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to initialize Cody: ollama run llama3.1");
    return (ChildGuard { child: ollama_server }, ChildGuard { child: ollama_llm });
}
