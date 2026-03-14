use crate::ast::{Redirection, RedirectionType};
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use libc;

pub struct RedirectionManager {
    // Stores (original_fd, backup_fd)
    backups: Vec<(RawFd, RawFd)>,
}

impl RedirectionManager {
    pub fn new() -> Self {
        Self { backups: Vec::new() }
    }

    pub fn apply(&mut self, redirs: &[Redirection]) -> Result<(), String> {
        for redir in redirs {
            self.apply_one(redir)?;
        }
        Ok(())
    }

    fn apply_one(&mut self, redir: &Redirection) -> Result<(), String> {
        // Determine the file descriptor to be redirected (e.g., 1 for stdout, 0 for stdin)
        let target_fd = redir.fd.unwrap_or_else(|| match redir.redirection_type {
            RedirectionType::Input | RedirectionType::HereDoc | RedirectionType::ReadWrite | RedirectionType::DupInput => 0,
            _ => 1,
        }) as RawFd;

        // Backup the original FD if it's currently open
        self.backup(target_fd);

        match redir.redirection_type {
            RedirectionType::Input => {
                let f = File::open(&redir.target)
                    .map_err(|e| format!("Failed to open input {}: {}", redir.target, e))?;
                self.dup2(f.as_raw_fd(), target_fd)?;
            }
            RedirectionType::Output | RedirectionType::CLobber => {
                // TODO: CLobber support (force overwrite)
                let f = File::create(&redir.target)
                    .map_err(|e| format!("Failed to create output {}: {}", redir.target, e))?;
                self.dup2(f.as_raw_fd(), target_fd)?;
            }
            RedirectionType::Append => {
                let f = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&redir.target) // Note: append implies write
                    .map_err(|e| format!("Failed to append {}: {}", redir.target, e))?;
                self.dup2(f.as_raw_fd(), target_fd)?;
            }
            RedirectionType::ReadWrite => {
                let f = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&redir.target)
                    .map_err(|e| format!("Failed to open RDWR {}: {}", redir.target, e))?;
                self.dup2(f.as_raw_fd(), target_fd)?;
            }
            RedirectionType::DupInput | RedirectionType::DupOutput => {
                if redir.target == "-" {
                    // Close the target descriptor
                    unsafe { libc::close(target_fd) };
                } else {
                    let source_fd = redir.target.parse::<RawFd>()
                        .map_err(|_| "Invalid file descriptor for duplication".to_string())?;
                    
                    // Verify source_fd is valid
                    if unsafe { libc::fcntl(source_fd, libc::F_GETFD) } < 0 {
                         return Err(format!("Bad file descriptor: {}", source_fd));
                    }
                    self.dup2(source_fd, target_fd)?;
                }
            }
            RedirectionType::HereDoc => {
                // HereDoc implementation usually relies on the parser 
                // passing the content via a pipe or temp file.
                // Assuming we might handle it later or differently.
                 return Err("HereDoc not fully implemented in executor".to_string());
            }
        }
        Ok(())
    }

    fn backup(&mut self, fd: RawFd) {
        unsafe {
             // Check if fd is valid
            if libc::fcntl(fd, libc::F_GETFD) >= 0 {
                let backup = libc::dup(fd);
                if backup >= 0 {
                    // Ensure backup doesn't leak to child processes
                    libc::fcntl(backup, libc::F_SETFD, libc::FD_CLOEXEC);
                    self.backups.push((fd, backup));
                }
            }
        }
    }

    fn dup2(&self, oldfd: RawFd, newfd: RawFd) -> Result<(), String> {
        let ret = unsafe { libc::dup2(oldfd, newfd) };
        if ret < 0 {
            Err(std::io::Error::last_os_error().to_string())
        } else {
            Ok(())
        }
    }
    pub fn keep(mut self) {
        self.backups.clear();
    }
}

impl Drop for RedirectionManager {
    fn drop(&mut self) {
        // Restore FDs in reverse order of backup
        while let Some((original_fd, backup_fd)) = self.backups.pop() {
            unsafe {
                libc::dup2(backup_fd, original_fd);
                libc::close(backup_fd);
            }
        }
    }
}
