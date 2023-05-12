pub fn cmd_run_command_content(line: String) -> String {
    let args_stringed = line.replace("cmd:run_command(", "").replace(")", "");

    let mut result: String = r#"        let process: String = match std::env::consts::FAMILY {
        "unix" => "bash".to_string(),

        "windows" => "cmd".to_string(),
    };

    let _ = std::process::Command::new(process).args("#.to_string();

    result.push_str(&args_stringed);

    let part2 = r#").output().expect("Couldn't execute command");"#;

    result.push_str(part2);

    return result;
}

pub fn clear_cmd() -> String {
    return cmd_run_command_content(r#"cmd:run_command("clear")"#.to_string());
}