resource "github_repository" "primary_repository" {
  name       = "cli-temple"
  visibility = "public"

  # Metadata
  description = "Structured templating CLI tool"

  # Features
  has_issues = true
}

output "repository_url" {
  description = "The URL of the repository"
  value       = github_repository.primary_repository.http_clone_url
}
