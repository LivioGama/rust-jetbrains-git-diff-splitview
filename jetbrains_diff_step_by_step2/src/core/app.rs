use eframe::egui;
use crate::core::data_loader::{DataLoader, DiffData, DataLoadError};
use crate::state::StateManager;
use crate::actions::ActionHandler;

/// Core application structure that coordinates all components
pub struct DiffViewerCore {
    data_loader: DataLoader,
    state_manager: StateManager,
    action_handler: ActionHandler,
    diff_data: Option<DiffData>,
    loading_error: Option<String>,
}

impl DiffViewerCore {
    /// Create a new core application with default configuration
    pub fn new() -> Self {
        let data_loader = DataLoader::with_default_config();
        let state_manager = StateManager::new();
        let action_handler = ActionHandler::new(data_loader.git_ops().clone());

        Self {
            data_loader,
            state_manager,
            action_handler,
            diff_data: None,
            loading_error: None,
        }
    }

    /// Create a new core application with custom configuration
    pub fn with_config(config: crate::core::data_loader::DataLoaderConfig) -> Self {
        let data_loader = DataLoader::new(config);
        let state_manager = StateManager::new();
        let action_handler = ActionHandler::new(data_loader.git_ops().clone());

        Self {
            data_loader,
            state_manager,
            action_handler,
            diff_data: None,
            loading_error: None,
        }
    }

    /// Load the diff data
    pub fn load_data(&mut self) -> Result<(), String> {
        match self.data_loader.load_data() {
            Ok(data) => {
                self.diff_data = Some(data.clone());
                
                // Update state manager with loaded data
                self.state_manager.update_state(|state| {
                    state.current_file = data.file_path;
                    state.left_lines = data.old_lines;
                    state.right_lines = data.new_lines;
                    state.change_blocks = data.change_blocks;
                    state.anchors = data.anchors;
                    state.mapping_segments = data.mapping_segments;
                });

                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to load data: {:?}", e);
                self.loading_error = Some(error_msg.clone());
                Err(error_msg)
            }
        }
    }

    /// Get the current diff data
    pub fn diff_data(&self) -> Option<&DiffData> {
        self.diff_data.as_ref()
    }

    /// Get the loading error if any
    pub fn loading_error(&self) -> Option<&String> {
        self.loading_error.as_ref()
    }

    /// Get references to the main components
    pub fn components(&self) -> (&StateManager, &ActionHandler) {
        (&self.state_manager, &self.action_handler)
    }

    /// Get mutable references to the main components
    pub fn components_mut(&mut self) -> (&mut StateManager, &mut ActionHandler) {
        (&mut self.state_manager, &mut self.action_handler)
    }

    /// Create the eframe app
    pub fn create_app(self) -> Result<Box<dyn eframe::App>, String> {
        if self.diff_data.is_none() && self.loading_error.is_none() {
            return Err("No data loaded and no error - call load_data() first".to_string());
        }

        Ok(Box::new(DiffViewerApp::new(self.state_manager, self.action_handler)))
    }
}

// Wrapper around the actual app to maintain compatibility
pub struct DiffViewerApp {
    state_manager: StateManager,
    action_handler: ActionHandler,
}

impl DiffViewerApp {
    pub fn new(state_manager: StateManager, action_handler: ActionHandler) -> Self {
        Self {
            state_manager,
            action_handler,
        }
    }
}

impl eframe::App for DiffViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // This would delegate to the existing app implementation
        // For now, we'll keep the existing logic
        use crate::app::DiffViewerApp as OriginalApp;
        
        let mut original_app = OriginalApp::new(
            self.state_manager.clone(),
            self.action_handler.clone(),
        );
        
        original_app.update(ctx, _frame);
    }
}
