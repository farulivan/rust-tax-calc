#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerCategory {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PtkpStatus {
    TK0, TK1, TK2, TK3,
    K0, K1, K2, K3,
}

impl PtkpStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            PtkpStatus::TK0 => "TK/0 - Tidak Kawin, Tanpa Tanggungan",
            PtkpStatus::TK1 => "TK/1 - Tidak Kawin, 1 Tanggungan",
            PtkpStatus::TK2 => "TK/2 - Tidak Kawin, 2 Tanggungan",
            PtkpStatus::TK3 => "TK/3 - Tidak Kawin, 3 Tanggungan",
            PtkpStatus::K0  => "K/0  - Kawin, Tanpa Tanggungan",
            PtkpStatus::K1  => "K/1  - Kawin, 1 Tanggungan",
            PtkpStatus::K2  => "K/2  - Kawin, 2 Tanggungan",
            PtkpStatus::K3  => "K/3  - Kawin, 3 Tanggungan",
        }
    }

    pub fn get_ter_category(&self) -> TerCategory {
        match self {
            PtkpStatus::TK0 | PtkpStatus::TK1 | PtkpStatus::K0 => TerCategory::A,
            PtkpStatus::TK2 | PtkpStatus::TK3 | PtkpStatus::K1 | PtkpStatus::K2 => TerCategory::B,
            PtkpStatus::K3 => TerCategory::C,
        }
    }
}