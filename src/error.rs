
use failure::{ Backtrace, Context, Fail };

use std::result;
use std::path::{ Path, PathBuf };
use std::fmt;

pub type GLResult<T> = result::Result<T, GLError>;

// -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct GLError {
    ctx: Context<GLErrorKind>,
}

impl GLError {

    pub fn kind(&self) -> &GLErrorKind {
        self.ctx.get_context()
    }

    pub fn args(description: impl AsRef<str>) -> GLError {
        GLError::from(GLErrorKind::Args { description: description.as_ref().to_string() })
    }

    pub fn unsupported(feature: &'static str) -> GLError {
        GLError::from(GLErrorKind::UnSupport { feature })
    }

    pub fn device(ops_description: &'static str) -> GLError {
        GLError::from(GLErrorKind::Device { ops_description })
    }

    pub fn window(description: impl AsRef<str>) -> GLError {
        GLError::from(GLErrorKind::Window { description: description.as_ref().to_string() })
    }

    /// A convenience routine for creating an error associated with a path.
    pub fn path(path: impl AsRef<Path>)-> GLError {
        GLError::from(GLErrorKind::Path { path: path.as_ref().to_path_buf() })
    }

    pub fn unimplemented(function: impl AsRef<str>) -> GLError {
        GLError::from(GLErrorKind::Unimplemented { function: function.as_ref().to_string() })
    }

    pub fn custom(description: impl AsRef<str>) -> GLError {
        GLError::from(GLErrorKind::Custom {
            description: description.as_ref().to_string()
        })
    }
}

impl Fail for GLError {

    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for GLError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}
// -------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------
/// The specific kind of error that can occur.
#[derive(Debug, Fail)]
pub enum GLErrorKind {

    /// An error occurred while building connection between application and Vulkan.
    #[fail(display = "Something wrong when parsing command line arguments: \n\t{}", description)]
    Args { description: String },
    /// An error occurred while compiling shaders in OpenGL.
    #[fail(display = "Failed to create program: {:?}", _0)]
    CreateProgram(glium::program::ProgramCreationError),
    /// An error indicated requiring some unsupported feature.
    #[fail(display = "Feature {} is not supported in current OpenGL Device.", feature)]
    UnSupport { feature: &'static str },
    /// An error triggered by Invalid Device operations.
    #[fail(display = "Invalid Operation: {}", ops_description)]
    Device { ops_description: &'static str },
    /// An error occurred while communicate with Window.
    #[fail(display = "Failed to interact with Window: {}.", description)]
    Window { description: String },
    /// An error that occurred while working with a file path.
    #[fail(display = "Failed to locate file at: {:?}", path)]
    Path { path: PathBuf },
    #[fail(display = "{} is not implemented yet.", function)]
    Unimplemented { function: String },
    /// Other errors.
    #[fail(display = "{}", description)]
    Custom { description: String },
}

impl From<GLErrorKind> for GLError {

    fn from(kind: GLErrorKind) -> GLError {
        GLError::from(Context::new(kind))
    }
}

impl From<Context<GLErrorKind>> for GLError {

    fn from(ctx: Context<GLErrorKind>) -> GLError {
        GLError { ctx }
    }
}
// -------------------------------------------------------------------------------------------
