pub mod ssd_git;

// Generated from operation getCoreContrailJuniperNetV4APIResources

/// get available resources
///
/// Use the returned [`crate::ResponseBody`]`<`[`GetCoreContrailJuniperNetV4APIResourcesResponse`]`>` constructor, or [`GetCoreContrailJuniperNetV4APIResourcesResponse`] directly, to parse the HTTP response.
#[cfg(feature = "api")]
pub fn get_core_contrail_juniper_net_v4_api_resources(
) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<GetCoreContrailJuniperNetV4APIResourcesResponse>), crate::RequestError> {
    let __url = "/apis/core.contrail.juniper.net/v4/".to_owned();

    let __request = crate::http::Request::get(__url);
    let __body = vec![];
    match __request.body(__body) {
        Ok(request) => Ok((request, crate::ResponseBody::new)),
        Err(err) => Err(crate::RequestError::Http(err)),
    }
}

/// Use `<GetCoreContrailJuniperNetV4APIResourcesResponse as Response>::try_from_parts` to parse the HTTP response body of [`get_core_contrail_juniper_net_v4_api_resources`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum GetCoreContrailJuniperNetV4APIResourcesResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for GetCoreContrailJuniperNetV4APIResourcesResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((GetCoreContrailJuniperNetV4APIResourcesResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((GetCoreContrailJuniperNetV4APIResourcesResponse::Other(result), read))
            },
        }
    }
}
