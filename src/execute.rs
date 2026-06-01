use std::collections::HashMap;

pub fn execute_line(line: &str, variables: &mut HashMap<String, String>, line_num: usize, last_if_result: &mut bool,) {
    let line = line.trim();

    if line.is_empty() {
        return;
    }

    if line.starts_with(":)") {
        return;
    }

    if line.starts_with("cow_sayln! ") {
        let value = line[11..].trim();

        if let Some(var) = variables.get(value) {
            println!("{}", var);
        } else {
            println!("{}", value);
        }
    }

    else if line.starts_with("cow_say! ") {
        let value = line[9..].trim();

        if let Some(var) = variables.get(value) {
            print!("{}", var);
        } else {
            print!("{}", value);
        }
    }

    else if line.starts_with("cook ") {
        let rest = &line[5..];

        if let Some((name, value)) = rest.split_once('=') {
            variables.insert(
                name.trim().to_string(),
                value.trim().to_string(),
            );
        } else {
            println!(
                "Line {}: Invalid variable syntax",
                line_num
            );
        }
    }

    else if line.starts_with("sus ") {
        let condition = line[4..].trim();

        if let Some((left, right)) = condition.split_once("==") {
            let left = left.trim();
            let right = right.trim();

            let left_value = variables.get(left).map(|s| s.as_str())
                .unwrap_or(left);

            *last_if_result = left_value == right;
        } else {
            println!(
                "Line {}: Invalid condition syntax",
                line_num
            );
        }
    }

    else {
        println!(
            "Line {}: Unknown command '{}'",
            line_num,
            line
        );
    }
}