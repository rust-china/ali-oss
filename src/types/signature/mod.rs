mod header_signature;
mod param_signature;

pub use header_signature::HeaderSignature;
pub use param_signature::ParamSignature;

use std::ops::Deref;
pub trait SignatureAble {
	fn get_signature_string<T: Deref<Target = crate::OssConfig>>(&self, oss_config: T) -> String;
}
