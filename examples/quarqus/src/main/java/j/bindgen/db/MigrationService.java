package j.bindgen.db;

import io.quarkus.runtime.Startup;
import jakarta.annotation.PostConstruct;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import org.flywaydb.core.Flyway;
import org.jboss.logging.Logger;

@Startup
@ApplicationScoped
public class MigrationService {

    Logger logger = Logger.getLogger(MigrationService.class);

    @Inject
    Flyway flyway;

    @PostConstruct
    public void checkMigration() {
        var version = flyway.info().current().getVersion().toString();
        logger.info("Db version: " + version);
    }
}
