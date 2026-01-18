#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumString,
    strum::AsRefStr,
)]
pub enum BundleKind {
    #[strum(to_string = "static")]
    Static,
    #[strum(to_string = "nodejs")]
    NodeJS,
}
