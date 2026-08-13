use serde::{Deserialize, Serialize};

use crate::presenter::PerformanceCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguePacket {
    pub speech: String,
    #[serde(default)]
    pub performance: Vec<PerformanceCommand>,
}

impl DialoguePacket {
    pub fn demo() -> Self {
        Self {
            speech: "Twenty-four i is coming to IBC in Amsterdam. Press TEXT for the details."
                .into(),
            performance: vec![
                PerformanceCommand::HeadJerk { x: -20.0, y: 4.0 },
                PerformanceCommand::Freeze { milliseconds: 160 },
                PerformanceCommand::Grin,
            ],
        }
    }
}
