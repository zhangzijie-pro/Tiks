
#[derive(Debug, PartialEq, PartialOrd, Eq,Clone)]
pub enum CommandPriority {
    Low,
    Medium,
    High,
}

impl CommandPriority{
    fn as_number(&self) -> u8 {
        match *self {
            CommandPriority::Low => 0,
            CommandPriority::Medium => 1,
            CommandPriority::High => 2,
        }
    }
}

impl Ord for CommandPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_number().cmp(&other.as_number())
    }
}

// Set command priority
pub fn get_priority(command: &str) -> CommandPriority{
    match command{
        "pwd"|"ls"|"mkdir"|"touch"|"whoami"|"exit" => CommandPriority::Low,
        "cd"|"rm"|"cat"|"python"|"html"|"web"|"rn"|"mv"|"tar"|"grep"|"pd"|"root"|"apt"|"history" => CommandPriority::Medium,
        "sleep"|"kill"|"ps"=> CommandPriority::High,
        _ => CommandPriority::Low
    }
}