
module "log_group" {
  source  = "terraform-aws-modules/cloudwatch/aws//modules/log-group"
  version = "~> 2.0"

  name              = "enterprise-add"
  retention_in_days = 120

  tags = {
    Project = "enterprise-add"
  }
}

module "api_gateway" {
  source = "terraform-aws-modules/apigateway-v2/aws"

  name          = "add-http"
  description   = "Enterprise Add HTTP API Gateway"
  protocol_type = "HTTP"

  create_api_domain_name = false
  create_default_stage   = true

  cors_configuration = {
    allow_headers = ["content-type", "x-amz-date", "authorization", "x-api-key", "x-amz-security-token", "x-amz-user-agent"]
    allow_methods = ["*"]
    allow_origins = ["*"]
  }

  # Custom domain
  #   domain_name                 = "terraform-aws-modules.modules.tf"
  #   domain_name_certificate_arn = "arn:aws:acm:eu-west-1:052235179155:certificate/2b3a7ed9-05e1-4f9e-952b-27744ba06da6"
  
  # Access logs
  # default_stage_access_log_destination_arn = module.log_group.cloudwatch_log_group_arn
  # default_stage_access_log_format          = "$context.identity.sourceIp - - [$context.requestTime] \"$context.httpMethod $context.routeKey $context.protocol\" $context.status $context.responseLength $context.requestId $context.integrationErrorMessage"

  # Routes and integrations
  integrations = {
    "POST /add-numbers" = {
      lambda_arn             = module.lambda_function.lambda_function_arn
      payload_format_version = "2.0"
      timeout_milliseconds   = 12000
    }

    # "$default" = {
    #   lambda_arn = "arn:aws:lambda:eu-west-1:052235179155:function:my-default-function"
    # }
  }

  tags = {
    Name    = "http-apigateway"
    Project = "enterprise-add"
  }
}
