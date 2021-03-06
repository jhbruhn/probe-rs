use super::memory::MemoryRegion;
use std::borrow::Cow;

/// A single chip variant.
///
/// This describes an exact chip variant, including the flash and memory size. For example,
/// the `nRF52832` chip has two variants, `nRF52832_xxAA` and `nRF52832_xxBB`. For this case,
/// the struct will correspond to one of the variants, e.g. `nRF52832_xxAA`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chip {
    /// This is the name of the chip in base form.
    /// E.g. `nRF52832`.
    pub name: Cow<'static, str>,
    /// The `PART` register of the chip.
    /// This value can be determined via the `cli info` command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<u16>,
    /// The memory regions available on the chip.
    pub memory_map: Cow<'static, [MemoryRegion]>,
    /// Names of all flash algorithms available for this chip.
    ///
    /// This can be used to look up the flash algorithm in the
    /// [`ChipFamily::flash_algorithms`] field.
    ///
    /// [`ChipFamily::flash_algorithms`]: crate::config::ChipFamily::flash_algorithms
    pub flash_algorithms: Cow<'static, [Cow<'static, str>]>,
}
