module "gke" {
  source  = "terraform-google-modules/kubernetes-engine/google//modules/safer-cluster"
  version = "~> 25.0.0"

  project_id = module.enabled_google_apis.project_id
  name       = var.cluster_name
  region     = var.region
  network    = module.vpc.network_name
  # tflint-ignore: terraform_deprecated_index
  subnetwork = module.vpc.subnets_names[0]
  # tflint-ignore: terraform_deprecated_index
  ip_range_pods = module.vpc.subnets_secondary_ranges[0].*.range_name[0]
  # tflint-ignore: terraform_deprecated_index
  ip_range_services       = module.vpc.subnets_secondary_ranges[0].*.range_name[1]
  enable_private_endpoint = false
  master_authorized_networks = [{
    cidr_block   = "${module.bastion.ip_address}/32"
    display_name = "Bastion Host"
  }]
  grant_registry_access = true
  node_pools = [
    {
      name          = "gke-pool"
      min_count     = 4
      max_count     = 5
      auto_upgrade  = true
      node_metadata = "GKE_METADATA"
    }
  ]
}

# terraform import google_service_account.cluster_sa projects/${var.project_id}/serviceAccounts/${module.gke.service_account}@${var.project_id}.iam.gserviceaccount.com
resource "google_service_account" "cluster_sa" {
  display_name = "Terraform-managed service account for cluster cluster"
  account_id   = trimsuffix(module.gke.service_account, "@${var.project_id}.iam.gserviceaccount.com")
}

module "app-workload-identity" {
  source              = "terraform-google-modules/kubernetes-engine/google//modules/workload-identity"
  version             = "~> 25.0.0"
  use_existing_gcp_sa = true
  use_existing_k8s_sa = true
  annotate_k8s_sa     = false
  name                = google_service_account.cluster_sa.account_id
  k8s_sa_name         = "${var.project_id}-ksa"
  namespace           = "default"
  project_id          = var.project_id
}