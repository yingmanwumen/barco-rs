use std::ffi::CString;

use anyhow::Result;
use log::{debug, error, info};
use nix::errno::errno;
use nix::libc::{execvp, mount, printf, sethostname, MS_PRIVATE, MS_REC};
use nix::sched::{clone, CloneFlags};
use nix::sys::signal::*;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

pub struct Container {
    stack: Vec<u8>,
    pid: Option<Pid>,
}

impl Container {
    pub fn new(stack_size: u64) -> Self {
        info!("Creating Container with options:");
        info!("\tstack_size: {} byte", stack_size);
        Self {
            stack: vec![0; stack_size as usize],
            pid: None,
        }
    }

    fn container_start(&mut self) -> impl FnMut() -> isize {
        || -> isize {
            info!("Container initalizing...");
            let hostname = "container";
            debug!("Invoking syscall: sethostname");
            let err = unsafe { sethostname(hostname.as_ptr() as *const i8, hostname.len()) };
            if err != 0 {
                return errno() as isize;
            }
            debug!("Invoking syscall: mount");
            // let err = unsafe {
            //     mount(
            //         std::ptr::null(),
            //         "/\0".as_ptr() as *const i8,
            //         std::ptr::null(),
            //         MS_REC | MS_PRIVATE,
            //         std::ptr::null(),
            //     )
            // };
            // if err != 0 {
            //     return errno() as isize;
            // }
            info!("Start executing container");
            debug!("Invoking syscall: execvp");
            unsafe {
                execvp(
                    CString::new("/bin/bash").unwrap().into_raw(),
                    std::ptr::null(),
                )
            };
            errno() as isize
        }
    }

    pub fn start(&mut self) -> Result<()> {
        info!("Starting Container...");
        debug!("Invoking syscall: clone");
        let flags = CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWCGROUP
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWNET
            | CloneFlags::CLONE_NEWUTS;
        let pid = unsafe {
            clone(
                Box::new(self.container_start()),
                self.stack.as_mut_slice(),
                flags,
                Some(SIGCHLD as i32),
            )?
        };
        info!("Pid: {}", pid);
        self.pid = Some(pid);
        Ok(())
    }

    pub fn wait(&mut self) -> Result<i8> {
        match self.pid.take() {
            None => {
                log::error!("Container is not started");
                Err(anyhow::anyhow!("Container is not started"))
            }
            Some(pid) => {
                let status = waitpid(pid, None)?;
                match status {
                    WaitStatus::Exited(_, res) => {
                        match res {
                            0 => info!("Container exited successfully"),
                            _ => error!("Container exited with code: {}", res),
                        }
                        Ok(res as i8)
                    }
                    _ => Err(anyhow::anyhow!("Container doesn't exit normally")),
                }
            }
        }
    }
}
