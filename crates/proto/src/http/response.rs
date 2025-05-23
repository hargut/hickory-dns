// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! HTTP request creation and validation

use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{Response, StatusCode};

use crate::error::ProtoError;
use crate::http::Version;
use crate::http::error::Result;

/// Create a new Response for an http dns-message request
///
/// ```text
/// RFC 8484              DNS Queries over HTTPS (DoH)          October 2018
///
///  4.2.1.  Handling DNS and HTTP Errors
///
/// DNS response codes indicate either success or failure for the DNS
/// query.  A successful HTTP response with a 2xx status code (see
/// Section 6.3 of [RFC7231]) is used for any valid DNS response,
/// regardless of the DNS response code.  For example, a successful 2xx
/// HTTP status code is used even with a DNS message whose DNS response
/// code indicates failure, such as SERVFAIL or NXDOMAIN.
///
/// HTTP responses with non-successful HTTP status codes do not contain
/// replies to the original DNS question in the HTTP request.  DoH
/// clients need to use the same semantic processing of non-successful
/// HTTP status codes as other HTTP clients.  This might mean that the
/// DoH client retries the query with the same DoH server, such as if
/// there are authorization failures (HTTP status code 401; see
/// Section 3.1 of [RFC7235]).  It could also mean that the DoH client
/// retries with a different DoH server, such as for unsupported media
/// types (HTTP status code 415; see Section 6.5.13 of [RFC7231]), or
/// where the server cannot generate a representation suitable for the
/// client (HTTP status code 406; see Section 6.5.6 of [RFC7231]), and so
/// on.
/// ```
pub fn new(version: Version, message_len: usize) -> Result<Response<()>> {
    Response::builder()
        .status(StatusCode::OK)
        .version(version.to_http())
        .header(CONTENT_TYPE, crate::http::MIME_APPLICATION_DNS)
        .header(CONTENT_LENGTH, message_len)
        .body(())
        .map_err(|e| ProtoError::from(format!("invalid response: {e}")).into())
}
