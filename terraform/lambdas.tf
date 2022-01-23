module "lambda_function" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "~> 2.0"

  function_name = "enterprise-add-api-lambda"
  description   = "Enterprise add api"
  handler       = "qq"
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
}