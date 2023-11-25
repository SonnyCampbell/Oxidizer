#[derive(PartialEq, Clone, EnumIter, IntoStaticStr)]
pub enum WaveType {
    Sin,
    Saw,
    Triangle,
    Square,
    Pulse
}

impl Default for WaveType {
    fn default() -> Self {
        Self::Sin
    }
}