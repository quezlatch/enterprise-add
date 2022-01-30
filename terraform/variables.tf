variable "project" {
  default = "Enterprise Add"
}

variable "prefix" {
  default = "enterprise_add"
}

variable "lambda_dir" {
  default = "/workspaces/enterprise-add/lambda"
}

variable "tags" {
  default = {
    Project = "Enterprise Add"
    Org     = "quezlatch"
  }
}