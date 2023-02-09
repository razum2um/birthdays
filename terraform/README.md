# Birthday infrastructure

## Deploy in GCP

You will require working `gcloud`, `terraform`, `heml` and prepare a blank project 

```
cp terraform.tfvars.example terraform.tfvars # edit variables
terraform init
terraform apply
```

## Lint

```
tflint
```