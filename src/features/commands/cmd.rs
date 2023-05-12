use crate::features::push_stringed_vector::push_stringed_vector;

pub fn cmd_run_command_content(line: String) -> String {
    let args_stringed = line.replace("cmd:run_command(", "").replace(")", "");

    let mut result: String = match &args_stringed {
        x if x.split_whitespace().count() >= 2 => {
            r#"        let process: String = match std::env::consts::FAMILY {
        "unix" => "bash".to_string(),

        "windows" => "cmd".to_string(),

        &_ => todo!()
    };

    let _ = std::process::Command::new(process).args("#
                .to_string()
        }

        _ => r#"        let process: String = match std::env::consts::FAMILY {
            "unix" => "bash".to_string(),
    
            "windows" => "cmd".to_string(),

            &_ => todo!()
        };
    
        let _ = std::process::Command::new(process).arg("#
            .to_string(),
    };

    match args_stringed.split_whitespace().count() {
        x if x >= 2 => {
            result.push_str(push_stringed_vector(args_stringed, " ").as_str());
        }

        _ => {
            result.push_str(&args_stringed);
        }
    };

    let part2 = r#").output().expect("Couldn't execute command");"#;

    result.push_str(part2);

    return result;
}

pub fn clear_cmd() -> String {
    return cmd_run_command_content(r#"cmd:run_command("clear")"#.to_string());
}

