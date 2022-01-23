variable "project" {
   default =   "Enterprise Add"
}

variable "prefix" {
   default =   "enterprise_add"
}

variable "lambda_dir" {
    default = "/Users/mikey/Projects/rust/enterprise-add/lambda"
}

locals {
  tags = {
      Project = var.project
      Org = "quezlatch"
  }
}