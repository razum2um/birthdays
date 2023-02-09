terraform {
  required_providers {
    google      = ">= 4.48.0"
    google-beta = ">= 4.52.0"
    random      = "~> 3.4.3"
    template    = "~> 2.2.0"
  }

  required_version = "~> 1.3.6"
}

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  zone    = var.db_master_zone
  project = var.project_id
  region  = var.region
}