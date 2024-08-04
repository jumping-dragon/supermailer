resource "aws_acm_certificate" "cert" {
  domain_name       = "courriel.alvinjanuar.com"
  # subject_alternative_names = [ "courriel.alvinjanuar.com"]
  validation_method = "DNS"

  tags = {
    Environment = "test"
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_acm_certificate_validation" "cert" {
  certificate_arn         = "${aws_acm_certificate.cert.arn}"
  validation_record_fqdns = [for record in aws_route53_record.cert_validation : record.fqdn]
}