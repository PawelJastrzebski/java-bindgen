package j.bindgen.api;

import io.quarkus.security.UnauthorizedException;
import j.bindgen.api.dto.UserDto;
import j.bindgen.security.AuthUser;
import jakarta.annotation.security.RolesAllowed;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import org.jboss.logging.Logger;

@Path("/user")
public class UserController {

    Logger logger = Logger.getLogger(UserController.class);

    @Inject
    AuthUser user;

    @GET
    @RolesAllowed("user")
    public UserDto should_get_user() {
        return user.getUser().orElseThrow(() -> new UnauthorizedException("Unauthorized"));
    }

}
