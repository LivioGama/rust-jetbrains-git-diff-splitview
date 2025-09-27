#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationAction {
    NextDiffBlock,
    PreviousDiffBlock,
    NextConnector,
    PreviousConnector,
    ApplyHunk,
    RevertHunk,
    StageHunk,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct NavigationState {
    pub current_block_index: usize,
    pub total_blocks: usize,
    pub current_connector_index: usize,
    pub total_connectors: usize,
}

impl NavigationState {
    #[allow(dead_code)]
    pub fn update(&mut self, total_blocks: usize, total_connectors: usize) {
        self.total_blocks = total_blocks;
        self.total_connectors = total_connectors;
        if self.current_block_index >= self.total_blocks && self.total_blocks > 0 {
            self.current_block_index = self.total_blocks - 1;
        }
        if self.current_connector_index >= self.total_connectors && self.total_connectors > 0 {
            self.current_connector_index = self.total_connectors - 1;
        }
    }

    pub fn next_block(&mut self) -> Option<usize> {
        if self.total_blocks == 0 {
            return None;
        }
        self.current_block_index = (self.current_block_index + 1) % self.total_blocks;
        Some(self.current_block_index)
    }

    pub fn previous_block(&mut self) -> Option<usize> {
        if self.total_blocks == 0 {
            return None;
        }
        if self.current_block_index == 0 {
            self.current_block_index = self.total_blocks - 1;
        } else {
            self.current_block_index -= 1;
        }
        Some(self.current_block_index)
    }

    pub fn next_connector(&mut self) -> Option<usize> {
        if self.total_connectors == 0 {
            return None;
        }
        self.current_connector_index = (self.current_connector_index + 1) % self.total_connectors;
        Some(self.current_connector_index)
    }

    pub fn previous_connector(&mut self) -> Option<usize> {
        if self.total_connectors == 0 {
            return None;
        }
        if self.current_connector_index == 0 {
            self.current_connector_index = self.total_connectors - 1;
        } else {
            self.current_connector_index -= 1;
        }
        Some(self.current_connector_index)
    }
}
