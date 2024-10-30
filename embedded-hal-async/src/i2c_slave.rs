//! Async I2C Slave API.

pub use embedded_hal::i2c::{Error, ErrorKind, ErrorType};

/// Async I2cSlave.
pub trait I2cSlave: ErrorType {
    /// Wait asynchronously for request from an I2C master.
    async fn listen(&mut self) -> Result<Request, Self::Error>;

    /// Respond to an I2C Master Read request, asynchronously.
    async fn respond_to_read(&mut self, buffer: &[u8]) -> Result<Status, Self::Error>;

    /// Respond to an I2C Master Write request, asynchronously.
    async fn respond_to_write(&mut self, buffer: &mut [u8]) -> Result<Status, Self::Error>;

    /// Respond to an I2C Master General Call request, asynchronously.
    async fn respond_to_general_call(&mut self, buffer: &mut [u8]) -> Result<Status, Self::Error>;
}

/// Received request
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Request {
    /// I2C Master Read
    Read,

    /// I2C Master Write-then-read
    WriteRead(usize),

    /// I2C Master Write
    Write(usize),

    /// General Call
    GeneralCall(usize),
}

/// Possible responses to responding to a read
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Status {
    /// Request complete.
    Complete,

    /// Request incomplete, controller trying to transfer more bytes than were provided
    Incomplete,

    /// Request complete, but controller did not transfer all bytes.
    PartialRequest(usize),
}

impl<T: I2cSlave + ?Sized> I2cSlave for &mut T {
    #[inline]
    async fn listen(&mut self) -> Result<Request, Self::Error> {
        T::listen(self).await
    }

    #[inline]
    async fn respond_to_read(&mut self, buffer: &[u8]) -> Result<Status, Self::Error> {
        T::respond_to_read(self, buffer).await
    }

    #[inline]
    async fn respond_to_write(&mut self, buffer: &mut [u8]) -> Result<Status, Self::Error> {
        T::respond_to_write(self, buffer).await
    }

    #[inline]
    async fn respond_to_general_call(&mut self, buffer: &mut [u8]) -> Result<Status, Self::Error> {
        T::respond_to_general_call(self, buffer).await
    }
}
