# Create a bucket
resource "aws_s3_bucket" "bucket" {
  bucket = "courriel.alvinjanuar.com"
}

module "template_files" {
  source = "hashicorp/dir/template"
  base_dir = "../target/site"
}

resource "aws_s3_object" "codebase" {
    for_each = module.template_files.files
        bucket = aws_s3_bucket.bucket.id
        key          = each.key
        content_type = each.value.content_type

        # The template_files module guarantees that only one of these two attributes
        # will be set for each file, depending on whether it is an in-memory template
        # rendering result or a static file on disk.
        source  = each.value.source_path
        content = each.value.content

        # Unless the bucket has encryption enabled, the ETag of each object is an
        # MD5 hash of that object.
        etag = each.value.digests.md5
}

resource "aws_s3_bucket_policy" "read_only_from_oac_s3_policy" {
  bucket = aws_s3_bucket.bucket.id
  policy = data.aws_iam_policy_document.read_only_from_oac_s3_policy.json
}

# # IAM Policy Document
data "aws_iam_policy_document" "read_only_from_oac_s3_policy" {
  statement {
    actions   = ["s3:GetObject"]
    resources = ["${aws_s3_bucket.bucket.arn}/*"]

    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = ["${aws_cloudfront_distribution.s3_distribution.arn}"]
    }
  }
}

# Create a bucket
resource "aws_s3_bucket" "bucket-logs" {
  bucket = "courriel.alvinjanuar.com-logs"
}