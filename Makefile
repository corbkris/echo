DEVOPS_DIR = devops/develop

.PHONY: up down clean

up_dev:
	@cd $(DEVOPS_DIR) && docker-compose up -d

down_dev:
	@cd $(DEVOPS_DIR) && docker-compose down

clean_dev:
	@cd $(DEVOPS_DIR)/postgres && sudo rm -r pgdata/ && \
	docker system prune && \
	docker volume rm $(docker volume ls -q)

