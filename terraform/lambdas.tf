module "api_lambda" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "~> 2.0"

  function_name = "api-lambda"
  description   = "${var.project} API"
  handler       = "dummy"
  runtime       = "provided.al2"
  architectures = ["x86_64"]

  publish = true

  create_package         = false
  local_existing_package = "${var.lambda_dir}/add-numbers-api/target/rust.zip"

  allowed_triggers = {
    AllowExecutionFromAPIGateway = {
      service    = "apigateway"
      source_arn = "${module.api_gateway.apigatewayv2_api_execution_arn}/*/*"
    }
  }

  tags = var.tags
}
