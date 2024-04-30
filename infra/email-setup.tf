terraform {
  backend "s3" {
    bucket = "alvinjanuar.com-stacks"
    key = "prod/supermailer/terraform.tfstate"
    region = "us-east-1"
  }
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      Owner   = "Alvin"
      Project = "Supermailer"
    }
  }
}

variable "zone_id" {
    default = "Z3CQ9EX4ULAT14"
}

variable "domain" {
  default = "alvinjanuar.com"
}

# ses spf
resource "aws_route53_record" "spf-record" {
    zone_id = "${var.zone_id}"
    name = ""
    type = "TXT"
    ttl = "300"
    records = [
        "v=spf1 include:amazonses.com -all"
    ]
}

resource "aws_ses_domain_identity" "domain-identity" {
  domain = "${var.domain}"
}

# ses dmarc
resource "aws_route53_record" "route_53_dmarc_txt" {
  zone_id = var.zone_id
  name    = "_dmarc.${var.domain}"
  type    = "TXT"
  ttl     = "300"
  records = [
    "v=DMARC1;p=quarantine;pct=75;rua=mailto:engineering@example.org"
    # "v=DMARC1;p=none;pct=100;rua=mailto:root@alvinjanuar.com"
  ]
}

# ses dkim
resource "aws_ses_domain_dkim" "domain-dkim" {
  domain = "${aws_ses_domain_identity.domain-identity.domain}"
}

resource "aws_route53_record" "dkim-records" {
  count   = 3
  zone_id = "${var.zone_id}"
  name    = "${element(aws_ses_domain_dkim.domain-dkim.dkim_tokens, count.index)}._domainkey.${var.domain}"
  type    = "CNAME"
  ttl     = "600"

  records = [
    "${element(aws_ses_domain_dkim.domain-dkim.dkim_tokens, count.index)}.dkim.amazonses.com",
  ]
}

# ses mail to records
resource "aws_route53_record" "mx-records" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  type    = "MX"
  ttl     = "600"

  records = [
    "10 inbound-smtp.us-east-1.amazonses.com",
    "10 inbound-smtp.us-east-1.amazonaws.com",
  ]
}

# ses rule set
resource "aws_ses_receipt_rule_set" "rule_set" {
  rule_set_name = "${var.domain}_receive_all"
}

resource "aws_ses_active_receipt_rule_set" "active-rule-set" {
  rule_set_name = "${aws_ses_receipt_rule_set.rule_set.rule_set_name}"

  depends_on = [
    aws_ses_receipt_rule.catch-all
  ]
}

# lambda catch all
resource "aws_ses_receipt_rule" "catch-all" {
  name          = "${var.domain}-catch-all"
  rule_set_name = "${aws_ses_receipt_rule_set.rule_set.rule_set_name}"

  recipients = [
    "${var.domain}",
  ]

  enabled      = true
  scan_enabled = true

  s3_action {
    bucket_name = "${aws_s3_bucket.mail-bucket.bucket}"
    topic_arn   = "${aws_sns_topic.mail-topic.arn}"
    position    = 1
  }

  lambda_action {
    function_arn = "${aws_lambda_function.inbox_lambda.arn}"
    invocation_type = "Event"
    position = 2
  }

  stop_action {
    scope    = "RuleSet"
    position = 3
  }

  depends_on = [
    aws_s3_bucket.mail-bucket,
    aws_s3_bucket_policy.mail-bucket-policy,
    aws_sns_topic.mail-topic,
  ]
}

resource "aws_s3_bucket" "mail-bucket" {
    bucket = "${var.domain}-mail-bucket"
}

resource "aws_s3_bucket_policy" "mail-bucket-policy" {
  bucket = aws_s3_bucket.mail-bucket.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      # {
      #   Sid       = "PublicReadGetObject"
      #   Effect    = "Allow"
      #   Principal = "*"
      #   Action    = "s3:GetObject"
      #   Resource = [
      #     aws_s3_bucket.mail-bucket.arn,
      #     "${aws_s3_bucket.mail-bucket.arn}/*",
      #   ]
      # },
      {
        Sid       = "AllowSESPuts"
        Effect    = "Allow"
        Principal = {
          "Service": "ses.amazonaws.com"
        }
        Action    = "s3:PutObject"
        Resource = [
          aws_s3_bucket.mail-bucket.arn,
          "${aws_s3_bucket.mail-bucket.arn}/*",
        ]
      },
    ]
  })
}

resource "aws_sns_topic" "mail-topic" {
  name = "mail-topic-receipt-sns"
}

resource "aws_sns_topic" "mail-error-topic" {
  name = "mail-topic-ses-error"
}

resource "aws_ses_configuration_set" "alvinjanuar" {
  name = "alvinjanuar-ses-configuration-set"
}

resource "aws_ses_event_destination" "ses_errors" {
  name                   = "ses-error-sns-destination"
  configuration_set_name = "${aws_ses_configuration_set.alvinjanuar.name}"
  enabled                = true

  matching_types = [
    "reject",
    "reject",
    "send",
  ]

  sns_destination {
    topic_arn = "${aws_sns_topic.mail-error-topic.arn}"
  }
}

resource "aws_ses_event_destination" "ses_cloudwatch" {
  name                   = "event-destination-cloudwatch"
  configuration_set_name = aws_ses_configuration_set.alvinjanuar.name
  enabled                = true
  matching_types         = ["bounce", "send", "reject"]

  cloudwatch_destination {
    default_value  = "default"
    dimension_name = "dimension"
    value_source   = "emailHeader"
  }
}

data "archive_file" "api_server_zip" {                                                                                                                                                                                   
  type        = "zip"                                                                                                                                                                                                                                         
  source_dir  = "../target/lambda/supermailer"                                                                                                                                                                                         
  output_path = "./api_server.zip"                                                                                                                                                                         
} 

resource "aws_lambda_function" "api_server" {
  architectures                  = ["arm64"]
  function_name                  = "api_server"
  handler                        = "bootstrap"
  memory_size                    = "2048"
  reserved_concurrent_executions = "-1"
  role                           = aws_iam_role.api_server_role.arn
  depends_on                     = [aws_iam_role_policy_attachment.api_server_policy_role_attachment]
  runtime                        = "provided.al2023"
  source_code_hash               = data.archive_file.api_server_zip.output_base64sha256  
  filename                       = data.archive_file.api_server_zip.output_path 
  timeout                        = "120"
  environment {
    variables = {
      # AWS_STS_REGIONAL_ENDPOINTS="regional",
      # TF_LOG="trace"
      MAIL_BUCKET="${aws_s3_bucket.mail-bucket.bucket}"
      MAIL_DB="${aws_dynamodb_table.example.name}"
    }
  }

  # vpc_config {
  #   subnet_ids         = [aws_subnet.subnet_public.id]
  #   security_group_ids = [aws_security_group.lambda_sg.id]
  # }

  # file_system_config {
  #   arn = aws_efs_access_point.lambda_access_point.arn
  #   local_mount_path = "/mnt/files"
  # }

  tracing_config {
    mode = "PassThrough"
  }
}

resource "aws_lambda_function_url" "function_url" {
  # TODO: Find a way to authenticate lambda function with CloudFront
  # checkov:skip=CKV_AWS_258:Lambda Function URL is public for CloudFront origin

  function_name      = aws_lambda_function.api_server.function_name
  authorization_type = "NONE"
}

resource "aws_iam_role" "api_server_role" {
  name               = "api_server_role"
  assume_role_policy = file("${path.module}/lambda_assume_role_policy.json")
}

resource "aws_iam_policy" "api_server_policy" {
  name        = "api_server_policy"
  path        = "/"
  description = "AWS IAM Policy for managing aws lambda role"
  policy      = file("${path.module}/lambda_policy.json")
}

resource "aws_iam_role_policy_attachment" "api_server_policy_role_attachment" {
  role       = aws_iam_role.api_server_role.name
  policy_arn = aws_iam_policy.api_server_policy.arn
}

data "archive_file" "inbox_lambda_zip" {                                                                                                                                                                                   
  type        = "zip"                                                                                                                                                                                                                                                                
  source_dir  = "../target/lambda/inbox"                                                                                                                                                                                      
  output_path = "./inbox_lambda.zip"                                                                                                                                                                         
} 

resource "aws_lambda_function" "inbox_lambda" {
  architectures                  = ["arm64"]
  function_name                  = "inbox_lambda"
  handler                        = "bootstrap"
  memory_size                    = "2048"
  reserved_concurrent_executions = "-1"
  role                           = aws_iam_role.api_server_role.arn
  depends_on                     = [aws_iam_role_policy_attachment.api_server_policy_role_attachment]
  runtime                        = "provided.al2023"
  source_code_hash               = data.archive_file.inbox_lambda_zip.output_base64sha256
  filename                       = data.archive_file.inbox_lambda_zip.output_path 
  timeout                        = "120"
  publish                        = true
  environment {
    variables = {
      # AWS_STS_REGIONAL_ENDPOINTS="regional",
      # TF_LOG="trace"
      MAIL_BUCKET="${aws_s3_bucket.mail-bucket.bucket}"
    }
  }

  # vpc_config {
  #   subnet_ids         = [aws_subnet.subnet_public.id]
  #   security_group_ids = [aws_security_group.lambda_sg.id]
  # }

  # file_system_config {
  #   arn = aws_efs_access_point.lambda_access_point.arn
  #   local_mount_path = "/mnt/files"
  # }

  tracing_config {
    mode = "PassThrough"
  }
}

resource "aws_lambda_permission" "allow_ses_to_lambda" {
    statement_id = "AllowExecutionFromSES"
    action = "lambda:InvokeFunction"
    function_name = aws_lambda_function.inbox_lambda.function_name
    principal = "ses.amazonaws.com"
    source_arn = aws_ses_receipt_rule.catch-all.arn
}

resource "aws_lambda_function_event_invoke_config" "lambda_to_ses_topic" {
  function_name = aws_lambda_function.inbox_lambda.function_name
  depends_on = [ aws_lambda_function.inbox_lambda ]
}

resource "aws_dynamodb_table" "example" {
  name             = "SupermailerTable"
  hash_key         = "pk"
  range_key        = "sk"
  billing_mode     = "PAY_PER_REQUEST"
  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "sk"
    type = "N"
  }
}