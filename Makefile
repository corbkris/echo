DEVOPS_DIR = devops/develop

.PHONY: up_dev down_dev logs_dev clean_postgres_dev clean_volumes_dev

up_dev:
	@cd $(DEVOPS_DIR) && docker-compose up -d

down_dev:
	@cd $(DEVOPS_DIR) && docker-compose down

logs_dev:
	@cd $(DEVOPS_DIR) && docker-compose logs

clean_postgres_dev:
	@cd $(DEVOPS_DIR)/postgres && sudo rm -r pgdata/

clean_volumes_dev:
	@cd $(DEVOPS_DIR)/postgres && docker system prune && \
	docker volume rm $(docker volume ls -q)
