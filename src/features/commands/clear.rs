use super::cmd::cmd_run_command_content;

pub fn clear_cmd() -> String {
    return cmd_run_command_content(r#"cmd:run_command("clear")"#.to_string());
}
