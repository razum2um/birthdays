variable "project_id" {
  description = "project id"
  type        = string
}

variable "region" {
  description = "region"
  type        = string
}

variable "cluster_name" {
  description = "Cluster name"
  type        = string
}

variable "db_master_zone" {
  description = "The zone where PgBouncer will be created."
  type        = string
}

variable "bastion_members" {
  description = "List of users, groups, SAs who need access to the bastion host (e.g. \"group:devs@example.com\", \"user:me@example.com\")"
  type        = list(string)
  default     = []
}

variable "network_name" {
  description = "Cluster network name"
  type        = string
}

variable "subnet_name" {
  description = "Cluster subnetwork name"
  type        = string
}

variable "subnet_ip" {
  description = "Cluster subnetwork ip"
  type        = string
}

variable "ip_range_pods_name" {
  description = "ip range for pods"
  type        = string
}

variable "ip_range_services_name" {
  description = "ip range for podservices"
  type        = string
}
