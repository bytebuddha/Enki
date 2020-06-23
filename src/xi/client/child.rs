use log::trace;
use serde_json::{to_string, Value};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::process::{Child, Command};

use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Result as IoResult;
use std::process::Stdio;

use super::ClientImpl;
use crate::xi::Message;

pub struct ChildProcess {
    request_id: usize,
    inner: Child,
}

impl ChildProcess {
    pub fn new(cmd: &str) -> IoResult<ChildProcess> {
        let inner = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("XI_LOG", "trace")
            .spawn()?;
        Ok(ChildProcess {
            request_id: 0,
            inner,
        })
    }
}

#[async_trait::async_trait]
impl ClientImpl for ChildProcess {
    fn next_id(&mut self) -> usize {
        self.request_id += 1;
        self.request_id - 1
    }

    async fn receive(&mut self) -> IoResult<Message> {
        let stdout = self.inner.stdout.as_mut().unwrap();
        let stderr = self.inner.stderr.as_mut().unwrap();
        let mut stdout_reader = BufReader::new(stdout);
        let mut stderr_reader = BufReader::new(stderr);
        let mut stderr_line = String::new();
        let mut stdout_line = String::new();
        tokio::select! {
            Ok(_) = stdout_reader.read_line(&mut stdout_line) => {
                trace!("client < xi-core: {}", stdout_line);
                Ok(serde_json::from_slice::<Message>(stdout_line.as_bytes()).unwrap())
            }
            Ok(_) = stderr_reader.read_line(&mut stderr_line) => {
                trace!("client < xi-core: {}", stderr_line);
                Ok(Message::Error(stderr_line))
            }
        }
    }

    async fn send(&mut self, msg: Value) -> IoResult<()> {
        if let Some(stdin) = &mut self.inner.stdin {
            let data = format!("{}\n", to_string(&msg)?);
            trace!("client > xi-core: {}", data);
            stdin.write_all(data.as_ref()).await?;
            Ok(())
        } else {
            Err(IoError::new(
                ErrorKind::NotConnected,
                "Child Process stdin is not connected",
            ))
        }
    }
}
