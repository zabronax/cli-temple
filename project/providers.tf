terraform {
  required_providers {
    github = {
      source  = "integrations/github"
      version = "6.10.2"
    }
  }
}

variable "github_token" {
  type        = string
  description = "The token to use for the GitHub provider"
  sensitive   = true
}

provider "github" {
  token = var.github_token
}
