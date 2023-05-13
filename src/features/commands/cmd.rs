use crate::features::push_stringed_vector::push_stringed_vector;

pub fn cmd_run_command_content(line: String) -> String {
    let args_stringed = line.replace("cmd:run_command(", "").replace(")", "");

    let mut result: String = r#"        let process: String = match std::env::consts::FAMILY {
            "unix" => "bash".to_string(),
    
            "windows" => "cmd".to_string(),

            &_ => todo!()
        };

        let mut output: std::process::Output;
    
        match std::env::consts::FAMILY { "#
        .to_string();

    let part2 = r#").output().expect("Couldn't execute command");"#;

    match args_stringed.split_whitespace().count() {
        x if x >= 2 => {
            result.push_str(
                r#""unix" => {output = std::process::Command::new(process).arg("-c").args("#,
            );
            result.push_str(push_stringed_vector(&args_stringed, " ").as_str());
            result.push_str(r#"}"#);
            result.push_str(&part2);
            result.push_str(
                r#""windows" => {output = std::process::Command::new(process).arg("/C").args("#,
            );
            result.push_str(push_stringed_vector(&args_stringed, " ").as_str());
            result.push_str(r#"}"#);
            result.push_str(&part2.replace(",", ""));
        }

        _ => {
            result.push_str(
                r#"            "unix" => {output = std::process::Command::new(process).arg("-c").arg(
                "#,
            );
            result.push_str(&args_stringed);
            result.push_str(&part2);
            result.push_str(r#"}"#);
            result.push_str(
                r#"    
            "windows" => {output = std::process::Command::new(process).arg("/C").arg("#,
            );
            result.push_str(&args_stringed);
            result.push_str(&part2.replace(",", ""));
        }
    };

    result.push_str(r#"}"#);
    result.push_str("&_ => todo!()");
    result.push_str(r#"};"#);
    result.push_str(r#"println!("{}", String::from_utf8_lossy(&output.stdout));"#);
    result.push_str(r#"println!("{}", String::from_utf8_lossy(&output.stderr));"#);

    return result;
}
