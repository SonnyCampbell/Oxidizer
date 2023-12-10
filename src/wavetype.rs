#[derive(Debug, PartialEq, Copy, Clone, EnumIter, IntoStaticStr, EnumString, EnumVariantNames)]
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