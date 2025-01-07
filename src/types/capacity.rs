#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Capacity {
    Low,
    Medium,
    High
}

impl Capacity {
    pub fn as_value(&self) -> &'static str {
        match self {
            &Capacity::Low => "-Xmx4096M",
            &Capacity::Medium => "-Xmx6144M",
            &Capacity::High => "-Xmx8192M"
        }
    }
}
