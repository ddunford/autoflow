use autoflow_data::Result;
use std::fmt;

/// Quality gate trait
pub trait QualityGate: Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, context: &GateContext) -> Result<GateResult>;
    fn is_critical(&self) -> bool {
        true
    }
}

/// Context passed to quality gates
#[derive(Debug, Clone)]
pub struct GateContext {
    pub sprints_path: String,
    pub project_root: String,
    pub auto_fix: bool,
}

impl GateContext {
    pub fn new(sprints_path: String, project_root: String) -> Self {
        Self {
            sprints_path,
            project_root,
            auto_fix: false,
        }
    }

    pub fn with_auto_fix(mut self, auto_fix: bool) -> Self {
        self.auto_fix = auto_fix;
        self
    }
}

/// Result from a quality gate
#[derive(Debug, Clone)]
pub struct GateResult {
    pub passed: bool,
    pub gate_name: String,
    pub messages: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub fixed: bool,
}

impl GateResult {
    pub fn pass(gate_name: String) -> Self {
        Self {
            passed: true,
            gate_name,
            messages: vec![],
            errors: vec![],
            warnings: vec![],
            fixed: false,
        }
    }

    pub fn fail(gate_name: String, errors: Vec<String>) -> Self {
        Self {
            passed: false,
            gate_name,
            messages: vec![],
            errors,
            warnings: vec![],
            fixed: false,
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.messages.push(message);
        self
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_fixed(mut self) -> Self {
        self.fixed = true;
        self
    }
}

impl fmt::Display for GateResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.passed {
            write!(f, "✅ {} - PASSED", self.gate_name)?;
        } else {
            write!(f, "❌ {} - FAILED", self.gate_name)?;
        }

        if self.fixed {
            write!(f, " (auto-fixed)")?;
        }

        if !self.messages.is_empty() {
            write!(f, "\n  {}", self.messages.join("\n  "))?;
        }

        if !self.errors.is_empty() {
            write!(f, "\n  Errors:\n    {}", self.errors.join("\n    "))?;
        }

        if !self.warnings.is_empty() {
            write!(f, "\n  Warnings:\n    {}", self.warnings.join("\n    "))?;
        }

        Ok(())
    }
}

/// Quality report aggregating all gate results
#[derive(Debug)]
pub struct QualityReport {
    pub results: Vec<GateResult>,
    pub passed: bool,
}

impl QualityReport {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            passed: true,
        }
    }

    pub fn add_result(&mut self, result: GateResult) {
        if !result.passed {
            self.passed = false;
        }
        self.results.push(result);
    }

    pub fn total_gates(&self) -> usize {
        self.results.len()
    }

    pub fn passed_gates(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_gates(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub fn auto_fixed_gates(&self) -> usize {
        self.results.iter().filter(|r| r.fixed).count()
    }
}

impl fmt::Display for QualityReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n📊 Quality Gate Report")?;
        writeln!(f, "════════════════════════════════════════")?;

        for result in &self.results {
            writeln!(f, "\n{}", result)?;
        }

        writeln!(f, "\n════════════════════════════════════════")?;
        writeln!(f, "Total Gates: {}", self.total_gates())?;
        writeln!(f, "Passed: {}", self.passed_gates())?;
        writeln!(f, "Failed: {}", self.failed_gates())?;

        if self.auto_fixed_gates() > 0 {
            writeln!(f, "Auto-fixed: {}", self.auto_fixed_gates())?;
        }

        writeln!(f)?;

        if self.passed {
            writeln!(f, "✅ All quality gates passed!")?;
        } else {
            writeln!(f, "❌ Some quality gates failed!")?;
        }

        Ok(())
    }
}

/// Quality gate pipeline
pub struct QualityPipeline {
    gates: Vec<Box<dyn QualityGate>>,
    stop_on_failure: bool,
}

impl QualityPipeline {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            stop_on_failure: true,
        }
    }

    pub fn add_gate<G: QualityGate + 'static>(mut self, gate: G) -> Self {
        self.gates.push(Box::new(gate));
        self
    }

    pub fn stop_on_failure(mut self, stop: bool) -> Self {
        self.stop_on_failure = stop;
        self
    }

    /// Run all gates in sequence
    pub fn run(&self, context: &GateContext) -> Result<QualityReport> {
        let mut report = QualityReport::new();

        for gate in &self.gates {
            tracing::info!("Running quality gate: {}", gate.name());

            match gate.run(context) {
                Ok(result) => {
                    let passed = result.passed;
                    let is_critical = gate.is_critical();

                    report.add_result(result);

                    if self.stop_on_failure && !passed && is_critical {
                        tracing::warn!("Critical gate failed, stopping pipeline");
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("Gate {} failed with error: {}", gate.name(), e);

                    let result = GateResult::fail(
                        gate.name().to_string(),
                        vec![format!("Gate execution error: {}", e)],
                    );

                    report.add_result(result);

                    if self.stop_on_failure && gate.is_critical() {
                        break;
                    }
                }
            }
        }

        Ok(report)
    }
}

impl Default for QualityPipeline {
    fn default() -> Self {
        Self::new()
    }
}
