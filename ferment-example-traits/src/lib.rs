pub mod from_proof;
pub mod fermented;

extern crate ferment_macro;

pub mod nested {
    use crate::from_proof::from_proof::FromProof;

    #[ferment_macro::export]
    pub enum ProtocolError {
        IdentifierError(String)
    }


    #[ferment_macro::export]
    pub struct FromProofImpl {
        pub obj: String,
    }

    #[ferment_macro::export]
    #[derive(Debug)]
    pub enum IdentityRequest {
        Get(String),
    }
    #[derive(Debug)]
    #[ferment_macro::export]
    pub enum IdentityResponse {
        Get(String)
    }

    impl<Req> FromProof<Req> for FromProofImpl {
        type Request = IdentityRequest;
        type Response = IdentityResponse;

        fn maybe_from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(request: I, response: O, platform_version: u32) -> Result<Option<Self>, ProtocolError> where Self: Sized + 'a {
            println!("request: {:?}: {:?} {}", request.into(), response.into(), platform_version);
            Ok(None)
        }

        fn from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(request: I, response: O, platform_version: u32) -> Result<Self, ProtocolError> where Self: Sized + 'a {
            println!("request: {:?}: {:?} {}", request.into(), response.into(), platform_version);
            Err(ProtocolError::IdentifierError(format!("bad platform version {platform_version}")))
        }
    }
}