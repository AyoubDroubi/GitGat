use anyhow::{Context, Result, bail};
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output, Stdio};

#[derive(Debug, Clone, Default)]
pub struct ProcessRunner;

impl ProcessRunner {
    pub fn run<I, S>(&self, program: &str, args: I, cwd: Option<&Path>) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(program);
        command.args(args);
        if let Some(path) = cwd {
            command.current_dir(path);
        }
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let output = command
            .output()
            .with_context(|| format!("failed to start {program}"))?;
        self.output_to_string(program, output)
    }

    pub fn run_allow_failure<I, S>(
        &self,
        program: &str,
        args: I,
        cwd: Option<&Path>,
    ) -> Result<(bool, String)>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(program);
        command.args(args);
        if let Some(path) = cwd {
            command.current_dir(path);
        }
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let output = command
            .output()
            .with_context(|| format!("failed to start {program}"))?;
        let success = output.status.success();
        let mut text = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&stderr);
        }
        Ok((success, sanitize(&text)))
    }

    fn output_to_string(&self, program: &str, output: Output) -> Result<String> {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = sanitize(&String::from_utf8_lossy(&output.stderr));
        if !output.status.success() {
            bail!("{program} exited with {}: {}", output.status, stderr.trim());
        }
        Ok(stdout)
    }
}

pub fn sanitize(input: &str) -> String {
    let mut out = input.to_owned();
    for prefix in ["ghp_", "github_pat_"] {
        while let Some(start) = out.find(prefix) {
            let end = out[start..]
                .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ',' | ')' | ']'))
                .map(|offset| start + offset)
                .unwrap_or(out.len());
            out.replace_range(start..end, "[REDACTED]");
        }
    }

    let mut cursor = 0;
    while let Some(relative) = out[cursor..].find("https://") {
        let start = cursor + relative + "https://".len();
        let rest = &out[start..];
        let Some(at) = rest.find('@') else {
            break;
        };
        let credential = &rest[..at];
        if credential.contains(':') && !credential.contains('/') {
            out.replace_range(start..start + at, "[REDACTED]");
            cursor = start + "[REDACTED]@".len();
        } else {
            cursor = start + at + 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn redacts_github_tokens_and_url_credentials() {
        let value = "ghp_secret https://user:pass@example.com github_pat_abc";
        let clean = sanitize(value);
        assert!(!clean.contains("ghp_secret"));
        assert!(!clean.contains("user:pass"));
        assert!(!clean.contains("github_pat_abc"));
        assert!(clean.contains("[REDACTED]"));
    }
}
