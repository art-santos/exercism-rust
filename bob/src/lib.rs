pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message {
        msg if msg.is_empty() => "Fine. Be that way!",
        msg if !msg.chars().any(|c| c.is_alphabetic()) && !msg.ends_with("?") => "Whatever.",
        msg if msg.chars().all(|c| c.is_numeric()) => "Whatever.",
        msg if msg.ends_with("?") && msg.to_lowercase() == msg => "Sure.",
        msg if msg.ends_with("?") && msg.to_uppercase() != msg => "Sure.",
        msg if !msg.ends_with("?") && msg.to_uppercase() == msg => "Whoa, chill out!",
        msg if msg.ends_with("?") && msg.to_uppercase() == msg => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
