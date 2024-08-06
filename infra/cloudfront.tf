resource "aws_cloudfront_origin_access_control" "oac" {
  name                              = "courriel.alvinjanuar.com-oac"
  description                       = "OAC for courriel.alvinjanuar.com"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

# resource "aws_cloudfront_function" "redirector" {
#   name    = "astro-multi-page-cf-redirector-function"
#   runtime = "cloudfront-js-1.0"
#   comment = "Remap paths to index.html when request origin for multi-page sites"
#   publish = true
#   code    = file("./redirector.js")
# }

resource "aws_cloudfront_distribution" "s3_distribution" {
  origin {
    domain_name              = aws_s3_bucket.bucket.bucket_regional_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.oac.id
    origin_id                = aws_s3_bucket.bucket.id
  }

  origin {
    domain_name              = "${aws_lambda_function_url.function_url.url_id}.lambda-url.${data.aws_region.current.name}.on.aws"
    # origin_access_control_id = aws_cloudfront_origin_access_control.oac.id
    origin_id                = aws_lambda_function_url.function_url.id
  
    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  enabled             = true
  is_ipv6_enabled     = true
  comment             = "CDN for courriel.alvinjanuar.com"
  # default_root_object = "index.html"

  logging_config {
    include_cookies = false
    bucket          = aws_s3_bucket.bucket.bucket_domain_name
    prefix          = "cloudfront"
  }

  aliases = [
    "courriel.alvinjanuar.com"
  ]

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = aws_s3_bucket.bucket.id

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }
    # function_association {
    #   event_type   = "viewer-request"
    #   function_arn = aws_cloudfront_function.redirector.arn
    # }

    viewer_protocol_policy = "allow-all"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  # Cache behavior with precedence 0
  ordered_cache_behavior {
    path_pattern     = "/favicon.ico"
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD", "OPTIONS"]
    target_origin_id = aws_s3_bucket.bucket.id

    forwarded_values {
      query_string = false
      headers      = ["Origin"]

      cookies {
        forward = "none"
      }
    }
    
    # function_association {
    #   event_type   = "viewer-request"
    #   function_arn = aws_cloudfront_function.redirector.arn
    # }

    min_ttl                = 0
    default_ttl            = 86400
    max_ttl                = 31536000
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  # Cache behavior with precedence 1
  ordered_cache_behavior {
    path_pattern     = "/pkg/*"
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD", "OPTIONS"]
    target_origin_id = aws_s3_bucket.bucket.id

    cache_policy_id = aws_cloudfront_cache_policy.s3_cache_policy.id
    origin_request_policy_id = aws_cloudfront_origin_request_policy.s3_origin_request_policy.id
    response_headers_policy_id = aws_cloudfront_response_headers_policy.s3_response_header_policy.id

    min_ttl                = 0
    default_ttl            = 86400
    max_ttl                = 31536000
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  # Cache behavior with precedence 2
  ordered_cache_behavior {
    path_pattern     = "*"
    allowed_methods  = ["POST", "HEAD", "PATCH", "DELETE", "PUT", "GET", "OPTIONS"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = aws_lambda_function_url.function_url.id

    cache_policy_id = "4135ea2d-6df8-44a3-9df3-4b5a84be39ad" #Managed-CachingDisabled
    origin_request_policy_id = "b689b0a8-53d0-40ab-baf2-68738e2966ac" #Managed-AllViewerExceptHostHeader
    response_headers_policy_id = "eaab4381-ed33-4a86-88ca-d9558dc6cd63" #Managed-CORS-with-preflight-and-SecurityHeadersPolicy

    min_ttl                = 0
    default_ttl            = 86400
    max_ttl                = 31536000
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  price_class = "PriceClass_200"

  restrictions {
    geo_restriction {
        restriction_type = "none"
    #   restriction_type = "whitelist"
    #   locations        = ["US", "CA", "GB", "DE"]
    }
  }
  
  viewer_certificate {
    cloudfront_default_certificate = false
    acm_certificate_arn = "${aws_acm_certificate.cert.arn}"
    ssl_support_method = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }
}

resource "aws_cloudfront_cache_policy" "s3_cache_policy" {
  name        = "s3-cache-policy"
  comment     = "S3 Cache policy"
  default_ttl = 50
  max_ttl     = 100
  min_ttl     = 1
  parameters_in_cache_key_and_forwarded_to_origin {
    cookies_config {
      cookie_behavior = "none"
    }
    headers_config {
      header_behavior = "whitelist"
      headers {
        items = ["Origin", "Authorization"]
      }
    }
    query_strings_config {
      query_string_behavior = "all"
    }
  }
}

resource "aws_cloudfront_response_headers_policy" "s3_response_header_policy" {
  name    = "s3-response-header-policy"
  comment = "S3 Response Header Policy"

  cors_config {
    access_control_allow_credentials = true

    access_control_allow_headers {
      items = ["Origin", "Authorization"]
    }

    access_control_allow_methods {
      items = ["ALL"]
    }

    access_control_allow_origins {
      items = ["courriel.alvinjanuar.com"]
    }

    origin_override = false
  }
}

resource "aws_cloudfront_origin_request_policy" "s3_origin_request_policy" {
  name    = "s3-origin-request-policy"
  comment = "S3 Origin Request Policy"
  cookies_config {
    cookie_behavior = "none"
  }
  headers_config {
    header_behavior = "whitelist"
    headers {
      items = ["Origin"]
    }
  }
  query_strings_config {
    query_string_behavior = "all"
  }
}

# resource "aws_cloudfront_distribution" "analytics_distribution" {
#   # Analytics Origin
#   origin {
#     domain_name = "us.i.posthog.com"
#     origin_id   = "analytics-origin"

#     custom_origin_config {
#       http_port              = 80
#       https_port             = 443
#       origin_protocol_policy = "https-only"
#       origin_ssl_protocols   = ["TLSv1.2"]
#     }
#   }

#   # Analytics assets Origin
#   origin {
#     domain_name = "us-assets.i.posthog.com"
#     origin_id   = "analytics-assets-origin"

#     custom_origin_config {
#       http_port              = 80
#       https_port             = 443
#       origin_protocol_policy = "https-only"
#       origin_ssl_protocols   = ["TLSv1.2"]
#     }
#   }

#   enabled             = true
#   is_ipv6_enabled     = true
#   comment             = "CDN for analytics of courriel.alvinjanuar.com"

#   default_cache_behavior {
#     allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
#     cached_methods   = ["GET", "HEAD"]
#     target_origin_id = "analytics-origin"

#     forwarded_values {
#       query_string = true

#       cookies {
#         forward = "none"
#       }
#     }
#     function_association {
#       event_type   = "viewer-request"
#       function_arn = aws_cloudfront_function.redirector.arn
#     }

#     viewer_protocol_policy = "allow-all"
#     min_ttl                = 0
#     default_ttl            = 3600
#     max_ttl                = 86400
#   }

#   custom_error_response {
#     error_caching_min_ttl = 10
#     error_code            = 403
#     response_code         = 200
#     response_page_path    = "/index.html"
#   }

#   custom_error_response {
#     error_caching_min_ttl = 10
#     error_code            = 404
#     response_code         = 200
#     response_page_path    = "/404.html"
#   }

#   # Cache behavior with precedence 0
#   ordered_cache_behavior {
#     path_pattern     = "/static/*"
#     allowed_methods  = ["GET", "HEAD", "OPTIONS"]
#     cached_methods   = ["GET", "HEAD", "OPTIONS"]
#     target_origin_id = "analytics-assets-origin"

#     cache_policy_id = aws_cloudfront_cache_policy.analytics_cache_policy.id
#     origin_request_policy_id = aws_cloudfront_origin_request_policy.analytics_origin_request_policy.id
#     response_headers_policy_id = aws_cloudfront_response_headers_policy.analytics_response_header_policy.id

#     min_ttl                = 0
#     default_ttl            = 86400
#     max_ttl                = 31536000
#     compress               = true
#     viewer_protocol_policy = "redirect-to-https"
#   }

#   # Cache behavior with precedence 1
#   ordered_cache_behavior {
#     path_pattern     = "*"
#     allowed_methods  = ["POST", "HEAD", "PATCH", "DELETE", "PUT", "GET", "OPTIONS"]
#     cached_methods   = ["GET", "HEAD"]
#     target_origin_id = "analytics-origin"

#     cache_policy_id = aws_cloudfront_cache_policy.analytics_cache_policy.id
#     origin_request_policy_id = aws_cloudfront_origin_request_policy.analytics_origin_request_policy.id
#     response_headers_policy_id = aws_cloudfront_response_headers_policy.analytics_response_header_policy.id

#     min_ttl                = 0
#     default_ttl            = 86400
#     max_ttl                = 31536000
#     compress               = true
#     viewer_protocol_policy = "redirect-to-https"
#   }

#   price_class = "PriceClass_200"

#   restrictions {
#     geo_restriction {
#         restriction_type = "none"
#     #   restriction_type = "whitelist"
#     #   locations        = ["US", "CA", "GB", "DE"]
#     }
#   }
  
#   viewer_certificate {
#     cloudfront_default_certificate = true
#     # acm_certificate_arn = "${aws_acm_certificate.cert.arn}"
#     ssl_support_method = "sni-only"
#     minimum_protocol_version = "TLSv1.2_2021"
#   }
# }

# resource "aws_cloudfront_cache_policy" "analytics_cache_policy" {
#   name        = "analytics-cache-policy"
#   comment     = "Analytics Cache policy"
#   default_ttl = 50
#   max_ttl     = 100
#   min_ttl     = 1
#   parameters_in_cache_key_and_forwarded_to_origin {
#     cookies_config {
#       cookie_behavior = "none"
#     }
#     headers_config {
#       header_behavior = "whitelist"
#       headers {
#         items = ["Origin", "Authorization"]
#       }
#     }
#     query_strings_config {
#       query_string_behavior = "all"
#     }
#   }
# }

# resource "aws_cloudfront_response_headers_policy" "analytics_response_header_policy" {
#   name    = "analytics-response-header-policy"
#   comment = "Analytics Response Header Policy"

#   cors_config {
#     access_control_allow_credentials = true

#     access_control_allow_headers {
#       items = ["Origin", "Authorization"]
#     }

#     access_control_allow_methods {
#       items = ["ALL"]
#     }

#     access_control_allow_origins {
#       items = ["www.courriel.alvinjanuar.com", "courriel.alvinjanuar.com"]
#     }

#     origin_override = false
#   }
# }

# resource "aws_cloudfront_origin_request_policy" "analytics_origin_request_policy" {
#   name    = "analytics-origin-request-policy"
#   comment = "Analytics Origin Request Policy"
#   cookies_config {
#     cookie_behavior = "none"
#   }
#   headers_config {
#     header_behavior = "whitelist"
#     headers {
#       items = ["Origin"]
#     }
#   }
#   query_strings_config {
#     query_string_behavior = "all"
#   }
# }