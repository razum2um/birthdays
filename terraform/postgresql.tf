# apply was taking hours, then complaining about no available ip_ranges
# had to create database inside the cloud console, but same settings

# resource "random_id" "suffix" {
#   byte_length = 5
# }

# module "db-private-service-access" {
#   source      = "GoogleCloudPlatform/sql-db/google//modules/private_service_access"
#   project_id  = var.project_id
#   vpc_network = module.vpc.network_name
# }

# module "db" {
#   module_depends_on = [module.db-private-service-access.peering_completed]
#   source  = "GoogleCloudPlatform/sql-db/google//modules/postgresql"
#   version = "~> 14.0.0"
#   project_id = var.project_id
#   name       = "birthdays-${random_id.suffix.hex}"
#   database_version = "POSTGRES_14"
#   region           = var.region
#   zone             = var.db_master_zone
#   tier             = "db-g1-small"
#   availability_type = "REGIONAL"
#   backup_configuration = {
#     enabled                        = true
#     retained_backups               = 2
#     retention_unit                 = "COUNT"
#     location                       = ""
#     point_in_time_recovery_enabled = true
#     start_time                     = null
#     transaction_log_retention_days = "1"
#   }
#   ip_configuration = {
#     ipv4_enabled        = false
#     private_network     = "projects/${var.project_id}/global/networks/${module.vpc.network_name}"
#     require_ssl         = true
#     authorized_networks = []
#   }
#   maintenance_window_update_track = "stable"
#   create_timeout = "2h"
# }
