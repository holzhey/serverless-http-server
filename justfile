# Deploy
deploy:
  cd service && just deploy
  cd infra && just deploy

# Deploy locally
deploy-local:
  cd service && just deploy
  cd infra && just deploy-local

# Destroy resources previously deployed
destroy:
  cd infra && just destroy

# Watch service logs
watch-local-service-logs:
  awslocal logs tail /aws/lambda/ApiServiceHandler --follow
