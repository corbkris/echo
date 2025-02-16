DEVOPS_DIR = devops/develop

#makefile is used for starting services

.PHONY: up_dev down_dev logs_dev clean_postgres_dev clean_volumes_dev logs_service_dev up_server_dev up_watch_dev

up_dev:
	@cd $(DEVOPS_DIR) && podman compose up -d

up_watch_dev:
	@cd $(DEVOPS_DIR) && podman compose up

up_server_dev:
	@cd $(DEVOPS_DIR) && task queues

down_dev:
	@cd $(DEVOPS_DIR) && podman compose down

logs_dev:
	@cd $(DEVOPS_DIR) && podman compose logs

logs_service_dev:
	@cd $(DEVOPS_DIR) && podman compose logs $(service)	

clean_postgres_dev:
	@cd $(DEVOPS_DIR)/postgres && sudo rm -r pgdata/

clean_volumes_dev:
	podman system prune && podman volume rm $$(podman volume ls -q)
