use serde::{Deserialize, Serialize};

use crate::presenter::PerformanceCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DialoguePacket {
    pub speech: String,
    #[serde(default)]
    pub performance: Vec<PerformanceCommand>,
}
