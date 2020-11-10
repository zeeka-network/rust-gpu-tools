use ocl;

#[derive(thiserror::Error, Debug)]
pub enum GPUError {
    #[error("Ocl Error: {0}")]
    Ocl(ocl::Error),
    #[error("Device not found!")]
    DeviceNotFound,
    #[error("Device info not available!")]
    DeviceInfoNotAvailable(ocl::enums::DeviceInfo),
    #[error("Device index out of range!")]
    DeviceIndexOutOfRange,
    #[error("Program info not available!")]
    ProgramInfoNotAvailable(ocl::enums::ProgramInfo),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}

#[allow(dead_code)]
pub type GPUResult<T> = std::result::Result<T, GPUError>;

impl From<ocl::Error> for GPUError {
    fn from(error: ocl::Error) -> Self {
        GPUError::Ocl(error)
    }
}

impl From<ocl::core::Error> for GPUError {
    fn from(error: ocl::core::Error) -> Self {
        GPUError::Ocl(error.into())
    }
}
