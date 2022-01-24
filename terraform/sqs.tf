module "add_queue" {
  source  = "terraform-aws-modules/sqs/aws"
  version = "~> 2.0"

  name = "add_queue"

  tags = var.tags
}