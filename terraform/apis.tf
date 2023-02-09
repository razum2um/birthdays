module "enabled_google_apis" {
  source  = "terraform-google-modules/project-factory/google//modules/project_services"
  version = "~> 14.0"

  project_id                  = var.project_id
  disable_services_on_destroy = false

  activate_apis = [
    "serviceusage.googleapis.com",
    "iam.googleapis.com",
    "compute.googleapis.com",
    "logging.googleapis.com",
    "monitoring.googleapis.com",
    "containerregistry.googleapis.com",
    "container.googleapis.com",
    "binaryauthorization.googleapis.com",
    "stackdriver.googleapis.com",
    "iap.googleapis.com",
    "servicenetworking.googleapis.com",
    "sqladmin.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "domains.googleapis.com",
    # gcloud dns --project=birthday-app managed-zones create birthday-app --description="" --dns-name="birthday-app.com." --visibility="public" --dnssec-state="on"
    # gcloud dns --project=birthday-app record-sets create birthday-app.com. --zone="birthday-app" --type="A" --ttl="300" --routing-policy-type="WRR" --routing-policy-data="0.0=34.78.15.100;0.0=34.22.239.59"
    "dns.googleapis.com"
  ]
}
