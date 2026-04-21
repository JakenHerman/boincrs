/// BOINC project summary shown in the Projects pane.
#[derive(Debug, Clone, Default)]
pub struct Project {
    /// Project master URL.
    pub url: String,
    /// Human-readable project name.
    pub name: String,
    /// Whether suspended via GUI RPC command.
    pub suspended_via_gui: bool,
    /// Whether project is set to no-more-work mode.
    pub dont_request_more_work: bool,
}

/// Normalized task status used by UI grouping and coloring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskStatus {
    /// Task currently executing.
    Running,
    /// Task is queued/preempted and waiting to run.
    WaitingToRun,
    /// Task can start but is not currently running.
    #[default]
    ReadyToStart,
    /// Task completed and ready to upload/report.
    ReadyToReport,
}

/// BOINC result/task as displayed in the Tasks pane.
#[derive(Debug, Clone, Default)]
pub struct Task {
    /// Owning project URL.
    pub project_url: String,
    /// BOINC result name.
    pub name: String,
    /// True when active execution is detected.
    pub active_task: bool,
    /// Whether suspended via GUI control.
    pub suspended_via_gui: bool,
    /// Progress in [0.0, 1.0] if available.
    pub fraction_done: Option<f64>,
    /// Normalized UI status bucket.
    pub status: TaskStatus,
    /// Elapsed runtime in seconds.
    pub elapsed_seconds: Option<f64>,
    /// Estimated remaining runtime in seconds.
    pub remaining_seconds: Option<f64>,
    /// Report deadline epoch (seconds since UNIX epoch).
    pub report_deadline: Option<f64>,
    /// Best-effort application/resource identifier.
    pub application: Option<String>,
}

/// BOINC transfer item from GUI RPC.
#[derive(Debug, Clone, Default)]
pub struct Transfer {
    /// Owning project URL.
    pub project_url: String,
    /// Transfer file name.
    pub file_name: String,
    /// Transfer status string from BOINC.
    pub status: String,
    /// Total file size in bytes, if known.
    pub nbytes: Option<u64>,
    /// Bytes transferred so far.
    pub bytes_xferred: Option<u64>,
    /// Current transfer speed in bytes/sec.
    pub xfer_speed: Option<f64>,
    /// True when this is an upload (generated locally).
    pub is_upload: bool,
    /// Error message if transfer is in a retry/error state.
    pub error_msg: Option<String>,
}

/// BOINC run mode setting values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    /// Always enabled.
    Always,
    /// Automatic behavior.
    Auto,
    /// Disabled.
    Never,
}

impl RunMode {
    /// Returns the BOINC mode tag used in `set_*_mode` requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use boincrs::boinc::models::RunMode;
    ///
    /// assert_eq!(RunMode::Always.as_boinc_tag(), "always");
    /// assert_eq!(RunMode::Auto.as_boinc_tag(), "auto");
    /// assert_eq!(RunMode::Never.as_boinc_tag(), "never");
    /// ```
    pub fn as_boinc_tag(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Auto => "auto",
            Self::Never => "never",
        }
    }
}

/// Client-wide BOINC status snapshot.
#[derive(Debug, Clone)]
pub struct ClientState {
    /// Network activity mode.
    pub network_mode: RunMode,
    /// CPU run mode.
    pub run_mode: RunMode,
    /// GPU run mode.
    pub gpu_mode: RunMode,
    /// Optional message text payloads.
    pub messages: Vec<String>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            network_mode: RunMode::Auto,
            run_mode: RunMode::Auto,
            gpu_mode: RunMode::Auto,
            messages: Vec::new(),
        }
    }
}
