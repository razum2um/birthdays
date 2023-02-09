data "template_file" "startup_script" {
  template = <<-EOF
  sudo apt-get update -y
  sudo apt-get install -y tinyproxy
  EOF
}

module "bastion" {
  source  = "terraform-google-modules/bastion-host/google"
  version = "~> 5.0"

  network        = module.vpc.network_self_link
  subnet         = module.vpc.subnets_self_links[0]
  project        = module.enabled_google_apis.project_id
  host_project   = module.enabled_google_apis.project_id
  name           = format("%s-bastion", var.cluster_name)
  zone           = var.db_master_zone
  image_project  = "debian-cloud"
  machine_type   = "g1-small"
  startup_script = data.template_file.startup_script.rendered
  members        = var.bastion_members
  shielded_vm    = "false"
}
