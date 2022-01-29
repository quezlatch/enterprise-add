module "add_queue" {
  source  = "terraform-aws-modules/sqs/aws"
  version = "~> 3.3"

  name = "add_queue"

  tags = var.tags
}

module "sqs_lambda" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "~> 2.0"

  function_name = "sqs-lambda"
  description   = "${var.project} SQS"
  handler       = "dummy"
  runtime       = "provided.al2"
  architectures = ["x86_64"]

  publish = true

  create_package         = false
  local_existing_package = "${var.lambda_dir}/add-numbers/target/rust.zip"

  event_source_mapping = {
    sqs = {
      event_source_arn = module.add_queue.sqs_queue_arn
    }
  }

  allowed_triggers = {
    sqs = {
      principal  = "sqs.amazonaws.com"
      source_arn = module.add_queue.sqs_queue_arn
    }
  }

  create_current_version_allowed_triggers = false

  attach_policies    = true
  number_of_policies = 1

  policies = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaSQSQueueExecutionRole",
  ]

  tags = var.tags
}