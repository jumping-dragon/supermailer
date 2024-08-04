resource "aws_route53_record" "cert_validation" {
  depends_on = [
    aws_acm_certificate.cert
  ]
  for_each = {
    for domain in aws_acm_certificate.cert.domain_validation_options : domain.domain_name => {
      name   = domain.resource_record_name
      record = domain.resource_record_value
      type   = domain.resource_record_type
    }
  }
  zone_id = "${data.aws_route53_zone.zone.id}"
  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  type            = each.value.type
  ttl             = 60
}

resource "aws_route53_record" "dns_record_a" {
  for_each = toset(local.domain_name)

  zone_id = data.aws_route53_zone.zone.id
  name    = each.value
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.s3_distribution.domain_name
    zone_id                = aws_cloudfront_distribution.s3_distribution.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "dns_record_aaaa" {
  for_each = toset(local.domain_name)

  zone_id = data.aws_route53_zone.zone.zone_id
  name    = each.value
  type    = "AAAA"

  alias {
    name                   = aws_cloudfront_distribution.s3_distribution.domain_name
    zone_id                = aws_cloudfront_distribution.s3_distribution.hosted_zone_id
    evaluate_target_health = false
  }
}
