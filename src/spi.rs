//! Helpers and functions relating to W5500 SPI transfers.

/// SPI Access Modes.
#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum AccessMode {
    /// Read access.
    Read = 0,
    /// Write access.
    Write = 1,
}

impl From<AccessMode> for u8 {
    fn from(val: AccessMode) -> Self {
        val as u8
    }
}

/// SPI header length.
pub const HEADER_LEN: usize = 3;

/// Helper to create a variable data length SPI header.
///
/// # Example
///
/// SPI header to read the VERSIONR register in variable data length mode.
///
/// ```
/// use w5500_ll::{reg, spi, COMMON_BLOCK_OFFSET};
///
/// let hdr = spi::vdm_header(reg::VERSIONR, COMMON_BLOCK_OFFSET, spi::AccessMode::Read);
/// assert_eq!(hdr, [0x00, 0x39, 0x00]);
/// ```
#[inline(always)]
pub const fn vdm_header(address: u16, block: u8, mode: AccessMode) -> [u8; HEADER_LEN] {
    [
        (address >> 8) as u8,
        address as u8,
        (block << 3) | ((mode as u8) << 2),
    ]
}

/// Helper to create a 1 byte fixed data length SPI header.
///
/// # Example
///
/// SPI header to read the VERSIONR register in fixed data length mode.
///
/// ```
/// use w5500_ll::{reg, spi, COMMON_BLOCK_OFFSET};
///
/// let hdr = spi::fdm_header_1b(reg::VERSIONR, COMMON_BLOCK_OFFSET, spi::AccessMode::Read);
/// assert_eq!(hdr, [0x00, 0x39, 0x01]);
/// ```
pub const fn fdm_header_1b(address: u16, block: u8, mode: AccessMode) -> [u8; HEADER_LEN] {
    [
        (address >> 8) as u8,
        address as u8,
        (block << 3) | ((mode as u8) << 2) | 0b01,
    ]
}

/// Helper to create a 2 byte fixed data length SPI header.
///
/// # Example
///
/// SPI header to read the UPORTR register in fixed data length mode.
///
/// ```
/// use w5500_ll::{reg, spi, COMMON_BLOCK_OFFSET};
///
/// let hdr = spi::fdm_header_2b(reg::UPORTR, COMMON_BLOCK_OFFSET, spi::AccessMode::Read);
/// assert_eq!(hdr, [0x00, 0x2C, 0x02]);
/// ```
pub const fn fdm_header_2b(address: u16, block: u8, mode: AccessMode) -> [u8; HEADER_LEN] {
    [
        (address >> 8) as u8,
        address as u8,
        (block << 3) | ((mode as u8) << 2) | 0b10,
    ]
}

/// Helper to create a 4 byte fixed data length SPI header.
///
/// # Example
///
/// SPI header to read the UIPR register in fixed data length mode.
///
/// ```
/// use w5500_ll::{reg, spi, COMMON_BLOCK_OFFSET};
///
/// let hdr = spi::fdm_header_4b(reg::UIPR, COMMON_BLOCK_OFFSET, spi::AccessMode::Read);
/// assert_eq!(hdr, [0x00, 0x28, 0x03]);
/// ```
pub const fn fdm_header_4b(address: u16, block: u8, mode: AccessMode) -> [u8; HEADER_LEN] {
    [
        (address >> 8) as u8,
        address as u8,
        (block << 3) | ((mode as u8) << 2) | 0b11,
    ]
}

/// Recommended W5500 SPI mode.
///
/// The W5500 may operate in SPI mode 0 or SPI mode 3.
#[cfg(any(feature = "embedded-hal", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-hal")))]
pub const MODE: embedded_hal::spi::Mode = embedded_hal::spi::Mode {
    polarity: embedded_hal::spi::Polarity::IdleLow,
    phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
};
