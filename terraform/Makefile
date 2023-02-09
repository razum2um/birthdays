.PHONY: tunnel
REGION ?= 'europe-west1'

tunnel:
	echo "Make calls using HTTPS_PROXY=localhost:8888 kubectl"
	gcloud beta compute ssh cluster-bastion --project birthday-app --zone ${REGION}-b -- -L8888:127.0.0.1:8888 -N

kubeconfig:
	gcloud container clusters get-credentials cluster --region ${REGION}
	sed -i '.bak' 's/35.195.201.42/10.0.0.2/' ${HOME}/.kube/config

login:
	gcloud auth application-default login
