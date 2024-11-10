use crate::region::RegionCluster;
use crate::share::centralized::centralized_secrete_share_link_token::CentralizedSecretShareLinkToken;
use crate::share::versioning::UrlEncodedShareLinksVersioning;

pub struct CentralizedSecreteShareLinkTokenUrlEncoded {
    pub subdomain: String,
    pub domain: String,
    pub version: UrlEncodedShareLinksVersioning,
    pub token: String,
}



impl TryFrom<CentralizedSecretShareLinkToken> for CentralizedSecreteShareLinkTokenUrlEncoded {
    type Error = ();

    fn try_from(value: CentralizedSecretShareLinkToken) -> Result<Self, Self::Error> {
        let domain = "bucketdirve.co";
        let subdomain = RegionCluster;
        Ok(Self { subdomain: "".to_string(), domain: , version: UrlEncodedShareLinksVersioning::V1 })
    }
}